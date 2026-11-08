# dhoni (ধ্বনি)
[![Build Status](https://travis-ci.org/OpenBangla/dhoni.svg?branch=master)](https://travis-ci.org/OpenBangla/dhoni)
[![crates.io](https://img.shields.io/crates/v/dhoni.svg)](https://crates.io/crates/dhoni)
[![DOCS.rs](https://docs.rs/dhoni/badge.svg)](https://docs.rs/dhoni)

A crate for converting Bengali text into a phonetic Roman counterpart (Avro, Khipro).

## Usage
Add this to your `Cargo.toml`:
```toml
[dependencies]
dhoni = "0.1"
```

## Example
```rust
use dhoni::{convert_to_phonetic, Mode};

let banglish = convert_to_phonetic("আমি", Mode::Avro);
assert_eq!(banglish, "ami");

let khipro = convert_to_phonetic("চাঁদ", Mode::Khipro);
assert_eq!(khipro, "ca/d");
```

## License
`dhoni` is distributed under the terms of MIT License.

See [LICENSE](https://github.com/OpenBangla/dhoni/blob/master/LICENSE) for details.
