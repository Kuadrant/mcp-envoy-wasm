use proxy_wasm::hostcalls::{log, set_property};
use proxy_wasm::traits::*;
use proxy_wasm::types::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Deserialize, Serialize)]
struct Request {
    method: String,
    #[serde(default)]
    params: Option<Value>,
    #[serde(default)]
    id: Option<Value>, // Can be string, number, or null
}

struct MCPRoot {}

impl Context for MCPRoot {}

impl RootContext for MCPRoot {
    fn on_vm_start(&mut self, _vm_configuration_size: usize) -> bool {
        let _ = log(LogLevel::Info, "vm start called");
        true
    }

    fn get_type(&self) -> Option<ContextType> {
        Some(ContextType::HttpContext)
    }

    fn create_http_context(&self, _context_id: u32) -> Option<Box<dyn HttpContext>> {
        Some(Box::new(MCPHttpFilter {
            request_body: Vec::new(),
        }))
    }
}

struct MCPHttpFilter {
    request_body: Vec<u8>,
}

impl Context for MCPHttpFilter {}

impl HttpContext for MCPHttpFilter {
    fn on_http_request_body(&mut self, body_size: usize, end_of_stream: bool) -> Action {
        let _ = log(LogLevel::Info, "on_http_request_body called");

        if let Some(chunk) = self.get_http_request_body(0, body_size) {
            self.request_body.extend_from_slice(&chunk);
        }

        if end_of_stream {
            if let Ok(body_str) = std::str::from_utf8(&self.request_body) {

                let _ = log(LogLevel::Info, &format!("FULL REQUEST BODY: {}", body_str));
                let _ = set_property(vec!["request", "custom", "raw_body"], Some(body_str.as_bytes()));
                match serde_json::from_str::<Request>(body_str) {
                    Ok(req) => {
                        let _ = log(LogLevel::Info, &format!("PARSED REQUEST: METHOD = {}", req.method));
                        let _ = set_property(vec!["request", "custom", "method"], Some(req.method.as_bytes()));

                        // Expose id if present
                        if let Some(id_val) = req.id {
                            if let Ok(id_str) = serde_json::to_string(&id_val) {
                                let _ = log(LogLevel::Info, &format!("REQUEST ID: {}", id_str));
                                let _ = set_property(vec!["request", "custom", "id"], Some(id_str.as_bytes()));
                            }
                        }
                        

                        if let Some(params) = req.params {
                            if let Ok(param_str) = serde_json::to_string(&params) {
                                let _ = log(LogLevel::Info, &format!("PARSED PARAMS: {}", param_str));
                                let _ = set_property(vec!["request", "custom", "params"], Some(param_str.as_bytes()));
                            }

                            // Extract tool name if present
                            if let Some(tool_name) = params.get("name").and_then(|v| v.as_str()) {
                                let _ = log(LogLevel::Info, &format!("TOOL NAME: {}", tool_name));
                                let _ = set_property(vec!["request", "custom", "tool_name"], Some(tool_name.as_bytes()));
                            }
                        }
                    }
                    Err(e) => {
                        let _ = log(LogLevel::Error, &format!("Failed to parse Request JSON: {}", e));
                    }
                }
            } else {
                let _ = log(LogLevel::Error, "Invalid UTF-8 in request body");
            }
        }

        Action::Continue
    }
}

proxy_wasm::main!({
    proxy_wasm::set_root_context(|_| Box::new(MCPRoot {}));
});
