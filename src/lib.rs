
#![feature(const_generics)]


mod arguments;
mod command_manager;

#[derive(Debug)]
pub enum CommandError {
    CommandNotFound,
    // ExpectedChar(char),
    // Err(String),
    Err { msg: String, rest: usize },
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

pub trait Argument: Sized {
    fn parse<'a>(input: &'a str) -> Result<(Self, &'a str), CommandError>;
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::arguments::Space;
    use crate::arguments::Literal;
    
    #[test]
    fn it_works() {
        let command = Command::new("/add", |ctx: &mut usize, input: &str| {
            let (args, rest) : ((Literal::<"/add">,i64, i64), &str) = Argument::parse(input)?;
            Ok(args.1  + args.2 + ctx.clone() as i64)
        });


        let res = command.call(&mut 10, "/add 10 10");

        assert_eq!(30, res.unwrap())
    }
}
