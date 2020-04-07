[![Crates.io](https://img.shields.io/crates/v/versuch.svg)](https://crates.io/crates/versuch)
[![API reference](https://docs.rs/versuch/badge.svg)](https://docs.rs/versuch/)

# Versuch

Procedural macro polyfill for Ok-wrapping functions.

```rust
use versuch::try_fn;

#[try_fn]
fn word_count(path: &Path) -> io::Result<usize> {
    let mut res = 0;
    let file = fs::File::open(path)?;
    let mut reader = io::BufReader::new(file);
    for line in reader.lines() {
        let line = line?;
        res += line.split_whitespace().count();
    }
    res
}
```

This is a polyfill for the following imaginary syntax:

```rust
fn word_count(path: &Path) -> io::Result<usize> try {
    let mut res = 0;
    let file = fs::File::open(path)?;
    let mut reader = io::BufReader::new(file);
    for line in reader.lines() {
        let line = line?;
        res += line.split_whitespace().count();
    }
    res
}
```
