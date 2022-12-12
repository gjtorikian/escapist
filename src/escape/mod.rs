mod href;
mod html;

use std::fmt::{Arguments, Write as FmtWrite};
use std::io::{self, ErrorKind, Write};

/// This wrapper exists because we can't have both a blanket implementation
/// for all types implementing `Write` and types of the for `&mut W` where
/// `W: StrWrite`. Since we need the latter a lot, we choose to wrap
/// `Write` types.
#[derive(Debug)]
pub struct WriteWrapper<W>(pub W);

/// Trait that allows writing string slices. This is basically an extension
/// of `std::io::Write` in order to include `String`.
pub trait StrWrite {
    fn write_str(&mut self, s: &str) -> io::Result<()>;

    fn write_fmt(&mut self, args: Arguments) -> io::Result<()>;
}

impl<W> StrWrite for WriteWrapper<W>
where
    W: Write,
{
    #[inline]
    fn write_str(&mut self, s: &str) -> io::Result<()> {
        self.0.write_all(s.as_bytes())
    }

    #[inline]
    fn write_fmt(&mut self, args: Arguments) -> io::Result<()> {
        self.0.write_fmt(args)
    }
}

impl<'w> StrWrite for String {
    #[inline]
    fn write_str(&mut self, s: &str) -> io::Result<()> {
        self.push_str(s);
        Ok(())
    }

    #[inline]
    fn write_fmt(&mut self, args: Arguments) -> io::Result<()> {
        // FIXME: translate fmt error to io error?
        FmtWrite::write_fmt(self, args).map_err(|_| ErrorKind::Other.into())
    }
}

impl<W> StrWrite for &'_ mut W
where
    W: StrWrite,
{
    #[inline]
    fn write_str(&mut self, s: &str) -> io::Result<()> {
        (**self).write_str(s)
    }

    #[inline]
    fn write_fmt(&mut self, args: Arguments) -> io::Result<()> {
        (**self).write_fmt(args)
    }
}

pub use href::*;
pub use html::*;
