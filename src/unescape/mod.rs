mod html;

use entities::ENTITIES;
use std::cmp::min;
use std::str::from_utf8_unchecked;

pub const ENTITY_MIN_LENGTH: usize = 2;
pub const ENTITY_MAX_LENGTH: usize = 32;

#[rustfmt::skip]
const CMARK_CTYPE_CLASS: [u8; 256] = [
    /*      0  1  2  3  4  5  6  7  8  9  a  b  c  d  e  f */
    /* 0 */ 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 1, 0, 0,
    /* 1 */ 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    /* 2 */ 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
    /* 3 */ 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 2, 2, 2, 2, 2, 2,
    /* 4 */ 2, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
    /* 5 */ 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 2, 2, 2, 2, 2,
    /* 6 */ 2, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
    /* 7 */ 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 2, 2, 2, 2, 0,
    /* 8 */ 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    /* 9 */ 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    /* a */ 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    /* b */ 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    /* c */ 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    /* d */ 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    /* e */ 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    /* f */ 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];

pub fn unescape(text: &[u8]) -> Option<(Vec<u8>, usize)> {
    let mut has_semicolon = 0;
    if text.len() >= 3 && text[0] == b'#' {
        let mut codepoint: u32 = 0;
        let mut i = 0;

        let num_digits = if isdigit(text[1]) {
            i = 1;
            while i < text.len() && isdigit(text[i]) {
                codepoint = (codepoint * 10) + (text[i] as u32 - '0' as u32);
                codepoint = min(codepoint, 0x11_0000);
                i += 1;
            }
            i - 1
        } else if text[1] == b'x' || text[1] == b'X' {
            i = 2;
            while i < text.len() && isxdigit(&text[i]) {
                codepoint = (codepoint * 16) + ((text[i] as u32 | 32) % 39 - 9);
                codepoint = min(codepoint, 0x11_0000);
                i += 1;
            }
            i - 2
        } else {
            0
        };

        if num_digits >= 1 && num_digits <= 8 && i < text.len() {
            if codepoint == 0
                || (codepoint >= 0xD800 && codepoint <= 0xE000)
                || codepoint >= 0x110000
            {
                codepoint = 0xFFFD;
            }

            if text[i] == b';' {
                has_semicolon = 1;
            }

            return Some((
                char::from_u32(codepoint)
                    .unwrap_or('\u{FFFD}')
                    .to_string()
                    .into_bytes(),
                i + has_semicolon,
            ));
        }
    }

    let size = min(text.len(), ENTITY_MAX_LENGTH);
    for i in ENTITY_MIN_LENGTH..size {
        if text[i] == b' ' {
            return None;
        }

        if text[i] == b';' {
            return lookup(&text[..i]).map(|e| (e.to_vec(), i + 1));
        }
    }

    None
}

fn lookup(text: &[u8]) -> Option<&[u8]> {
    let entity_str = format!("&{};", unsafe { from_utf8_unchecked(text) });

    let entity = ENTITIES.iter().find(|e| e.entity == entity_str);

    match entity {
        Some(e) => Some(e.characters.as_bytes()),
        None => None,
    }
}

fn isxdigit(ch: &u8) -> bool {
    (*ch >= b'0' && *ch <= b'9') || (*ch >= b'a' && *ch <= b'f') || (*ch >= b'A' && *ch <= b'F')
}

pub use html::*;
