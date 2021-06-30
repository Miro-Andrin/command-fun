use std::collections::HashMap;

mod arguments;
mod command_manager;

pub enum CommandError {
    CommandNotFound,
    ExpectedChar(char),
    Err(String),
}

pub enum CommandCaller {
    Player,
    Terminal,
    CommandBlock,
}

type CommandsResult<X> = Result<X, CommandError>;

pub struct Command<Ctx> {
    // What the command starts with, should be something like /msg, /echo, etc..
    // Notice how / is included.
    start: String,
    exec: Box<dyn Fn(&mut Ctx, &str) -> CommandsResult<i64>>,
}

impl<Ctx> Command<Ctx> {
    fn new<S, F>(start: S, func: F) -> Self
    where
        S: AsRef<str>,
        F: Fn(&mut Ctx, &str) -> CommandsResult<i64> + 'static,
    {
        Command {
            start: start.as_ref().to_owned(),
            exec: Box::new(func),
        }
    }

    fn call(&self, ctx: &mut Ctx, input: &str) -> CommandsResult<i64> {
        (self.exec)(ctx, input)
    }
}

pub trait Argumet: Sized {
    fn parse<'a>(input: &'a str) -> Option<(Self, &'a str)>;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
