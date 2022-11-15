# Errors

[OG Vid: Error Handling Isn't All About Errors](https://www.youtube.com/watch?v=rAF8mLI0naQ)

## What is Error Handling
  - defining errors
  - propagating errors and gathering context
  - reacting to specific errors
  - discarding errors
  - reporting errors and gathered context

## Recoverable vs Non-Recoverable

Most languages don't distinguish b/w these types, but Rust has no exceptions.
Rust has `panic!` for non-recoverable errors and `Result<T,E>` for recoverable errors.

Panics include a string w/ some context and hit a configurable "panic hook" which can do various things (like unwinding).

Result's use `#[must use]` which means code must handle both variants, thus errors must be explicitly handled.

## Try and ?

The `Try` traits abstracts the propagation of errors with the `try!` operator.

## The Error Trait

 - Representing an open set of errors
 - Reacting to specific errors in an open set (by downcasting)
 - reporting Interface for operators

## Fundamentals Recap

Recoverable
  - defined w/ types/traits
  - propagate with the `?` try operator
  - match/react w/ `match`/`downcast`
  - discard with `drop`/`unwrap`
  - report with `Error` trait
Non-Recoverable
  - define w/ `panic!`
  - don't propagate, they're `builtin`
  - no matching/reacting
  - [CAUTION] can discard w/ `catch_unwind`
  - report via the panic hook

## Definitions

  - **Error:** a description of why an operation failed
  - **Context:** any information relevant to an error or an error report that is not, itself, an error
  - **Error Report:** printed representation of an error and all of its associated context

## The Error Trait

Simplified version:

```rust
pub trait Error: Debug + Display {
  fn source(&self) -> Option<&(dyn Error + 'static)> {
    None
  }

  fn backtrace(&self) -> Option<&Backtrace> {
    None
  }
}
```

Simple Error:

```rust
#[derive(Debug)]
struct DeserializeError;

impl std::fmt::Display for DeserializeError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "unable to deserialize type")
  }
}

impl std::error::Error for DeserializeError {}
```

Simple reporter:

```rust
fn report(error: &(dyn std::error::Error + 'static)) {
  print!("Error:");

  let errors = std::iter::successors(Some(error), |e| e.source());

  for (ind, error) in errors.enumerate() {
    print!("\n    {}: {}", ind, error);
  }

  if let Some(backtrace) = error.backtrace() {
    print!("\n\nBacktrace: {}", backtrace);
  }
}
```

Most other languages make no distinction b/w Errors and Error Reporters.
By separating the source  from the message we move responsibility of formatting away from errors themselves making it possible get fancy.
The error trait can only represent errors with a single source.
You can only access 3 forms of context: error message, source, backtrace

## Example

```rust
fn main() -> Result<(), eyre::Report> {
  color_eyre::install()?;

  let _ = std::process::Command::new("git")
      .arg("cat")
      .output2()
      .wrap_error("the cat could not be got")
  
  Ok(())
}

trait CommandExt {
  fn outpu2(&mut self) -> Result<String, eyre::Report>;
}

impl CommandExt for std::process::Command {
  fn outpu2(&mut self) -> Result<String, eyre::Report> {
    let output = self.output()?;

    let stdout = String::from_utf8_lossy(&output.stdout).into_owned();

    if output.status.success() {
      Ok(stdout)
    } else {
      let cmd = format!("{:?}", self);

      let stderr = String::from_utf8_lossy(&output.stderr).into_owned();

      Err(eyre!("command exited unsuccessfully"))
        .section(cmd.header("Command:"))
        .section(stdout.header("stdout:"))
        .section(stderr.header("stderr:"))
    }
  }
}
```

## TIPS

  - reporters should almost always impl From<E: Error>
  - if they do that *cannot* impl Error
    - `anyhow::Error`
    - `eyre::Report`
    - `Box<dyn Error>
  - don't compose well

## Libraries

### Defining

#### `thiserror`

Define custom error types including display messages and source/from implementations

```rust
#[derive(Debug, thiserror::Error)]
pub enum DataStoreError {
  #[error("data store disconnected")]
  Disconnect(#[from] io::Error),
  #[error("the data for key `{0}` is not available")]
  Redaction(#[source] String),
  #[error("invalid header (expected {expected:?}, found {found:?})")]
  InvalidHeader {
    expected: String,
    found: String,
  }
}
```
#### `displaydoc`

Simplify thiserror display definitions with docstrings

```rust
#[derive(Debug, thiserror::Error, displaydoc::Display)]
pub enum DataStoreError {
  /// data store disconnected
  Disconnect(#[from] io::Error),
  /// the data for key `{0}` is not available
  Redaction(#[source] String),
  /// invalid header (expected {expected:?}, found {found:?})
  InvalidHeader {
    expected: String,
    found: String,
  }
}
```
#### `SNAFU`

Like `thiserror` w/ `context()` for capturing context.
Only need to capture additional context for defined variants (not source and backtrace)

```rust
#[derive(Debug, Snafu)]
enum Error {
  #[snafu(display("Unable to read configuration from {}: {}"))]
  ReadConfiguration { source: io::Error, path: PathBuf }
}

fn process_data() -> Result<(), Error> {
  let path = "config.toml";

  let configuration = fs::read_to_string(path)
      // wrap error while capturing `path` as context
      .context(ReadConfiguration { path })?;
  Ok(())
}
```

#### `anyhow`/`eyre`

Convert custom or standard errors to error reporter types

```rust
// Construct an ad-hoc error
Err(eyre!("file not found"))?

// Construct an ad-hoc wrapping error
fallible_fn()
  .wrap_err(".failed operation")?;
```

### API Stability

If you define a public enum for an error type, any new variant or change to existing variants is a breaking change.
Adding `#[non_exhaustive]` to enums or variants can prevent this

```rust
#[non_exhaustive]
#[derive(Debug, Display, Error)]
pub enum Error {
  #[non_exhaustive]
  Deserialize {
    source: io::Error,
  }
}
```

### Stack Size

```rust
struct LargeError { ... }

///   /!\ Doesn't impl Error /!\
fn fallible_fn() -> Result<(), Box<dyn Error>> {
  todo!()
}

///   \o/ Does imple Error \o/
fn fallible_fn() -> Result<(), Box<LargeError>> {
  todo!()
}
```

### Propagating

#### `fehler`

A library that adds "throw" functionality that will alter return types and wrap return results and automatically propagate errors.

```rust
fn foo(x: bool) -> i32 {
  if x {
    0
  } else {
    fehler::throw!(1);
  }
}
```
### Gathering Context

#### `tracing-error`

Tracing instrumentation library that exposes the SpanTrace type

```rust
// instrument the error
let error = std::fs::read_to_string("myfile.txt")
    .in_current_span()

// extract it from `dyn Error`
let error: &(dyn std::error::Error + 'static) = &error;
assert!(error.span_trace().is_some());
```

#### `extracterr`

`extracterr` exposes a `Bundled` type for bundling arbitrary types w/ errors.

```rust
#[derive(Debug, Display, Error)]
struct ExampleError;

type Error = extracter::Bundled<ExampleError, backtrace::Backtrace>;

let error = Error::from(ExampleError)

// extract it from `dyn Error`
let error: &(dyn std::error::Error + 'static) = &error;
assert!(error.extract::<backtrace::Backtrace>().is_some());
```

### Matching/Reacting

#### `anyhow`/`eyre`

```rust
#[derive(Display)]
/// Random error message
struct FooMessage;

let report = fallible_fn()
  .wrap_err(FooMessage)
  .unwrap_err();

assert!(report.downcast_ref::<FooMessage>().is_ok());
```

### Reporting

  - **Reporters:** `anyhow`/`eyre`
  - **Hooks:** `color-eyre`, `stable-eyre`, `jane-eyre`, `color-anyhow` (soon)

## Library vs Application

  - Library
    - don't know how users will handle errors
      - they could report, react, wrap and propagate, or discard them
    - we need error types that are maximally flexible
    - we want our errors to implement the Error trait so that they can compose w/ other errors
    - we want to be #[non_exhaustive] enums so they can easily be reacted to
    - using error **defining** libraries lets us accomplish this
  - Application
    - we know which errors we're going to handle vs report
    - we usually handle errors close to where they're returned
    - we need to create new errors that we wish to report
    - need to be able to report arbitrary errors
    - using error **reporting** libraries helps us here
  
