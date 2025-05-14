# wasm-on-server-runtime-rust

This repository is part of the "[Execute untrusted customer code on the server with Web Assembly (Wasm) components](https://mootoday.com/blog/execute-untrusted-customer-code-on-the-server-with-wasm-components)" blog post.

It contains a web server that exposes a POST endpoint at http://127.0.0.1:8080/run. The request payload is a WebAssembly component binary which will be executed in a Rust WebAssembly engine.

The Wasm Interface Type (WIT) used by this engine can be found at [./wit/world.wit](./wit/world.wit).

To look at examples of how to create a WebAssembly component for this engine:

- [github.com/mootoday/wasm-on-server-guest-rust](https://github.com/mootoday/wasm-on-server-guest-rust)
- [github.com/mootoday/wasm-on-server-guest-javascript](https://github.com/mootoday/wasm-on-server-guest-javascript)

For guides to write Wasm components in other languages, please [refer to the official docs](https://component-model.bytecodealliance.org/language-support.html).

## Getting started

1. Install <a href="https://www.jetify.com/docs/devbox/" target="_blank">Devbox</a>
   ```
   curl -fsSL https://get.jetify.com/devbox | bash
   ```
1. Run the web server
   ```
   devbox run dev
   ```
1. Build and deploy either a [Rust](https://github.com/mootoday/wasm-on-server-guest-rust) or [Javascript](https://github.com/mootoday/wasm-on-server-guest-javascript) Wasm component.
