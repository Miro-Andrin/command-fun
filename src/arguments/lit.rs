use crate::{Argument, CommandError};

pub struct Literal<const VALUE: &'static str>;

impl<const VALUE:&'static str, Ctx> Argument<Ctx> for Literal<VALUE>{
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

    fn tab_complete(context: Ctx, input: &str) -> Result<Vec<String>, crate::TabCompleteError> {
        
        // Figure out how much overlap there is.
        let overlap = match input.chars().zip(VALUE.chars()).position(|(a,b)| a != b) {
            Some(x) => x,
            None => input.len().min(VALUE.len()),
        };

        if overlap == input.len() {
            // Sugest rest of VALUE
            return Ok(vec![VALUE[overlap..VALUE.len()].to_owned()])
        }

        
        
        
        todo!()
    }
}


