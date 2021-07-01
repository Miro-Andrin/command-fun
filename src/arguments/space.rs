use crate::{Argument, CommandError};

/// One or more space
pub struct Space;
impl Argument for Space {
    fn parse<'a>(input: &'a str) -> Result<(Self, &'a str), CommandError> {
        if !input.starts_with(" ") {
            return Err(CommandError::Err{
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
impl Argument for Spaces {
    fn parse<'a>(input: &'a str) -> Result<(Self, &'a str), CommandError> {
        Ok((Spaces, input.trim_start()))
    }
}