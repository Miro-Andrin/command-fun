
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

#[derive(Debug)]
pub enum TabCompleteError {
    Err {
        rest: usize,
    }
}


type CommandsResult<X> = Result<X, CommandError>;

pub struct Command<Ctx> {
    // What the command starts with, should be something like /msg, /echo, etc..
    // Notice how / is included.
    start: String,
    exec: Box<dyn Fn(&mut Ctx, &str) -> CommandsResult<i64>>,
}

impl<Ctx> Command<Ctx> {
    pub fn new<F>(start: &str, func: F) -> Self
    where
        F: Fn(&mut Ctx, &str) -> CommandsResult<i64> + 'static,
    {
        Command {
            start: start.to_owned(),
            exec: Box::new(func),
        }
    }

    pub fn call(&self, ctx: &mut Ctx, input: &str) -> CommandsResult<i64> {
        (self.exec)(ctx, input)
    }
}

pub trait Argument<Ctx>: Sized {
    fn parse<'a>(input: &'a str) -> Result<(Self, &'a str), CommandError>;

    fn tab_complete(ctx: Ctx,input: &str) -> Result<Vec<String>, TabCompleteError>;
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::arguments::Literal;
    use crate::arguments::Empty;
    
    #[test]
    fn it_works() {
        let command = Command::new("/add", |ctx: &mut usize, input: &str| {
            let (args, rest) : ((Literal::<"/add">,i64, i64), &str) = Argument::<usize>::parse(input)?;
            // The rest of the string must be empty
            // The reason empty is not part of the tuple is because parsing tuples assumes that there is at least one
            // space between every argument. Making it so that the command requires a trailing whitepsace.  
            let (_empty, _rest) : (Empty, &str) = Argument::<usize>::parse(rest)?;

            Ok(args.1  + args.2 + ctx.clone() as i64)
        });


        let res = command.call(&mut 10, "/add 10 10");

        assert_eq!(30, res.unwrap())
    }
}


