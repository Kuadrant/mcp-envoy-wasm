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
        }))
    }
}

struct MinimalHttpFilter {
    response_sent: bool,
}

impl Context for MinimalHttpFilter {}

impl HttpContext for MinimalHttpFilter {
    fn on_http_request_headers(&mut self, _num_headers: usize, _end_of_stream: bool) -> Action {
        let _ = log(LogLevel::Info, "=== REQUEST HEADERS - CONTINUING IMMEDIATELY ===");
        Action::Continue
    }

    fn on_http_request_body(&mut self, body_size: usize, end_of_stream: bool) -> Action {
        let _ = log(LogLevel::Info, &format!("=== REQUEST BODY - body_size: {}, end_of_stream: {} ===", body_size, end_of_stream));
        
        if !self.response_sent {
            let _ = log(LogLevel::Info, "Sending hardcoded response");
            
            let response_body = r#"[{"id":1,"jsonrpc":"2.0","result":{"protocolVersion":"2025-03-26","capabilities":{"tools":{"enabled":true}},"serverInfo":{"name":"minimal-test","version":"1.0.0"}}}]"#;
            
            self.send_http_response(
                200,
                vec![
                    ("content-type", "application/json"),
                    ("mcp-session-id", "test-session-123"),
                ],
                Some(response_body.as_bytes()),
            );
            
            self.response_sent = true;
            let _ = log(LogLevel::Info, "Response sent, pausing");
        }
        
        Action::Pause
    }
}

proxy_wasm::main!({
    proxy_wasm::set_root_context(|_| {
        Box::new(MinimalRoot)
    });
}); 