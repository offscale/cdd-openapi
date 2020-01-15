use openapiv3::*;
use jsonrpc_ws_server::jsonrpc_core;
// use cdd::*;

// fn code_to_project(code: &str) -> Result<Project, jsonrpc_core::types::error::Error> {
//     let openapi = code_to_openapi(&code);

// }

// fn openapi_to_json(openapi: OpenAPI) -> Result<OpenAPI, jsonrpc_core::types::error::Error> {
//         serde_yaml::to_string(&openapi)
//     .map_err(|e| {jsonrpc_core::types::error::Error{
//         code: jsonrpc_core::types::error::ErrorCode::InternalError,
//         message: format!("error: {:?}", e),
//         data: None,
//     }})
// }

pub(crate) fn code_to_openapi(code: &str) -> Result<OpenAPI, jsonrpc_core::types::error::Error> {
    serde_yaml::from_str(&code)
    .map_err(|e| {jsonrpc_core::types::error::Error{
        code: jsonrpc_core::types::error::ErrorCode::InternalError,
        message: format!("error: {:?}", e),
        data: None,
    }})
}

pub(crate) fn code_to_models(code: &str) -> serde_json::value::Value {
    let openapi = code_to_openapi(code).expect("code to parse - fix this");

    let models: Vec<String> = openapi.components.map(|components|
        components.schemas.into_iter().map(|(component_name, _schema)| {
            // println!("SCHEMA: {:?}", component_name);
            component_name
        }).collect()

        // for (component_name, _schema) in components.schemas {
        //     println!("SCHEMA: {:?}", component_name);
        // }
    ).unwrap_or(Vec::new());

    println!("MODELS: {:?}", models);

    serde_json::json!({"models": models})
}