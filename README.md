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

### GitHub Actions

Some [standard GitHub Actions config files](https://gist.github.com/LukeMathWalker/5ae1107432ce283310c3e601fac915f3) as linked in book

Put all three files in `.github/workflows` and then magic!

**UPDATE**:

### actix-web

Ref links from book:

- [actix-web main site](https://actix.rs/)
- [actix-web docs](https://docs.rs/actix-web/latest/actix_web/)
- [actix-web examples](https://github.com/actix/examples/)

### cargo-expand

Install and use like

```
cargo install cargo-expand
cargo expand
```

### Arguments to Request Handlers

> All arguments in the signature of a route handler must implement the `FromRequest` trait.

## TODOS

- get code coverage to show on github
- perform compatibility testing with newest sqlx-cli verion
- do better than "SKIP_DOCKER" flag in `scripts/init_db.sh`
- refactor `routes/mod.rs` to `routes.rs`
- look into clippy warning in the routes mod saying the re-exports aren't used
- get familiar with the GitHub Actions specs
