use crate::{Argument, CommandError};

pub struct Literal<const VALUE: &'static str>;



impl<const VALUE:&'static str > Argument for Literal<VALUE>{
    fn parse<'a>(input: &'a str) -> Result<(Self, &'a str), crate::CommandError> {
        if input.starts_with(VALUE){
            Ok((Literal, &input[VALUE.len()..]))
        } else {
             // Todo: this could be more precises, to account for the parts that did overlap.
            Err(
                CommandError::Err {
                    msg: format!("Expected {}.",VALUE),
                    rest: input.len(),
                }
            )
        }
    }
}