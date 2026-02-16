# Rust: Axum WebApp

This project is a follow-along of
[Jeremy Chone's Rust Production Coding: Web App](https://www.youtube.com/playlist?list=PL7r-PXl6ZPcCTTxjmsb9bFZB9i01fAtI7)
series. It implements a production-style web application with a variety of
different features.

## Running

As this is a Rust project, everything can be set up with a simple `git clone`
and `cargo run`.

I recommend going further and installing
[Bacon](https://github.com/Canop/bacon):

```zsh
cargo install --locked bacon
# Run the client
bacon
```

Bacon supports quickly checking tests (with `test`) and quickly loading the
server.

### Quick note: Windows

Unfortunately (for some reason), Windows does not support running multiple
`cargo watch` or `bacon` commands in parallel. You will get an error like the
following:

```zsh
error: failed to remove file <PATH>

Caused by:
Access is denied. (os error 5)
```

Since I am developing on Windows, I refactored a decent portion of the tester
code (and the server code) to expose the server and quickly run it.

## Implemented Features

### Core Functionality

- Simple HTTP `GET` routing and responses.

### Testing

- Configured integration testing for server routes.
