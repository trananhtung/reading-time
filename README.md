# reading-time

[![All Contributors](https://img.shields.io/badge/all_contributors-1-orange.svg?style=flat-square)](#contributors-)

[![crates.io](https://img.shields.io/crates/v/reading-time.svg)](https://crates.io/crates/reading-time)
[![docs.rs](https://docs.rs/reading-time/badge.svg)](https://docs.rs/reading-time)
[![CI](https://github.com/trananhtung/reading-time/actions/workflows/ci.yml/badge.svg)](https://github.com/trananhtung/reading-time/actions/workflows/ci.yml)
[![license](https://img.shields.io/crates/l/reading-time.svg)](#license)

**Estimate how long a text takes to read.**

Counts the words in a text and estimates reading time, with sensible handling of CJK
characters (each counts as a word, and trailing punctuation is absorbed). A faithful Rust
port of the widely-used [`reading-time`](https://www.npmjs.com/package/reading-time) npm
package — handy for blogs, CMSs, and feeds.

- **Zero dependencies**
- Word count, minutes, milliseconds, and a `"N min read"` summary
- CJK / Hiragana / Hangul aware
- Differential-tested against the reference `reading-time` implementation (60k cases)

## Install

```toml
[dependencies]
reading-time = "0.1"
```

## Usage

```rust
use reading_time::{reading_time, reading_time_with};

let result = reading_time("Hello world, this is a test.");
assert_eq!(result.words, 6);
assert_eq!(result.text, "1 min read");

// Custom words-per-minute (default 200):
let slow = reading_time_with(&"word ".repeat(300), 200);
assert_eq!(slow.minutes, 1.5);
assert_eq!(slow.text, "2 min read");
```

The returned [`ReadingTime`] exposes `text`, `minutes` (fractional), `time` (milliseconds),
and `words`.

## Contributors ✨

This project follows the [all-contributors](https://github.com/all-contributors/all-contributors) specification. Contributions of any kind are welcome — code, docs, bug reports, ideas, reviews! See the [emoji key](https://allcontributors.org/docs/en/emoji-key) for how each contribution is recognized, and open a PR or issue to get involved.

Thanks goes to these wonderful people:

<!-- ALL-CONTRIBUTORS-LIST:START - Do not remove or modify this section -->
<!-- prettier-ignore-start -->
<!-- markdownlint-disable -->
<table>
  <tbody>
    <tr>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/trananhtung"><img src="https://avatars.githubusercontent.com/u/30992229?v=4?s=100" width="100px;" alt="Tung Tran"/><br /><sub><b>Tung Tran</b></sub></a><br /><a href="https://github.com/trananhtung/./commits?author=trananhtung" title="Code">💻</a> <a href="#maintenance-trananhtung" title="Maintenance">🚧</a></td>
    </tr>
  </tbody>
</table>

<!-- markdownlint-restore -->
<!-- prettier-ignore-end -->

<!-- ALL-CONTRIBUTORS-LIST:END -->

## License

Licensed under either of [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE) at your option.
