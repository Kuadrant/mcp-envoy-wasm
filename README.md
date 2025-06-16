# mcp-envoy-wasm

# Local LibreChat + MCP + http-streamable + Envoy Setup

TODO add intro

## Prerequisites

- Docker
- npm (Node Package Manager)
- rust - https://www.rust-lang.org/tools/install

## LibreChat

### Setup

Follow the librechat installation guide to install librechat via [docker](https://www.librechat.ai/docs/local/docker)

### Configuration

Use the docker-compose-override.yaml and libre-config.json provided in this repo to configure LibreChat.

### Run LibreChat Locally

```bash
cd librechat
docker compose up

# LibreChat should now be running at:
# http://localhost:3080
```

## Streamable HTTP MCP Server (Everything)

### Setup and running locally

Install & Run MCP Server with Streamable HTTP support for more info see [mcp-server](https://github.com/modelcontextprotocol/servers/tree/main/src/everything#run-from-source-with-streamable-http-transport)

```bash
cd mcp-server
npm install
npm run start:streamableHttp
```

The server should run on:  
`http://localhost:3001/mcp`

## Wasm module & Envoy Proxy

### Setup

Build the wasm module.
This command outputs the wasm binary to the `./target` folder.

```
cargo build --target wasm32-wasip1 --release
```

Start envoy with the wasm binary and envoy config mounted as path volumes.
The envoy config will route traffic from `/mcp` to the mcp server started on port 3001, proxying through the wasm module.

```
docker run --rm -it   -v "$PWD/target/wasm32-wasip1/release/mcp_wasm_filter.wasm:/etc/envoy/mcp_wasm_filter.wasm"   -v "$PWD/envoy/envoy.yaml:/etc/envoy/envoy.yaml"   -p 10000:10000 -p 9901:9901   envoyproxy/envoy-dev:latest
```

### Test it out

Initialise an mcp connection to the server

```
curl -v http://localhost:10000/mcp \
  -H 'Content-Type: application/json' \
  -H 'Accept: application/json, text/event-stream' \
  -d '{
    "jsonrpc": "2.0",
    "id": "1",
    "method": "initialize",
    "params": {"protocolVersion":"2024-11-05","capabilities":{"sampling":{},"roots":{"listChanged":true}},"clientInfo":{"name":"my-client","version":"0.7.2"}}
  }'
```

The response should look something like this:

```
event: message
id: 38134b7b-a51a-4edb-b169-cc741f23df88_1750105674999_8lup021w
data: {"result":{"protocolVersion":"2024-11-05","capabilities":{"prompts":{},"resources":{"subscribe":true},"tools":{},"logging":{},"completions":{}},"serverInfo":{"name":"example-servers/everything","version":"1.0.0"}},"jsonrpc":"2.0","id":"1"}
```

There should also be some logs in the envoy output like this:

```
[2025-06-16 16:36:59.563][38][info][wasm] [source/extensions/common/wasm/context.cc:1137] wasm log mcp_wasm_filter_root my_vm: vm start called

...

[2025-06-16 20:42:19.595][41][info][wasm] [source/extensions/common/wasm/context.cc:1137] wasm log mcp_wasm_filter mcp_wasm_filter_root my_vm: on_http_request_body called
[2025-06-16 20:42:19.595][41][info][wasm] [source/extensions/common/wasm/context.cc:1137] wasm log mcp_wasm_filter mcp_wasm_filter_root my_vm: Appending 229 bytes to buffer
[2025-06-16 20:42:19.595][41][info][wasm] [source/extensions/common/wasm/context.cc:1137] wasm log mcp_wasm_filter mcp_wasm_filter_root my_vm: on_http_request_body called
[2025-06-16 20:42:19.595][41][info][wasm] [source/extensions/common/wasm/context.cc:1137] wasm log mcp_wasm_filter mcp_wasm_filter_root my_vm: Wasm filter: Got request body! body_size: 0
```