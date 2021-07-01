use crate::{Argument, CommandError};

/// One or more space
pub struct Space;
impl<Ctx> Argument<Ctx> for Space {
    fn parse<'a>(input: &'a str) -> Result<(Self, &'a str), CommandError> {
        if !input.starts_with(" ") {
            return Err(CommandError::Err {
                msg: "Expected at least one space.".to_owned(),
                rest: input.len(),
            });
        } else {
            Ok((Space, input.trim_start()))
        }
    }
}

/// Zero or more space
pub struct Spaces;
impl<Ctx> Argument<Ctx> for Spaces {
    fn parse<'a>(input: &'a str) -> Result<(Self, &'a str), CommandError> {
        Ok((Spaces, input.trim_start()))
    }
}



/// The rest of the input is empty, containing only whitespace.
pub struct Empty;
impl<Ctx> Argument<Ctx> for Empty {
    fn parse<'a>(input: &'a str) -> Result<(Self, &'a str), CommandError> {

        let rest = input.trim_start();
        if rest.is_empty() {
            Ok((Empty, rest))
        } else {
            Err(CommandError::Err{
                msg: format!("Expected the rest of the input to be empty, but found: {}.", rest),
                rest: rest.len(),
            })
        }
    }
}
