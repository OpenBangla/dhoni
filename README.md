# dhoni (ধ্বনি)
[![Build Status](https://travis-ci.org/OpenBangla/dhoni.svg?branch=master)](https://travis-ci.org/OpenBangla/dhoni)
[![crates.io](https://img.shields.io/crates/v/dhoni.svg)](https://crates.io/crates/dhoni)
[![DOCS.rs](https://docs.rs/dhoni/badge.svg)](https://docs.rs/dhoni)

A crate for converting Bengali text into their phonetic counterpart.

*Requires Rust version >= 1.31*

## Usage
Add this to your `Cargo.toml`:
```toml
[dependencies]
dhoni = "0.1"
```

## Example
This example shows how to use this crate:
```rust
use dhoni::convert_to_phonetic;

let banglish = convert_to_phonetic("আমি");
assert_eq!(banglish, "ami");
```

## License
`dhoni` is distributed under the terms of MIT License.

See [LICENSE](https://github.com/OpenBangla/dhoni/blob/master/LICENSE) for details.
