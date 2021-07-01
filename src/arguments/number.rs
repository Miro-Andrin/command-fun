// fn split_off_number(input: &str) -> (&str, &str, &str) {
//     let (sign, rest) = if input.starts_with('+') || input.starts_with('-') {
//         (&input[0..1], &input[1..])
//     } else {
//         (&input[0..0], input)
//     };

//     let non_number = rest.trim_start_matches(|c: char| c.is_numeric());
//     let (number, rest) = rest.split_at(rest.len() - non_number.len());
//     (sign, number, rest)
// }

fn splitt_off(input: &str) -> (&str, &str) {
    let pos = input.chars().position(|c| c.is_whitespace());
    match pos {
        Some(pos) => input.split_at(pos),
        None => (input, &input[input.len()..input.len()]),
    }
}

macro_rules! impl_arg_positive_number {
    ($num:ident) => {
        impl<Ctx> crate::Argument<Ctx> for $num {
            fn parse<'a>(input: &'a str) -> Result<(Self, &'a str), crate::CommandError> {
                let (number, rest) = splitt_off(input);

                if number.starts_with("-") {
                    // Then it cant be a possitive number
                    return Err(crate::CommandError::Err {
                        msg: format!(
                            "Unable to parse {} as a possitive number: {}",
                            number,
                            stringify!($num)
                        ),

                        rest: input.len(),
                    });
                }

                let res = match number.parse() {
                    Ok(x) => x,
                    Err(_) => {
                        return Err(crate::CommandError::Err {
                            msg: format!(
                                "Unable to parse '{}' as a {} number.",
                                number,
                                stringify!($num)
                            ),
                            rest: input.len(),
                        })
                    }
                };

                Ok((res, rest))
            }
        }
    };
}

impl_arg_positive_number!(u8);
impl_arg_positive_number!(u16);
impl_arg_positive_number!(u32);
impl_arg_positive_number!(u64);
impl_arg_positive_number!(usize);

macro_rules! impl_arg_number {
    ($num:ident) => {
        impl<Ctx> crate::Argument<Ctx> for $num {
            fn parse<'a>(input: &'a str) -> Result<(Self, &'a str), crate::CommandError> {
                let (number, rest) = splitt_off(input);

                let res = match number.parse() {
                    Ok(x) => x,
                    Err(_) => {
                        return Err(crate::CommandError::Err {
                            msg: format!(
                                "Unable to parse '{}' as a {} number.",
                                number,
                                stringify!($num)
                            ),
                            rest: input.len(),
                        })
                    }
                };

                Ok((res, rest))
            }
        }
    };
}

impl_arg_number!(i8);
impl_arg_number!(i16);
impl_arg_number!(i32);
impl_arg_number!(i64);
impl_arg_number!(isize);

// input -> (sign, mantissa, dot, exponent, rest)
fn split_off_float(input: &str) -> (&str, &str) {
    // Trim off sign
    let rest = input.trim_start_matches(|c| c == '+' || c == '-');

    // Trim off mantissa
    let rest = rest.trim_start_matches(|c: char| c.is_numeric());

    // Trim off Dot if present
    let rest = if rest.starts_with(".") {
        &rest[".".len()..]
    } else {
        rest
    };
    // Trim off exponent
    let rest = rest.trim_start_matches(|c: char| c.is_numeric());
    let number = &input[0..input.len() - rest.len()];
    (number, rest)
}

macro_rules! impl_arg_float {
    ($num:ident) => {
        impl<Ctx> crate::Argument<Ctx> for $num {
            fn parse<'a>(input: &'a str) -> Result<(Self, &'a str), crate::CommandError> {
                let (number, rest) = split_off_float(input);

                let res = match number.parse() {
                    Ok(x) => x,
                    Err(_) => {
                        return Err(crate::CommandError::Err {
                            msg: format!("Unable to parse {} as a floating point number.", number),
                            rest: input.len(),
                        })
                    }
                };

                Ok((res, rest))
            }
        }
    };
}

impl_arg_float!(f32);
impl_arg_float!(f64);


#[cfg(test)]
mod test {
    use crate::Argument;

    #[test]
    fn test_numbers() {
        let input = "10";
        let (num, rest) : (u8,&str) = Argument::<()>::parse(input).unwrap(); 
        assert_eq!(num, 10);
        assert_eq!(rest,"");


        let input = "10 ";
        let (num, rest) : (u8,&str) = Argument::<()>::parse(input).unwrap(); 
        assert_eq!(num, 10);
        assert_eq!(rest," ");

        let input = ".1 ";
        let (num, rest) : (f32,&str) = Argument::<()>::parse(input).unwrap(); 
        assert_eq!(num, 0.1);
        assert_eq!(rest," ");

    }
}