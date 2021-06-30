use crate::Argumet;

/// One or more space
pub struct Space;
impl Argumet for Space {
    fn parse<'a>(input: &'a str) -> Result<(Self, &'a str), ()> {
        if !input.starts_with(" ") {
            return Err(());
        } else {
            Ok((Space, input.trim_start()))
        }
    }
}