use core::num;

use crate::Argumet;

fn split_off_number(input: &str) -> (&str, &str, &str) {
    let (sign, rest) = if input.starts_with('+') || input.starts_with('-') {
        (&input[0..1], &input[1..])
    } else {
        (&input[0..0], input)
    };

    let non_number = rest.trim_start_matches(|c: char| c.is_numeric());

    let (number, rest) = rest.split_at(rest.len() - non_number.len());

    (sign, number, rest)
}


macro_rules! impl_arg_positive_number {
    ($num:ident) => {
        
        impl crate::Argumet for $num {
            fn parse<'a>(input: &'a str) -> Result<(Self, &'a str),()> {
                
                let (sign, number, rest) = split_off_number(input);

                if sign == "-" {
                    // Then it cant be a possitive number
                    return Err(());
                }

                if number.is_empty() {
                    return Err(());
                }

                let res = match number.parse() {
                    Ok(x) => x,
                    Err(_) => return Err(()),
                };

                Ok((res,rest))

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
        impl crate::Argumet for $num {
            fn parse<'a>(input: &'a str) -> Result<(Self, &'a str),()> {
                
                let (sign, number, rest) = split_off_number(input);
        
                if number.is_empty() {
                    return Err(());
                }
                
                let res = match (&input[0..sign.len()+number.len()]).parse() {
                    Ok(x) => x,
                    Err(_) => return Err(()),
                };
        
                Ok((res,rest))
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
    let rest  = rest.trim_start_matches(|c: char| c.is_numeric());
    
    // Trim off Dot if present
    let rest = if rest.starts_with(".") {
        &rest[".".len()..]
    } else {
        rest
    };
    // Trim off exponent
    let rest  = rest.trim_start_matches(|c: char| c.is_numeric());
    let number = &input[0..input.len() - rest.len()];
    (number, rest)
}


macro_rules! impl_arg_float {
    ($num:ident) => {
        impl crate::Argumet for $num {
            fn parse<'a>(input: &'a str) -> Result<(Self, &'a str),()> {
                
                let (number, rest) = split_off_float(input);

                let res = match number.parse() {
                    Ok(x) => x,
                    Err(_) => return Err(()),
                };

                Ok((res,rest))
            }
        }
    };
}

impl_arg_float!(f32);
impl_arg_float!(f64);