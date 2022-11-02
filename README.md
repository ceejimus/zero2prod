# zero2prod

Learning project from the book by Luca Palmieri

## Notes

_some general notes as I go along_

### LLD

lld is a faster linker than Rust's default

lld can be configured per build target by special sections in the [Cargo config](.cargo/config.toml)

### cargo-watch

Install and use like

```
cargo install cargo-watch
cargo watch -x check -x test -x run
```

### cargo-tarpaulin

Install and use like

```
cargo install cargo-tarpaulin
cargo tarpaulin --ignore-tests
```

TODO: get code coverage to show on github

### GitHub Actions

Some [standard GitHub Actions config files](https://gist.github.com/LukeMathWalker/5ae1107432ce283310c3e601fac915f3) as linked in book
