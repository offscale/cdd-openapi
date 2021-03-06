use crate::models::*;
use jsonrpc_core::types::error::Error;
use jsonrpc_ws_server::jsonrpc_core::*;
use jsonrpc_ws_server::*;
use serde_json::value::Value;

fn rpc_error(message: &str) -> jsonrpc_core::types::error::Error {
    jsonrpc_core::types::error::Error {
        code: jsonrpc_core::types::error::ErrorCode::InternalError,
        message: message.into(),
        data: None,
    }
}

/// ensure filename has no directories / paths
fn sanitise_filename(filename: &str) -> Option<String> {
    std::path::Path::new(filename)
        .file_name()
        .and_then(|filename| filename.to_str().map(|filename| filename.to_string()))
}

pub fn start(hostname: &str) {
    let mut io = IoHandler::new();

    // all exposed rpc methods:
    io.add_method("update", update);        // update existing openapi code from an adt
    io.add_method("template", template);    // return a pre-made openapi template
    io.add_method("default", default);      // get a default template
    io.add_method("parse", parse);          // parse openapi yaml into an adt
    io.add_method("serialise", serialise);  // parse openapi yaml into a json structure
    io.add_method("deserialise", deserialise);

    let server = ServerBuilder::new(io)
        .start(&"0.0.0.0:7777".parse().unwrap()) // todo: custom ports
        .expect("Server must start with no issues");

    server.wait().unwrap()
}

fn template(params: jsonrpc_core::Params) -> std::result::Result<Value, Error> {
    log(format!("-> template: {:?}", params));

    let params: std::collections::HashMap<String, String> = params.parse()?;

    let template_name = params
        .get("name")
        .ok_or(rpc_error("missing parameter: name"))?;

    let sanitised_filename =
        sanitise_filename(template_name).ok_or(rpc_error("invalid parameter: name"))?;

    let template = crate::template::fetch_template_to_openapi(&sanitised_filename)
        .map_err(|e| rpc_error(&format!("{}", e)))?;

    return serde_yaml::to_string(&template)
        .map(|code| serde_json::json!({ "code": code }))
        .map_err(|e| rpc_error(&format!("{}", e)));
}

/// update a code block with directives from an adt structure
fn update(params: jsonrpc_core::Params) -> std::result::Result<Value, Error> {
    log(format!("-> update: {:?}", params));

    let params: UpdateRequest = params.parse()?;

    return crate::generator::update(params.project, &params.code)
        .map(|code| serde_json::json!({ "code": code }))
        .map_err(|e| rpc_error(&format!("{}", e)));
}

fn default(params: jsonrpc_core::Params) -> std::result::Result<Value, Error> {
    log(format!("-> default: {:?}", params));

    let default = crate::fixtures::petstore();

    serde_yaml::to_string(&default)
        .map(|code| serde_json::json!({ "code": code }))
        .map_err(|e| rpc_error(&format!("{}", e)))
}

#[derive(serde::Deserialize, Debug)]
pub struct SerialiseRequest {
    code: String,
}

// yaml string to serialised json
fn serialise(params: jsonrpc_core::Params) -> std::result::Result<Value, Error> {
    log(format!("-> serialise: {:?}", params));
    let params: SerialiseRequest = params.parse()?;

    let response = serde_yaml::from_str(&format!("{}", params.code))
        .map(|openapi:serde_yaml::Value| serde_json::json!({ "ast": openapi }))
        .map_err(|e| rpc_error(&format!("{:?}", e)));
    
    log(format!("<- serialise: {:?}", response));
    response
}

// json serialised structure to yaml string
fn deserialise(params: jsonrpc_core::Params) -> std::result::Result<Value, Error> {
    log(format!("-> deserialise: {:?}", params));
    let params: DeserialiseRequest = params.parse()?;

    let response = crate::parser::parse_json_to_yaml(&params.ast)
        .map(|openapi| serde_json::json!({ "output": openapi }))
        .map_err(|e| rpc_error(&format!("{}", e)));

    log(format!("<- deserialise: {:?}", response));
    response
}

fn parse(params: jsonrpc_core::Params) -> std::result::Result<Value, Error> {
    log(format!("-> parse: {:?}", params));

    #[derive(serde::Deserialize, Debug)]
    pub struct CodeRequest {
        code: String,
    }

    let request: CodeRequest = params.parse().map_err(|e| rpc_error(&format!("{}", e)))?;

    crate::parser::parse_yaml_to_project(&request.code)
        .map(|project: cdd::Project| serde_json::json!({"project": project, "code": ""}))
        .map_err(|e| rpc_error(&format!("{}", e)))
}

fn log(msg: String) {
    println!("{}", crate::util::truncate(msg, 128));
}
