use jsonrpc_ws_server::*;
use jsonrpc_ws_server::jsonrpc_core::*;

pub fn start_server() {
    let mut io = IoHandler::new();

    // Returns a code example.
    io.add_method("default", |_params| {
        let default = crate::fixtures::petstore();

        serde_yaml::to_string(&default)
        .map(|code|
            serde_json::json!({"code": code})
        )
        .map_err(|e| {jsonrpc_core::types::error::Error{
            code: jsonrpc_core::types::error::ErrorCode::InternalError,
            message: format!("error: {:?}", e),
            data: None,
        }})
    });
    
    // Returns any models or routes found in a given code block.
    io.add_method("parse", |params| {

        println!("parse -> params: {:?}", params);

       Ok(Value::String(
           format!("{:?}", params)
       )) 
    });

	let server = ServerBuilder::new(io)
		.start(&"0.0.0.0:7777".parse().unwrap())
		.expect("Server must start with no issues");

	server.wait().unwrap()
}
