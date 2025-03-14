## say

this project is an `echo` alternative written in `rust` for fun.

## Installation [[crates.io](https://crates.io/crates/say-rust)]
** Available on [crates.io](https://crates.io/crates/say-rust)!!

- Install with `cargo`
```bash
cargo install say-rust
```
- Use `say`
```bash
say "we love rust!!\n" -f
```

## Compilation [Building]

- Copy this `git` repository
```bash
git clone https://github.com/namnam1105/say
```
- Install cargo and rustc
- To compile run:
```bash
cargo build --release
```
- To install say run:
```bash
cargo install --path .
```
- Use `say`
```bash
say "we love rust!!\n" -f
```


## Syntax


```bash
say [OPTIONS] "TEXT"
```
** Using quotes is not necessary but it helps reduce error with different shells.

Options:
- -f --format: enables formatting with ANSI escapes
- -h --help: print out built-in help
