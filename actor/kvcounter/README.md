# Key value counter

This actor accepts http GET requests, and 
increments a counter whose name is based on the url path.
Each unique url is associated a unique counter.
The result is returned in a JSON payload as follows:

```json
{
    "counter": 12
}
```

This actor makes use of the HTTP server (`wasmcloud:httpserver`) capability 
and the key-value store capability (`wasmcloud:keyvalue`). 

As usual, it is worth noting that this actor does _not_ know 
where its HTTP server comes from, nor does it know which 
key-value implementation the host runtime has provided.

## Components POC

Use the latest [wasm-tools](https://github.com/bytecodealliance/wasm-tools) for gluing wasm modules together.

```bash
git clone https://github.com/bytecodealliance/wasm-tools.git
cd wasm-tools
cargo install wasm-tools
```

Specifically we will use [wasm-compose](https://github.com/bytecodealliance/wasm-tools/tree/main/crates/wasm-compose).

### Usage

```bash
cd wasmbus-sender
cargo component build
cd ../keyvalue
cargo component build
cd ../httpserver
cargo component build
cd ../actor
cargo component build

wasm-tools compose -c config.yml -o actor.wasm ./actor/target/wasm32-unknown-unknown/debug/kvcounter_actor.wasm
```
