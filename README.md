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

actix-web endpoint handlers will automatically return 400 if they fail to deserialize the request parameters

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

### Digital Ocean CLI

List apps with `doctl apps list`.
The first column is the App ID.

Create a new app with `doctl apps create --spec spec.yaml`.
The [spec YAML](./spec.yaml) file declaritively specifies app resources.
For this app we have a service and a database.

Update apps with `doctl apps update APP_ID --spec=spec.yaml`.

Migrate the database with `DATABASE_URL=<app-db-connection-string> sqlx migrate run`.
You can find the connection string in "Connection Details" at "https://cloud.digitalocean.com/apps/APP_ID/settings/DB_NAME"

### [notes about error video](error-handling-isnt-all-about-errors.nodes.md)

### Cargo sqlx

Update the offline [sqlx schema file](./sqlx-data.json) via:
`cargo sqlx prepare -- --lib`

## TODOS

- triage these TODOs
- get code coverage to show on github
- perform compatibility testing with newest sqlx-cli version
- do better than "SKIP_DOCKER" flag in `scripts/init_db.sh`
- refactor `routes/mod.rs` to `routes.rs`
- look into clippy warning in the routes mod saying the re-exports aren't used
- get familiar with the GitHub Actions specs
  - fix "Node.js 12 actions are deprecated" warnings
- get GitHub Actions tests to succeed w/o committing:
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
- how do extension traits work around rust's orphan rule?
- refactor database access into some kind of DAL
- break subscriptions.rs into its own module
- create some functionality to easily execute postgres workflows inside transactions
- create a macro (ooh) for using chain_fmt_error for Debug implementations (lofty?)
- set argon params manually to recommended rather than use default see pg 404 (hopefully I can find it)
