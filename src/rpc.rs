use jsonrpc_ws_server::*;
use jsonrpc_ws_server::jsonrpc_core::*;

fn rpc_error(message: &str) -> jsonrpc_core::types::error::Error {
    jsonrpc_core::types::error::Error{
        code: jsonrpc_core::types::error::ErrorCode::InternalError,
        message: message.into(),
        data: None,
    }
}

/// ensure filename has no directories / paths
fn sanitise_filename(filename: &str) -> Option<String> {
    std::path::Path::new(filename).file_name()
        .and_then(|filename| filename.to_str().map(|filename| filename.to_string()))
}

pub fn start_server() {
    let mut io = IoHandler::new();

    // all exposed rpc methods:
    io.add_method("generate", generate);    // generate new openapi code
    io.add_method("template", template);    // return a pre-made openapi template
    io.add_method("default", default);      // get a default template
    io.add_method("parse", parse);          // parse openapi yaml into an adt

	let server = ServerBuilder::new(io)
		.start(&"0.0.0.0:7777".parse().unwrap())    // todo: custom ports
		.expect("Server must start with no issues");

	server.wait().unwrap()
}

use serde_json::value::Value;
use jsonrpc_core::types::error::Error;

fn template(params: jsonrpc_core::Params) -> std::result::Result<Value, Error> {
    println!("-> template: {:?}", params);

    let params:std::collections::HashMap<String, String> = params.parse()?;

    // note that name is unclean, we need to sanitise this! or it's a vuln
    if let Some(template_file) = params.get("name") {
        if let Some(filename) = sanitise_filename(template_file) {
            let template = crate::parse::template(&filename)
                .map_err(|e| rpc_error(&format!("{}", e)))?;

            return serde_yaml::to_string(&template)
                .map(|code| serde_json::json!({"code": code}))
                .map_err(|e| rpc_error(&format!("{}", e)));
        }
    };

    Err(rpc_error(&format!("invalid template name")))
}

fn generate(params: jsonrpc_core::Params) -> std::result::Result<Value, Error> {
    println!("-> template: {:?}", params);

    let params:std::collections::HashMap<String, String> = params.parse()?;

    // note that name is unclean, we need to sanitise this! or it's a vuln
    if let Some(template_file) = params.get("name") {
        if let Some(filename) = sanitise_filename(template_file) {
            let template = crate::parse::template(&filename)
                .map_err(|e| rpc_error(&format!("{}", e)))?;

            return serde_yaml::to_string(&template)
                .map(|code| serde_json::json!({"code": code}))
                .map_err(|e| rpc_error(&format!("{}", e)));
        }
    };

    Err(rpc_error(&format!("invalid template name")))
}

fn default(params: jsonrpc_core::Params) -> std::result::Result<Value, Error> {
    println!("-> default: {:?}", params);

    let default = crate::fixtures::petstore();

    serde_yaml::to_string(&default)
        .map(|code| serde_json::json!({"code": code}))
        .map_err(|e| rpc_error(&format!("{}", e)))
}

fn parse(params: jsonrpc_core::Params) -> std::result::Result<Value, Error> {
    println!("-> parse: {:?}", params);

    #[derive(serde::Deserialize, Debug)]
    pub struct CodeRequest {
        code: String,
    }

    let request: CodeRequest = params.parse().map_err(|e| rpc_error(&format!("{}", e)))?;

    crate::parse::extract_project(&request.code)
        .map(|response| serde_json::json!(response))
        .map_err(|e| rpc_error(&format!("{}", e)))
}