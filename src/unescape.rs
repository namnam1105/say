// Edited code from "unescape" rust crate.
// Credits to Saghm Rossi

use std::iter::Peekable;
use std::str::Chars;

macro_rules! try_option {
    ($o:expr) => {
        match $o {
            Some(s) => s,
            None => return None,
        }
    };
}

pub fn unescape(s: &str) -> Option<String> {
    let mut chars = s.chars().peekable();
    let mut result = String::new();

    while let Some(c) = chars.next() {
        if c != '\\' {
            result.push(c);
            continue;
        }

        match chars.next() {
            Some('b') => result.push('\u{0008}'),
            Some('f') => result.push('\u{000C}'),
            Some('n') => result.push('\n'),
            Some('r') => result.push('\r'),
            Some('t') => result.push('\t'),
            Some('\'') => result.push('\''),
            Some('\"') => result.push('\"'),
            Some('\\') => result.push('\\'),
            Some('u') => result.push(try_option!(unescape_unicode(&mut chars))),
            Some('x') => result.push(try_option!(unescape_byte(&mut chars))),
            Some(c) if c.is_digit(8) => result.push(try_option!(unescape_octal(c, &mut chars))),
            _ => return None,
        };
    }

    Some(result)
}

fn unescape_unicode(chars: &mut Peekable<Chars>) -> Option<char> {
    let mut s = String::new();
    for _ in 0..4 {
        s.push(try_option!(chars.next()));
    }
    let u = try_option!(u32::from_str_radix(&s, 16).ok());
    char::from_u32(u)
}

fn unescape_byte(chars: &mut Peekable<Chars>) -> Option<char> {
    let mut s = String::new();
    for _ in 0..2 {
        s.push(try_option!(chars.next()));
    }
    let u = try_option!(u32::from_str_radix(&s, 16).ok());
    char::from_u32(u)
}

fn unescape_octal(c: char, chars: &mut Peekable<Chars>) -> Option<char> {
    let mut s = String::new();
    s.push(c);
    
    if let Some(&next) = chars.peek() {
        if next.is_digit(8) {
            s.push(chars.next().unwrap());
        }
    }
    
    if let Some(&next) = chars.peek() {
        if next.is_digit(8) {
            s.push(chars.next().unwrap());
        }
    }

    let u = try_option!(u32::from_str_radix(&s, 8).ok());
    char::from_u32(u)
}
