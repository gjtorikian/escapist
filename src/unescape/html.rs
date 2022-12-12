use crate::{likely, unescape, unlikely};

use super::CMARK_CTYPE_CLASS;

// pub fn isspace(ch: u8) -> bool {
//     CMARK_CTYPE_CLASS[ch as usize] == 1
// }

// pub fn ispunct(ch: u8) -> bool {
//     CMARK_CTYPE_CLASS[ch as usize] == 2
// }

pub fn isdigit(ch: u8) -> bool {
    CMARK_CTYPE_CLASS[ch as usize] == 3
}

// pub fn isalpha(ch: u8) -> bool {
//     CMARK_CTYPE_CLASS[ch as usize] == 4
// }

// pub fn isalnum(ch: u8) -> bool {
//     CMARK_CTYPE_CLASS[ch as usize] == 3 || CMARK_CTYPE_CLASS[ch as usize] == 4
// }

pub fn unescape_html(src: &[u8]) -> Vec<u8> {
    let size = src.len();
    let mut i = 0;
    let mut v = Vec::with_capacity(size);

    while i < size {
        let org = i;
        while i < size && src[i] != b'&' {
            i += 1;
        }

        if likely(i > org) {
            if unlikely(org == 0) && i >= size {
                return src.to_vec();
            }

            v.extend_from_slice(&src[org..i]);
        }

        // escaping
        if i >= size {
            return v;
        }

        i += 1;
        match unescape(&src[i..]) {
            Some((chs, size)) => {
                v.extend_from_slice(&chs);
                i += size;
            }
            None => v.push(b'&'),
        }
    }

    v
}
