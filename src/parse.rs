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

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Model {
    name: String,
    vars: Vec<String>,
}

pub(crate) fn extract_models(code: &str) -> Vec<Model> {
    let openapi = code_to_openapi(code).expect("code to parse - fix this");

    let models:Vec<Model> = openapi.components.map(|components|
        components.schemas.into_iter().map(|(component_name, schema)| {
            let vars = schema.map(|schema| {

            });
            Model {
                name: component_name,
                vars: Vec::new(),
            }
        }).collect()
    ).unwrap_or(Vec::new());

    models
}