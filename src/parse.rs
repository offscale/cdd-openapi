use openapiv3::*;
use jsonrpc_ws_server::jsonrpc_core;
use cdd::*;

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

// #[derive(serde::Serialize, serde::Deserialize, Debug)]
// pub struct Model {
//     name: String,
//     vars: Vec<crate::variable::Variable>,
// }

// fn extract_variables(schema: openapiv3::ReferenceOr<Schema>) -> Vec<crate::variable::Variable> {
//     if let ReferenceOr::Item(schema) = schema {
//         let mut is_array_type = false;
//         if let openapiv3::SchemaKind::Type(type_) = schema.schema_kind.clone() {
//             if let Type::Array(array_type) = type_ {
//                 let item_type = Project::parse_type(array_type.items.unbox());
//                 if let VariableType::ComplexType(reference) = item_type {
//                     arr_types.insert(name.clone(), reference);
//                     is_array_type = true
//                 }
//             };
//         }
//         if !is_array_type {
//             let model = Project::parse_model(name, schema)?;
//             project.models.push(model);
//         }
//     }

//     Vec::new()
// }

pub(crate) fn extract_project(code: &str) -> Result<Project, failure::Error> {
    let openapi = code_to_openapi(code).expect("code to parse - fix this");
    Project::parse_yml(openapi)

    // let models:Vec<Model> = openapi.components.map(|components|
    //     components.schemas.into_iter().map(|(component_name, schema)| {
    //         // let vars = schema.map(|schema| {
    //         // });
    //         Model {
    //             name: component_name,
    //             vars: extract_variables(schema),
    //         }
    //     }).collect()
    // ).unwrap_or(Vec::new());

    // models
}