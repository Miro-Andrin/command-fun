use crate::{Command, CommandError, CommandsResult};
use std::collections::HashMap;

#[derive(Debug)]
enum CommandAddError {
    StartContainsSpace,
    StartWasEmpty,
}

pub struct CommandMgr<Ctx> {
    mapping: HashMap<String, Vec<Command<Ctx>>>,
}

impl<Ctx> CommandMgr<Ctx> {
    fn add<F>(&mut self, start: &str, command: F) -> Result<(), CommandAddError>
    where
        F: Fn(&mut Ctx, &str) -> CommandsResult<i64> + 'static,
    {
        if start.is_empty() {
            return Err(CommandAddError::StartWasEmpty);
        }

        if start.contains(|c: char| c.is_whitespace()) {
            return Err(CommandAddError::StartContainsSpace);
        }

        let c = Command::new(start, command);

        match self.mapping.get_mut(&start.to_owned()) {
            Some(list) => {
                list.push(c);
            }
            None => {
                self.mapping.insert(start.to_owned(), vec![c]);
            }
        };
        
        Ok(())
    }

    fn call<S>(&self, msg: S, context: &mut Ctx) -> CommandsResult<i64>
    where
        S: AsRef<str>,
    {
        // Figure out the start of the command,
        let msg = msg.as_ref();

        if let Some(whitespace_pos) = msg.chars().position(|c| c.is_whitespace()) {
            let (start, mut rest) = msg.split_at(whitespace_pos);
            rest = rest.trim_start();

            if let Some(commands) = self.mapping.get(start) {
                for command in commands {
                    match command.call(context, rest) {
                        Ok(x) => return Ok(x),
                        // TODO_ what should happen here?  What Error should be triggerd? The last
                        // one like we have it now?
                        Err(e) => return Err(e),
                    }
                }
            }
        }

        Err(CommandError::CommandNotFound)
    }
}
