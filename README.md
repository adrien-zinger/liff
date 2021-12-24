# Liff 🍃 [![cargo version](https://img.shields.io/crates/v/liff.svg)](https://crates.io/crates/liff) [![CI](https://github.com/adrien-zinger/liff/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/adrien-zinger/liff/actions/workflows/ci.yml?query=branch%3Amain)

Compute the diff between two vectors with the [Levenshtein](https://en.wikipedia.org/wiki/Levenshtein_distance) algorithm.

Faster than other crate. Try `cargo bench` to see the difference.

## Usage

Levenshtein is a O(n1n2) time complexty and in space so you don't want to use it with a big raw. But you can use it with good chunks or text content!

```rust
    let from: Vec<u8> = input_text_from.as_bytes().into();
    let to: Vec<u8> = input_text_to.as_bytes().into();
    let diff = diff::diff::<u8>(&from, &to);
```

### Use the diff output

The diff output can be used as a patch from another part of your code.

```rust
    let res = apply::apply(from, &diff);
    // res should be equals to "to"
```

### Write temporary a compressed patch

the diff patch is quite bigger than we can do. You can compress it and that's what we do when we dump a file. You can read and write a diff with these functions.

```rust
    diffio::write(std::path::Path::new("diff.d"), diff.clone());
    let diff = diffio::read(std::path::Path::new("diff.d"));
```


#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>
