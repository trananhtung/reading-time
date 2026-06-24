//! Integration tests exercising the public API of `reading-time`.

use reading_time::{reading_time, reading_time_with};

#[test]
fn blog_post() {
    let article = "The quick brown fox jumps over the lazy dog. ".repeat(50);
    let result = reading_time(&article);
    assert_eq!(result.words, 450);
    assert_eq!(result.text, "3 min read");
    assert_eq!(result.minutes, 2.25);
}

#[test]
fn mixed_scripts() {
    let result = reading_time("Rust 是 一 门 系统 编程 语言");
    // "Rust" + 9 CJK characters = 10 words.
    assert_eq!(result.words, 10);
}

#[test]
fn custom_rate_rounds_up_display() {
    // 101 words at 100 wpm = 1.01 minutes -> "2 min read".
    let result = reading_time_with(&"word ".repeat(101), 100);
    assert_eq!(result.words, 101);
    assert_eq!(result.text, "2 min read");
}

#[test]
fn whitespace_only() {
    let result = reading_time("   \n\t  ");
    assert_eq!((result.words, result.text.as_str()), (0, "0 min read"));
}
