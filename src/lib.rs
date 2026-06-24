//! # reading-time — estimate how long a text takes to read
//!
//! Count the words in a text and estimate the reading time, with sensible handling of CJK
//! characters (each counts as a word, and trailing punctuation is absorbed). A faithful Rust
//! port of the widely-used [`reading-time`](https://www.npmjs.com/package/reading-time) npm
//! package.
//!
//! ```
//! use reading_time::reading_time;
//!
//! let result = reading_time("Hello world, this is a test.");
//! assert_eq!(result.words, 6);
//! assert_eq!(result.text, "1 min read");
//! ```
//!
//! Use [`reading_time_with`] to set the words-per-minute rate (default 200):
//!
//! ```
//! use reading_time::reading_time_with;
//! assert_eq!(reading_time_with(&"word ".repeat(400), 200).minutes, 2.0);
//! ```
//!
//! **Zero dependencies.** (Uses `std` for floating-point rounding.)

#![forbid(unsafe_code)]
#![doc(html_root_url = "https://docs.rs/reading-time/0.1.0")]
#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss,
    clippy::cast_possible_wrap,
    // Reading-time minutes are simple ratios; exact comparison in tests is intentional.
    clippy::float_cmp
)]

// Compile-test the README's examples as part of `cargo test`.
#[cfg(doctest)]
#[doc = include_str!("../README.md")]
struct ReadmeDoctests;

/// The result of a reading-time estimate.
#[derive(Debug, Clone, PartialEq)]
pub struct ReadingTime {
    /// A human-readable summary, e.g. `"3 min read"`.
    pub text: String,
    /// The estimated reading time in minutes (fractional).
    pub minutes: f64,
    /// The estimated reading time in milliseconds (rounded).
    pub time: i64,
    /// The number of words counted.
    pub words: usize,
}

fn in_ranges(code: u32, ranges: &[(u32, u32)]) -> bool {
    ranges
        .iter()
        .any(|&(low, high)| low <= code && code <= high)
}

/// A CJK character (each is counted as a standalone word).
fn is_cjk(unit: Option<u16>) -> bool {
    match unit {
        Some(u) => in_ranges(
            u32::from(u),
            &[
                (0x3040, 0x309F),   // Hiragana
                (0x4E00, 0x9FFF),   // CJK Unified Ideographs
                (0xAC00, 0xD7A3),   // Hangul
                (0x20000, 0x2EBE0), // CJK extensions (never matches a 16-bit unit; kept for parity)
            ],
        ),
        None => false,
    }
}

/// The default ASCII word boundary set: space, newline, carriage return, tab.
fn is_word_bound(unit: Option<u16>) -> bool {
    matches!(unit, Some(0x20 | 0x0A | 0x0D | 0x09))
}

/// Punctuation absorbed after a CJK character.
fn is_punctuation(unit: Option<u16>) -> bool {
    match unit {
        Some(u) => in_ranges(
            u32::from(u),
            &[
                (0x21, 0x2F),
                (0x3A, 0x40),
                (0x5B, 0x60),
                (0x7B, 0x7E),
                (0x3000, 0x303F), // CJK symbols and punctuation
                (0xFF00, 0xFFEF), // full-width forms
            ],
        ),
        None => false,
    }
}

/// Estimate the reading time of `text` at 200 words per minute.
///
/// ```
/// # use reading_time::reading_time;
/// assert_eq!(reading_time("one two three").words, 3);
/// ```
#[must_use]
pub fn reading_time(text: &str) -> ReadingTime {
    reading_time_with(text, 200)
}

/// Estimate the reading time of `text` at `words_per_minute` (a value of `0` falls back to
/// the default of 200, matching the reference).
#[must_use]
pub fn reading_time_with(text: &str, words_per_minute: usize) -> ReadingTime {
    let words_per_minute = if words_per_minute == 0 {
        200
    } else {
        words_per_minute
    };

    // Work on UTF-16 code units (as the reference indexes the string), with a trailing
    // newline appended so the final word is detected.
    let units: Vec<u16> = text.encode_utf16().collect();
    let length = units.len();
    let get = |i: isize| -> Option<u16> {
        usize::try_from(i).ok().and_then(|i| {
            if i == length {
                Some(0x0A) // the appended `\n`
            } else {
                units.get(i).copied()
            }
        })
    };

    // Trim leading and trailing word boundaries.
    let mut start = 0isize;
    while (start as usize) < length && is_word_bound(get(start)) {
        start += 1;
    }
    let mut end = length as isize - 1;
    while end >= 0 && is_word_bound(get(end)) {
        end -= 1;
    }

    let mut words = 0usize;
    let mut i = start;
    while i <= end {
        let current = get(i);
        let next = get(i + 1);
        // A CJK character is always a word; otherwise a non-boundary followed by a boundary
        // (or a CJK character) ends a word.
        if is_cjk(current) || (!is_word_bound(current) && (is_word_bound(next) || is_cjk(next))) {
            words += 1;
        }
        // After a CJK character, absorb following punctuation and boundaries.
        if is_cjk(current) {
            while i <= end && (is_punctuation(get(i + 1)) || is_word_bound(get(i + 1))) {
                i += 1;
            }
        }
        i += 1;
    }

    let minutes = words as f64 / words_per_minute as f64;
    let time = (minutes * 60.0 * 1000.0).round() as i64;
    // `Math.ceil(minutes.toFixed(2))`: round to two decimals (ties away from zero), then ceil.
    let displayed = ((minutes * 100.0).round() / 100.0).ceil() as i64;

    ReadingTime {
        text: format!("{displayed} min read"),
        minutes,
        time,
        words,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let r = reading_time("Hello world, this is a test.");
        assert_eq!(r.words, 6);
        assert_eq!(r.minutes, 0.03);
        assert_eq!(r.time, 1800);
        assert_eq!(r.text, "1 min read");
    }

    #[test]
    fn word_count_boundaries() {
        assert_eq!(reading_time("word ".repeat(200).trim()).words, 200);
        assert_eq!(reading_time("word ".repeat(200).trim()).text, "1 min read");
        assert_eq!(
            reading_time("w ".repeat(201).trim()).text,
            "1 min read"
        ); // 1.005 -> 1
        assert_eq!(
            reading_time("w ".repeat(300).trim()).text,
            "2 min read"
        );
        assert_eq!(reading_time("  leading and trailing  ").words, 3);
        assert_eq!(reading_time("a\nb\tc\rd").words, 4);
    }

    #[test]
    fn cjk() {
        assert_eq!(reading_time("中文字符测试一二三").words, 9);
        assert_eq!(reading_time("hello 中文 world").words, 4);
        assert_eq!(reading_time("ありがとう").words, 5); // hiragana
        assert_eq!(reading_time("こんにちは世界").words, 7);
        // CJK followed by punctuation: the punctuation is absorbed.
        assert_eq!(reading_time("中。文，字！").words, 3);
    }

    #[test]
    fn empty_and_wpm() {
        let r = reading_time("");
        assert_eq!(
            (r.words, r.minutes, r.time, r.text.as_str()),
            (0, 0.0, 0, "0 min read")
        );
        assert_eq!(reading_time_with("word", 0).minutes, 0.005); // 0 -> default 200
        assert_eq!(reading_time_with("one two three", 100).minutes, 0.03);
    }
}
