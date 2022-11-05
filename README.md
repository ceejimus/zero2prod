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

### Resource Acquisition Is Initialization (RAII)

Often implemented in rust by piggybacking on the `Drop` trait.

### Running cargo tools using nightly

`cargo +nightly command`

## TODOS

- get code coverage to show on github
- perform compatibility testing with newest sqlx-cli verion
- do better than "SKIP_DOCKER" flag in `scripts/init_db.sh`
- refactor `routes/mod.rs` to `routes.rs`
- look into clippy warning in the routes mod saying the re-exports aren't used
- get familiar with the GitHub Actions specs
  - fix "Node.js 12 actions are deprecated" warnings
- get GitHub Actions tests to succeed w/o committing .env file
- get GitHub Actions tests to succeed w/o committing Cargo.lock file
- get GitHub Actions tests to succed w/o committing:
  - Cargo.lock - hash of this file used in cache
  - .env file
    - the problem is sqlx macros need the DATABASE_URL environment variable and grab it from `.env`
    - see [the docs for setting environment variables](https://docs.github.com/en/enterprise-cloud@latest/actions/learn-github-actions/environment-variables)
    - see [this custom actions example](https://github.com/ozaytsev86/create-env-file-action) for getting values from GitHub secrets
  - configuration.yaml - needed to load configurations
- the hash of this file is used to cache dependencies
- move to use Environment Files instead of `save-state` in GitHub actions
- see [this post](https://github.blog/changelog/2022-10-11-github-actions-deprecating-save-state-and-set-output-commands/)
  0
- look into [higher-ranked trait bound (HRTB)](https://doc.rust-lang.org/nomicon/hrtb.html)
- make docker image tiny
  - see [rust-musl-builder](https://github.com/emk/rust-musl-builder) - pg #150 in book
  - see [how to strip symbols](https://github.com/johnthagen/min-sized-rust#strip-symbols-from-binary)
- documentation!
