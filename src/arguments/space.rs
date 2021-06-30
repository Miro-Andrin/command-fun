use crate::Argumet;

/// One or more space
pub struct Space;
impl Argumet for Space {
    fn parse<'a>(input: &'a str) -> Option<(Self, &'a str)> {
        if !input.starts_with(" ") {
            return None;
        } else {
            Some((Space, input.trim_start()))
        }
    }
}