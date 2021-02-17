# `rust-fullstack-template`

**Kickstart your full-stack Rust project!**

This template comes pre-configured with all the boilerplate for a rust application distributed between a browser component and a server component.

* `cargo run` -- Serve the project locally for
  development at `http://localhost:8080`.

* `cargo build` -- Build the project (in production mode)

## What's inside?

This template tries not to put more emphasis on the frontend or backend parts. So the main lib project is one that is shared between both parts and the individual parts are a little more hidden away.

```
root - base crate - fullstack application
| \- web - parcel bundled frontend
| | \- crate - frontend wasm crate
| |- js - frontend js
|\- server - backend crate

## Using This Template

Requirements:

* npm
* rust toolchain

```sh
cargo install wasm-pack
```

```sh
# crate from template
```
