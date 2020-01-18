use jsonrpc_ws_server::*;
use jsonrpc_ws_server::jsonrpc_core::*;

fn rpc_error(message: &str) -> jsonrpc_core::types::error::Error {
    jsonrpc_core::types::error::Error{
        code: jsonrpc_core::types::error::ErrorCode::InternalError,
        message: message.into(),
        data: None,
    }
}

pub fn start_server() {
    let mut io = IoHandler::new();

    // Returns a code example.
    io.add_method("default", |params| {
        println!("-> default: {:?}", params);

        let default = crate::fixtures::petstore();

        serde_yaml::to_string(&default)
            .map(|code| serde_json::json!({"code": code}))
            .map_err(|e| rpc_error(&format!("error: {:?}", e)))
    });
    
    // Returns any models or routes found in a given code block.
    io.add_method("parse", |params: jsonrpc_core::Params| {
        println!("-> parse: {:?}", params);

        #[derive(serde::Deserialize, Debug)]
        pub struct CodeRequest {
            code: String,
        }

        let request: CodeRequest = params.parse().map_err(|e| rpc_error(&format!("error: {:?}", e)))?;

        crate::parse::extract_project(&request.code)
            .map(|response| serde_json::json!(response))
            .map_err(|e| rpc_error(&format!("error: {:?}", e)))
    });

	let server = ServerBuilder::new(io)
		.start(&"0.0.0.0:7777".parse().unwrap())
		.expect("Server must start with no issues");

	server.wait().unwrap()
}
