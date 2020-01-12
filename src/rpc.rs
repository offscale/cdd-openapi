use openapiv3 as openapi;
use jsonrpc_ws_server::*;
use jsonrpc_ws_server::jsonrpc_core::*;

pub fn start_server() {
    let mut io = IoHandler::new();

    io.add_method("default", |_params| {
        let default = crate::petstore::petstore();

        serde_yaml::to_string(&default)
        .map(|code|
            serde_json::json!([{"code": code}])
        )
        .map_err(|e| {jsonrpc_core::types::error::Error{
            code: jsonrpc_core::types::error::ErrorCode::InternalError,
            message: format!("error: {:?}", e),
            data: None,
        }})
    });
    
	io.add_method("get-projects", |_params| {
		Ok(Value::String("hello".into()))
    });
    
    io.add_method("parse-file", |params| {
       Ok(Value::String(
           format!("{:?}", params)
       )) 
    });

	let server = ServerBuilder::new(io)
		.start(&"0.0.0.0:7777".parse().unwrap())
		.expect("Server must start with no issues");

	server.wait().unwrap()
}
