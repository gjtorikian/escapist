use crate::StrWrite;

use std::{io::Result, str::from_utf8};

#[rustfmt::skip]
static HREF_SAFE: [u8; 128] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 1, 0, 1, 1, 1, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 0, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 1,
    0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 0,
];

static HEX_CHARS: &[u8] = b"0123456789ABCDEF";
static AMP_ESCAPE: &str = "&amp;";
static SINGLE_QUOTE_ESCAPE: &str = "&#x27;";

/// Writes an href to the buffer, escaping href unsafe bytes.
pub fn escape_href<W>(mut w: W, s: &str) -> Result<()>
where
    W: StrWrite,
{
    let bytes = s.as_bytes();
    let mut mark = 0;
    for i in 0..bytes.len() {
        let c = bytes[i];
        if c >= 0x80 || HREF_SAFE[c as usize] == 0 {
            // character needing escape

            // write partial substring up to mark
            if mark < i {
                w.write_str(&s[mark..i])?;
            }
            match c {
                b'&' => {
                    w.write_str(AMP_ESCAPE)?;
                }
                b'\'' => {
                    w.write_str(SINGLE_QUOTE_ESCAPE)?;
                }
                _ => {
                    let mut buf = [0u8; 3];
                    buf[0] = b'%';
                    buf[1] = HEX_CHARS[((c as usize) >> 4) & 0xF];
                    buf[2] = HEX_CHARS[(c as usize) & 0xF];
                    let escaped = from_utf8(&buf).unwrap();
                    w.write_str(escaped)?;
                }
            }
            mark = i + 1; // all escaped characters are ASCII
        }
    }
    w.write_str(&s[mark..])
}
