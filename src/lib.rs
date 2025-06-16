use proxy_wasm::hostcalls::{log};
use proxy_wasm::traits::*;
use proxy_wasm::types::*;
use serde::Deserialize;
// use serde_json::Value;

#[derive(Deserialize, Debug)] // Derive Debug

struct MCPRoot {
}

impl Context for MCPRoot {}

impl RootContext for MCPRoot {
    // Called when VM starts
    fn on_vm_start(&mut self, _vm_configuration_size: usize) -> bool {
        let _ = log(LogLevel::Info, "vm start called");
        true
    }

    // **Tell Envoy this is an HTTP context**
    fn get_type(&self) -> Option<ContextType> {
        Some(ContextType::HttpContext)
    }

    // Called to create an HttpContext for each HTTP stream
    fn create_http_context(&self, _context_id: u32) -> Option<Box<dyn HttpContext>> {
        Some(Box::new(MCPHttpFilter {
            request_body: Vec::new(),
        }))
    }
}

// Per-request context
struct MCPHttpFilter {
    request_body: Vec<u8>,
}

impl Context for MCPHttpFilter {}

impl HttpContext for MCPHttpFilter {
    fn on_http_request_body(&mut self, body_size: usize, end_of_stream: bool) -> Action {
        let _ = log(LogLevel::Info, "on_http_request_body called");
        if let Some(chunk) = self.get_http_request_body(0, body_size) {
            let _ = log(
                LogLevel::Info,
                &format!("Appending {} bytes to buffer", &chunk.len()),
            );
            self.request_body.extend_from_slice(&chunk);
        }
        if end_of_stream {
            let _ = log(
                LogLevel::Info,
                &format!("Wasm filter: Got request body! body_size: {:?}", body_size),
            );
        }
        Action::Continue
    }
}

proxy_wasm::main!({
    proxy_wasm::set_root_context(|_| {
        Box::new(MCPRoot {
        })
    });
});
