# wasm-lambda

[![CI](https://github.com/ChenKS12138/wasm-lambda/actions/workflows/CI.yml/badge.svg)](https://github.com/ChenKS12138/wasm-lambda/actions/workflows/CI.yml)

A serverless service tool based on [WebAssembly](https://webassembly.org), provides a simple way to write serverless module.

## Usage

1. Clone And Install

```shell
git clone https://github.com/ChenKS12138/wasm-lambda.git
cd wasm-lambda
cargo install --path .
```

2. Build Example `hello-world`

```shell
cd examples/hello-world
cargo build --release
```

3. Dev And Auto-reload

```shell
# Start Dev Server
wasm-lambda dev --bind 0.0.0.0:3000 -m hello-world:./target/wasm32-wasi/release/hello-world.wasm

# Rebuild
# cargo build
```

4. Send Request

Open Browser, visit `http://127.0.0.1:3000/hello-world/latest/`

OR

```shell
curl -iL http://127.0.0.1:3000/hello-world/latest/
curl -i http://127.0.0.1:3000/hello-world/latest/api/capitalize/hello-world
curl -i http://127.0.0.1:3000/hello-world/latest/api/echo-query?name=cattchen&age=21
curl -i -X POST -d '{"name":"cattchen","age":21}' http://127.0.0.1:3000/hello-world/latest/api/echo-person
```
