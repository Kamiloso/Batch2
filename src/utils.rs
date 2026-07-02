use std::fmt;

pub mod cli;

pub fn fmt_join<W, T>(
    w: &mut W,
    iter: impl IntoIterator<Item = T>,
    join: &str
) -> fmt::Result
where
    W: fmt::Write,
    T: fmt::Display
{
    let mut first = true;
    for item in iter {
        if first {
            write!(w, "{}", item)?;
            first = false;
        } else {
            write!(w, "{}{}", join, item)?;
        }
    }

    Ok(())
}