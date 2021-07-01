use crate::{Argument, CommandError};

use super::Space;

impl<Ctx, A> Argument<Ctx> for (A,)
where
    A: Argument<Ctx>,
{
    fn parse<'a>(input: &'a str) -> std::result::Result<(Self, &'a str), CommandError> {
        let (a, input) = A::parse(input)?;
        Ok(((a,), input))
    }

    fn tab_complete(ctx: Ctx, input: &str) -> Result<Vec<String>, crate::TabCompleteError> {
        A::tab_complete(ctx, input)
    }
}

impl<Ctx, A, B> Argument<Ctx> for (A, B)
where
    A: Argument<Ctx>,
    B: Argument<Ctx>,
{
    fn parse<'a>(input: &'a str) -> std::result::Result<(Self, &'a str), CommandError> {
        let (a, input) = A::parse(input)?;
        let (_, input) = <Space as Argument<Ctx>>::parse(input)?;
        let (b, input) = B::parse(input)?;

        Ok(((a, b), input))
    }

    fn tab_complete(ctx: Ctx, input: &str) -> Result<Vec<String>, crate::TabCompleteError> {
        if let Ok((_, input)) = A::parse(input) {
            if let Ok((_, _input)) = B::parse(input) {
                Ok(vec![])
            } else {
                B::tab_complete(ctx, input)
            }
        } else {
            A::tab_complete(ctx, input)
        }
    }
}

impl<Ctx, A, B, C> Argument<Ctx> for (A, B, C)
where
    A: Argument<Ctx>,
    B: Argument<Ctx>,
    C: Argument<Ctx>,
{
    fn parse<'a>(input: &'a str) -> std::result::Result<(Self, &'a str), CommandError> {
        let (a, input) = A::parse(input)?;
        let (_, input) = <Space as Argument<Ctx>>::parse(input)?;
        let (b, input) = B::parse(input)?;
        let (_, input) = <Space as Argument<Ctx>>::parse(input)?;
        let (c, input) = C::parse(input)?;

        Ok(((a, b, c), input))
    }

    fn tab_complete(ctx: Ctx, input: &str) -> Result<Vec<String>, crate::TabCompleteError> {
        if let Ok((_, input)) = A::parse(input) {
            if let Ok((_, input)) = B::parse(input) {
                if let Ok((_, _input)) = C::parse(input) {
                    Ok(vec![])
                } else {
                    C::tab_complete(ctx, input)
                }
            } else {
                B::tab_complete(ctx, input)
            }
        } else {
            A::tab_complete(ctx, input)
        }
    }
}

impl<Ctx, A, B, C, D> Argument<Ctx> for (A, B, C, D)
where
    A: Argument<Ctx>,
    B: Argument<Ctx>,
    C: Argument<Ctx>,
    D: Argument<Ctx>,
{
    fn parse<'a>(input: &'a str) -> std::result::Result<(Self, &'a str), CommandError> {
        let (a, input) = A::parse(input)?;
        let (_, input) = <Space as Argument<Ctx>>::parse(input)?;
        let (b, input) = B::parse(input)?;
        let (_, input) = <Space as Argument<Ctx>>::parse(input)?;
        let (c, input) = C::parse(input)?;
        let (_, input) = <Space as Argument<Ctx>>::parse(input)?;
        let (d, input) = D::parse(input)?;

        Ok(((a, b, c, d), input))
    }

    fn tab_complete(ctx: Ctx, input: &str) -> Result<Vec<String>, crate::TabCompleteError> {
        if let Ok((_, input)) = A::parse(input) {
            if let Ok((_, input)) = B::parse(input) {
                if let Ok((_, input)) = C::parse(input) {
                    if let Ok((_, _input)) = D::parse(input) {
                        Ok(vec![])
                    } else {
                        D::tab_complete(ctx, input)
                    }
                } else {
                    C::tab_complete(ctx, input)
                }
            } else {
                B::tab_complete(ctx, input)
            }
        } else {
            A::tab_complete(ctx, input)
        }
    }
}

impl<Ctx, A, B, C, D, E> Argument<Ctx> for (A, B, C, D, E)
where
    A: Argument<Ctx>,
    B: Argument<Ctx>,
    C: Argument<Ctx>,
    D: Argument<Ctx>,
    E: Argument<Ctx>,
{
    fn parse<'a>(input: &'a str) -> std::result::Result<(Self, &'a str), CommandError> {
        let (a, input) = A::parse(input)?;
        let (_, input) = <Space as Argument<Ctx>>::parse(input)?;
        let (b, input) = B::parse(input)?;
        let (_, input) = <Space as Argument<Ctx>>::parse(input)?;
        let (c, input) = C::parse(input)?;
        let (_, input) = <Space as Argument<Ctx>>::parse(input)?;
        let (d, input) = D::parse(input)?;
        let (_, input) = <Space as Argument<Ctx>>::parse(input)?;
        let (e, input) = E::parse(input)?;

        Ok(((a, b, c, d, e), input))
    }

    fn tab_complete(ctx: Ctx, input: &str) -> Result<Vec<String>, crate::TabCompleteError> {
        if let Ok((_, input)) = A::parse(input) {
            if let Ok((_, input)) = B::parse(input) {
                if let Ok((_, input)) = C::parse(input) {
                    if let Ok((_, input)) = D::parse(input) {
                        if let Ok((_, _input)) = E::parse(input) {
                            Ok(vec![])
                        } else {
                            E::tab_complete(ctx, input)
                        }
                    } else {
                        D::tab_complete(ctx, input)
                    }
                } else {
                    C::tab_complete(ctx, input)
                }
            } else {
                B::tab_complete(ctx, input)
            }
        } else {
            A::tab_complete(ctx, input)
        }
    }
}

impl<Ctx, A, B, C, D, E, F> Argument<Ctx> for (A, B, C, D, E, F)
where
    A: Argument<Ctx>,
    B: Argument<Ctx>,
    C: Argument<Ctx>,
    D: Argument<Ctx>,
    E: Argument<Ctx>,
    F: Argument<Ctx>,
{
    fn parse<'a>(input: &'a str) -> std::result::Result<(Self, &'a str), CommandError> {
        let (a, input) = A::parse(input)?;
        let (_, input) = <Space as Argument<Ctx>>::parse(input)?;
        let (b, input) = B::parse(input)?;
        let (_, input) = <Space as Argument<Ctx>>::parse(input)?;
        let (c, input) = C::parse(input)?;
        let (_, input) = <Space as Argument<Ctx>>::parse(input)?;
        let (d, input) = D::parse(input)?;
        let (_, input) = <Space as Argument<Ctx>>::parse(input)?;
        let (e, input) = E::parse(input)?;
        let (_, input) = <Space as Argument<Ctx>>::parse(input)?;
        let (f, input) = F::parse(input)?;

        Ok(((a, b, c, d, e, f), input))
    }

    fn tab_complete(ctx: Ctx, input: &str) -> Result<Vec<String>, crate::TabCompleteError> {
        if let Ok((_, input)) = A::parse(input) {
            if let Ok((_, input)) = B::parse(input) {
                if let Ok((_, input)) = C::parse(input) {
                    if let Ok((_, input)) = D::parse(input) {
                        if let Ok((_, input)) = E::parse(input) {
                            if let Ok((_, _input)) = F::parse(input) {
                                Ok(vec![])
                            } else {
                                F::tab_complete(ctx, input)
                            }
                        } else {
                            E::tab_complete(ctx, input)
                        }
                    } else {
                        D::tab_complete(ctx, input)
                    }
                } else {
                    C::tab_complete(ctx, input)
                }
            } else {
                B::tab_complete(ctx, input)
            }
        } else {
            A::tab_complete(ctx, input)
        }
    }
}

impl<Ctx, A, B, C, D, E, F, G> Argument<Ctx> for (A, B, C, D, E, F, G)
where
    A: Argument<Ctx>,
    B: Argument<Ctx>,
    C: Argument<Ctx>,
    D: Argument<Ctx>,
    E: Argument<Ctx>,
    F: Argument<Ctx>,
    G: Argument<Ctx>,
{
    fn parse<'a>(input: &'a str) -> std::result::Result<(Self, &'a str), CommandError> {
        let (a, input) = A::parse(input)?;
        let (_, input) = <Space as Argument<Ctx>>::parse(input)?;
        let (b, input) = B::parse(input)?;
        let (_, input) = <Space as Argument<Ctx>>::parse(input)?;
        let (c, input) = C::parse(input)?;
        let (_, input) = <Space as Argument<Ctx>>::parse(input)?;
        let (d, input) = D::parse(input)?;
        let (_, input) = <Space as Argument<Ctx>>::parse(input)?;
        let (e, input) = E::parse(input)?;
        let (_, input) = <Space as Argument<Ctx>>::parse(input)?;
        let (f, input) = F::parse(input)?;
        let (_, input) = <Space as Argument<Ctx>>::parse(input)?;
        let (g, input) = G::parse(input)?;

        Ok(((a, b, c, d, e, f, g), input))
    }

    fn tab_complete(ctx: Ctx, input: &str) -> Result<Vec<String>, crate::TabCompleteError> {
        if let Ok((_, input)) = A::parse(input) {
            if let Ok((_, input)) = B::parse(input) {
                if let Ok((_, input)) = C::parse(input) {
                    if let Ok((_, input)) = D::parse(input) {
                        if let Ok((_, input)) = E::parse(input) {
                            if let Ok((_, input)) = F::parse(input) {
                                if let Ok((_, _input)) = G::parse(input) {
                                    Ok(vec![])
                                } else {
                                    G::tab_complete(ctx, input)
                                }
                            } else {
                                F::tab_complete(ctx, input)
                            }
                        } else {
                            E::tab_complete(ctx, input)
                        }
                    } else {
                        D::tab_complete(ctx, input)
                    }
                } else {
                    C::tab_complete(ctx, input)
                }
            } else {
                B::tab_complete(ctx, input)
            }
        } else {
            A::tab_complete(ctx, input)
        }
    }
}

impl<Ctx, A, B, C, D, E, F, G, H> Argument<Ctx> for (A, B, C, D, E, F, G, H)
where
    A: Argument<Ctx>,
    B: Argument<Ctx>,
    C: Argument<Ctx>,
    D: Argument<Ctx>,
    E: Argument<Ctx>,
    F: Argument<Ctx>,
    G: Argument<Ctx>,
    H: Argument<Ctx>,
{
    fn parse<'a>(input: &'a str) -> std::result::Result<(Self, &'a str), CommandError> {
        let (a, input) = A::parse(input)?;
        let (_, input) = <Space as Argument<Ctx>>::parse(input)?;
        let (b, input) = B::parse(input)?;
        let (_, input) = <Space as Argument<Ctx>>::parse(input)?;
        let (c, input) = C::parse(input)?;
        let (_, input) = <Space as Argument<Ctx>>::parse(input)?;
        let (d, input) = D::parse(input)?;
        let (_, input) = <Space as Argument<Ctx>>::parse(input)?;
        let (e, input) = E::parse(input)?;
        let (_, input) = <Space as Argument<Ctx>>::parse(input)?;
        let (f, input) = F::parse(input)?;
        let (_, input) = <Space as Argument<Ctx>>::parse(input)?;
        let (g, input) = G::parse(input)?;
        let (_, input) = <Space as Argument<Ctx>>::parse(input)?;
        let (h, input) = H::parse(input)?;

        Ok(((a, b, c, d, e, f, g, h), input))
    }

    fn tab_complete(ctx: Ctx, input: &str) -> Result<Vec<String>, crate::TabCompleteError> {
        if let Ok((_, input)) = A::parse(input) {
            if let Ok((_, input)) = B::parse(input) {
                if let Ok((_, input)) = C::parse(input) {
                    if let Ok((_, input)) = D::parse(input) {
                        if let Ok((_, input)) = E::parse(input) {
                            if let Ok((_, input)) = F::parse(input) {
                                if let Ok((_, input)) = G::parse(input) {
                                    if let Ok((_, _input)) = H::parse(input) {
                                        Ok(vec![])
                                    } else {
                                        H::tab_complete(ctx, input)
                                    }
                                } else {
                                    G::tab_complete(ctx, input)
                                }
                            } else {
                                F::tab_complete(ctx, input)
                            }
                        } else {
                            E::tab_complete(ctx, input)
                        }
                    } else {
                        D::tab_complete(ctx, input)
                    }
                } else {
                    C::tab_complete(ctx, input)
                }
            } else {
                B::tab_complete(ctx, input)
            }
        } else {
            A::tab_complete(ctx, input)
        }
    }
}

impl<Ctx, A, B, C, D, E, F, G, H, I> Argument<Ctx> for (A, B, C, D, E, F, G, H, I)
where
    A: Argument<Ctx>,
    B: Argument<Ctx>,
    C: Argument<Ctx>,
    D: Argument<Ctx>,
    E: Argument<Ctx>,
    F: Argument<Ctx>,
    G: Argument<Ctx>,
    H: Argument<Ctx>,
    I: Argument<Ctx>,
{
    fn parse<'a>(input: &'a str) -> std::result::Result<(Self, &'a str), CommandError> {
        let (a, input) = A::parse(input)?;
        let (_, input) = <Space as Argument<Ctx>>::parse(input)?;
        let (b, input) = B::parse(input)?;
        let (_, input) = <Space as Argument<Ctx>>::parse(input)?;
        let (c, input) = C::parse(input)?;
        let (_, input) = <Space as Argument<Ctx>>::parse(input)?;
        let (d, input) = D::parse(input)?;
        let (_, input) = <Space as Argument<Ctx>>::parse(input)?;
        let (e, input) = E::parse(input)?;
        let (_, input) = <Space as Argument<Ctx>>::parse(input)?;
        let (f, input) = F::parse(input)?;
        let (_, input) = <Space as Argument<Ctx>>::parse(input)?;
        let (g, input) = G::parse(input)?;
        let (_, input) = <Space as Argument<Ctx>>::parse(input)?;
        let (h, input) = H::parse(input)?;
        let (_, input) = <Space as Argument<Ctx>>::parse(input)?;
        let (i, input) = I::parse(input)?;

        Ok(((a, b, c, d, e, f, g, h, i), input))
    }

    fn tab_complete(ctx: Ctx, input: &str) -> Result<Vec<String>, crate::TabCompleteError> {
        if let Ok((_, input)) = A::parse(input) {
            if let Ok((_, input)) = B::parse(input) {
                if let Ok((_, input)) = C::parse(input) {
                    if let Ok((_, input)) = D::parse(input) {
                        if let Ok((_, input)) = E::parse(input) {
                            if let Ok((_, input)) = F::parse(input) {
                                if let Ok((_, input)) = G::parse(input) {
                                    if let Ok((_, input)) = H::parse(input) {
                                        if let Ok((_, _input)) = I::parse(input) {
                                            Ok(vec![])
                                        } else {
                                            I::tab_complete(ctx, input)
                                        }
                                    } else {
                                        H::tab_complete(ctx, input)
                                    }
                                } else {
                                    G::tab_complete(ctx, input)
                                }
                            } else {
                                F::tab_complete(ctx, input)
                            }
                        } else {
                            E::tab_complete(ctx, input)
                        }
                    } else {
                        D::tab_complete(ctx, input)
                    }
                } else {
                    C::tab_complete(ctx, input)
                }
            } else {
                B::tab_complete(ctx, input)
            }
        } else {
            A::tab_complete(ctx, input)
        }
    }
}
