use crate::{Argument, CommandError};

use super::{Space, Spaces};

impl<A> Argument for (A,)
where
    A: Argument,
{
    fn parse<'a>(input: &'a str) -> std::result::Result<(Self, &'a str), CommandError> {
        let (a, input) = A::parse(input)?;
        Ok(((a,), input))
    }
}

impl<A, B> Argument for (A, B)
where
    A: Argument,
    B: Argument,
{
    fn parse<'a>(input: &'a str) -> std::result::Result<(Self, &'a str), CommandError> {
        let (a, input) = A::parse(input)?;
        let (_, input) = Space::parse(input)?;
        let (b, input) = B::parse(input)?;
        
        Ok(((a, b), input))
    }
}


impl<A, B, C> Argument for (A, B, C)
where
    A: Argument,
    B: Argument,
    C: Argument,
{
    fn parse<'a>(input: &'a str) -> std::result::Result<(Self, &'a str), CommandError> {
        let (a, input) = A::parse(input)?;
        let (_, input) = Space::parse(input)?;
        let (b, input) = B::parse(input)?;
        let (_, input) = Space::parse(input)?;
        let (c, input) = C::parse(input)?;
        
        Ok(((a, b, c), input))
    }
}

impl<A, B, C, D> Argument for (A, B, C, D)
where
    A: Argument,
    B: Argument,
    C: Argument,
    D: Argument,
{
    fn parse<'a>(input: &'a str) -> std::result::Result<(Self, &'a str), CommandError> {
        let (a, input) = A::parse(input)?;
        let (_, input) = Space::parse(input)?;
        let (b, input) = B::parse(input)?;
        let (_, input) = Space::parse(input)?;
        let (c, input) = C::parse(input)?;
        let (_, input) = Space::parse(input)?;
        let (d, input) = D::parse(input)?;
        
        Ok(((a, b, c, d), input))
    }
}

impl<A, B, C, D, E> Argument for (A, B, C, D, E)
where
    A: Argument,
    B: Argument,
    C: Argument,
    D: Argument,
    E: Argument,
{
    fn parse<'a>(input: &'a str) -> std::result::Result<(Self, &'a str), CommandError> {
        let (a, input) = A::parse(input)?;
        let (_, input) = Space::parse(input)?;
        let (b, input) = B::parse(input)?;
        let (_, input) = Space::parse(input)?;
        let (c, input) = C::parse(input)?;
        let (_, input) = Space::parse(input)?;
        let (d, input) = D::parse(input)?;
        let (_, input) = Space::parse(input)?;
        let (e, input) = E::parse(input)?;
        
        Ok(((a, b, c, d, e), input))
    }
}

impl<A, B, C, D, E, F> Argument for (A, B, C, D, E, F)
where
    A: Argument,
    B: Argument,
    C: Argument,
    D: Argument,
    E: Argument,
    F: Argument,
{
    fn parse<'a>(input: &'a str) -> std::result::Result<(Self, &'a str), CommandError> {
        let (a, input) = A::parse(input)?;
        let (_, input) = Space::parse(input)?;
        let (b, input) = B::parse(input)?;
        let (_, input) = Space::parse(input)?;
        let (c, input) = C::parse(input)?;
        let (_, input) = Space::parse(input)?;
        let (d, input) = D::parse(input)?;
        let (_, input) = Space::parse(input)?;
        let (e, input) = E::parse(input)?;
        let (_, input) = Space::parse(input)?;
        let (f, input) = F::parse(input)?;
        
        Ok(((a, b, c, d, e, f), input))
    }
}

impl<A, B, C, D, E, F, G> Argument for (A, B, C, D, E, F, G)
where
    A: Argument,
    B: Argument,
    C: Argument,
    D: Argument,
    E: Argument,
    F: Argument,
    G: Argument,
{
    fn parse<'a>(input: &'a str) -> std::result::Result<(Self, &'a str), CommandError> {
        let (a, input) = A::parse(input)?;
        let (_, input) = Space::parse(input)?;
        let (b, input) = B::parse(input)?;
        let (_, input) = Space::parse(input)?;
        let (c, input) = C::parse(input)?;
        let (_, input) = Space::parse(input)?;
        let (d, input) = D::parse(input)?;
        let (_, input) = Space::parse(input)?;
        let (e, input) = E::parse(input)?;
        let (_, input) = Space::parse(input)?;
        let (f, input) = F::parse(input)?;
        let (_, input) = Space::parse(input)?;
        let (g, input) = G::parse(input)?;
        
        Ok(((a, b, c, d, e, f, g), input))
    }
}

impl<A, B, C, D, E, F, G, H> Argument for (A, B, C, D, E, F, G, H)
where
    A: Argument,
    B: Argument,
    C: Argument,
    D: Argument,
    E: Argument,
    F: Argument,
    G: Argument,
    H: Argument,
{
    fn parse<'a>(input: &'a str) -> std::result::Result<(Self, &'a str), CommandError> {
        let (a, input) = A::parse(input)?;
        let (_, input) = Space::parse(input)?;
        let (b, input) = B::parse(input)?;
        let (_, input) = Space::parse(input)?;
        let (c, input) = C::parse(input)?;
        let (_, input) = Space::parse(input)?;
        let (d, input) = D::parse(input)?;
        let (_, input) = Space::parse(input)?;
        let (e, input) = E::parse(input)?;
        let (_, input) = Space::parse(input)?;
        let (f, input) = F::parse(input)?;
        let (_, input) = Space::parse(input)?;
        let (g, input) = G::parse(input)?;
        let (_, input) = Space::parse(input)?;
        let (h, input) = H::parse(input)?;
        
        Ok(((a, b, c, d, e, f, g, h), input))
    }
}

impl<A, B, C, D, E, F, G, H,I> Argument for (A, B, C, D, E, F, G, H,I)
where
    A: Argument,
    B: Argument,
    C: Argument,
    D: Argument,
    E: Argument,
    F: Argument,
    G: Argument,
    H: Argument,
    I: Argument,
{
    fn parse<'a>(input: &'a str) -> std::result::Result<(Self, &'a str), CommandError> {
        let (a, input) = A::parse(input)?;
        let (_, input) = Space::parse(input)?;
        let (b, input) = B::parse(input)?;
        let (_, input) = Space::parse(input)?;
        let (c, input) = C::parse(input)?;
        let (_, input) = Space::parse(input)?;
        let (d, input) = D::parse(input)?;
        let (_, input) = Space::parse(input)?;
        let (e, input) = E::parse(input)?;
        let (_, input) = Space::parse(input)?;
        let (f, input) = F::parse(input)?;
        let (_, input) = Space::parse(input)?;
        let (g, input) = G::parse(input)?;
        let (_, input) = Space::parse(input)?;
        let (h, input) = H::parse(input)?;
        let (_, input) = Space::parse(input)?;
        let (i, input) = I::parse(input)?;
        
        Ok(((a, b, c, d, e, f, g, h,i), input))
    }
}
