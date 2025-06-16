# mcp-envoy-wasm

# Local LibreChat + MCP + http-streamable + Envoy Setup

TODO add intro

## Prerequisites

- Docker
- npm (Node Package Manager)

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

## Envoy Proxy

### Setup

`

```

```
