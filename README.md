# mcp-envoy-wasm

# Local LibreChat + MCP + http-streamable + Envoy Setup

TODO add intro

## Prerequisites

- Docker/podman
- node.js/npm/npx - https://nodejs.org/en/download
- rust - https://www.rust-lang.org/tools/install

## Streamable HTTP MCP Server (Everything)

**Important:** Run this before librechat as it attempts to conenct to any configured mcp servers at startup

### Setup and running locally

Install & run an MCP Server with Streamable HTTP support.
For more info see [mcp-server](https://github.com/modelcontextprotocol/servers/tree/main/src/everything#run-from-source-with-streamable-http-transport)

```bash
npx @modelcontextprotocol/server-everything streamableHttp
```

### Test it out

```bash
curl http://localhost:3001/mcp
```

The response should look something like this:

```bash
{"jsonrpc":"2.0","error":{"code":-32000,"message":"Bad Request: No valid session ID provided"}}
```

## Wasm module & Envoy Proxy

**Important:** Run this before librechat as it attempts to conenct to any configured mcp servers at startup

### Setup

Build the wasm module.
This command outputs the wasm binary to the `./target` folder.

```bash
cargo build --target wasm32-wasip1 --release
```

Start envoy with the wasm binary and envoy config mounted as path volumes.
The envoy config will route traffic from `/mcp` to the mcp server started on port 3001, proxying through the wasm module.

```bash
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

## LibreChat

Bring it all together in the Librechat UI

### Run LibreChat Locally

Once off config setup:

```bash
cp librechat/.env.template librechat/.env
```

Start the services:

```bash
cd librechat
docker compose up -d
```

LibreChat should now be running at http://localhost:3080
The mcp server should show up in the UI, and allow you to exec the tool using a prompt like `add 7 and 3`.
You'll need to select an AI provider and model from the dropdown at the top first, and set an API Key (unless you have a custom model/endpoint set up in librechat you can use).
For example, you can use a Gemini API Key on the free tier to test this out.
