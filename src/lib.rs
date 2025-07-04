use proxy_wasm::hostcalls::log;
use proxy_wasm::traits::*;
use proxy_wasm::types::*;

struct MinimalRoot;

impl Context for MinimalRoot {}

impl RootContext for MinimalRoot {
    fn on_vm_start(&mut self, _vm_configuration_size: usize) -> bool {
        let _ = log(LogLevel::Info, "Minimal WASM filter started");
        true
    }

    fn get_type(&self) -> Option<ContextType> {
        Some(ContextType::HttpContext)
    }

    fn create_http_context(&self, _context_id: u32) -> Option<Box<dyn HttpContext>> {
        Some(Box::new(MinimalHttpFilter {
            response_sent: false,
            request_body: Vec::new(),
        }))
    }
}

struct MinimalHttpFilter {
    response_sent: bool,
    request_body: Vec<u8>,
}

impl Context for MinimalHttpFilter {}

impl HttpContext for MinimalHttpFilter {
    fn on_http_request_headers(&mut self, _num_headers: usize, _end_of_stream: bool) -> Action {
        let _ = log(LogLevel::Info, "=== REQUEST HEADERS - CONTINUING IMMEDIATELY ===");
        Action::Continue
    }

    fn on_http_request_body(&mut self, body_size: usize, end_of_stream: bool) -> Action {
        let _ = log(LogLevel::Info, &format!("=== REQUEST BODY - body_size: {}, end_of_stream: {} ===", body_size, end_of_stream));
        
        // Buffer the request body
        if let Some(chunk) = self.get_http_request_body(0, body_size) {
            let _ = log(LogLevel::Info, &format!("Appending {} bytes to buffer", chunk.len()));
            self.request_body.extend_from_slice(&chunk);
        }
        
        // Process complete request when we have all the data
        if end_of_stream && !self.response_sent {
            let _ = log(LogLevel::Info, "Processing complete request body");
            
            // Convert body to string
            if let Ok(body_str) = std::str::from_utf8(&self.request_body) {
                let _ = log(LogLevel::Info, &format!("Request body: {}", body_str));
                
                // Check if this is an initialize request
                if body_str.contains("\"method\":\"initialize\"") || body_str.contains("\"method\": \"initialize\"") {
                    let _ = log(LogLevel::Info, "Detected MCP initialize request - sending hardcoded SSE response");
                    
                    let json_response = r#"{"result":{"protocolVersion":"2024-11-05","capabilities":{"prompts":{},"resources":{"subscribe":true},"tools":{},"logging":{},"completions":{}},"serverInfo":{"name":"example-servers/everything","version":"1.0.0"}},"jsonrpc":"2.0","id":"1"}"#;
                    
                    // Format as SSE
                    let sse_response = format!("event: message\nid: hardcoded-session-12345_1\ndata: {}\n\n", json_response);
                    
                    self.send_http_response(
                        200,
                        vec![
                            ("content-type", "text/event-stream"),
                            ("cache-control", "no-cache"),
                            ("connection", "keep-alive"),
                            ("mcp-session-id", "hardcoded-session-12345"),
                        ],
                        Some(sse_response.as_bytes()),
                    );
                    
                    self.response_sent = true;
                    let _ = log(LogLevel::Info, "Hardcoded initialize SSE response sent - continuing to see if connection stays alive");
                    return Action::Pause;
                } else {
                    let _ = log(LogLevel::Info, "Not an initialize request - continuing to upstream");
                }
            } else {
                let _ = log(LogLevel::Warn, "Failed to parse request body as UTF-8");
            }
        }
        
        Action::Continue
    }

    fn on_http_response_headers(&mut self, _num_headers: usize, _end_of_stream: bool) -> Action {
        let _ = log(LogLevel::Info, "=== RESPONSE HEADERS - CONTINUING IMMEDIATELY ===");
        Action::Continue
    }
    
    fn on_http_response_body(&mut self, body_size: usize, end_of_stream: bool) -> Action {
        let _ = log(LogLevel::Info, &format!("=== RESPONSE BODY - body_size: {}, end_of_stream: {} ===", body_size, end_of_stream));
        Action::Continue
    }
}

proxy_wasm::main!({
    proxy_wasm::set_root_context(|_| {
        Box::new(MinimalRoot)
    });
}); 