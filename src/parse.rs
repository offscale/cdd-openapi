use cdd::*;
// use jsonrpc_ws_server::jsonrpc_core;
use openapiv3::*;
use url::Url;

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

fn read_file(path: &str) -> Result<String, std::io::Error> {
    use std::fs::File;
    use std::io::prelude::*;

    let mut f = File::open(std::path::PathBuf::from(path))?;
    let mut buffer = String::new();

    f.read_to_string(&mut buffer)?;

    Ok(buffer)
}

pub(crate) fn template(name: &str) -> Result<OpenAPI, failure::Error> {
    let file_content: String = read_file(&format!("templates/{}.yaml", name))?;
    Ok(code_to_openapi(&file_content)?)
}

pub(crate) fn code_to_openapi(code: &str) -> Result<OpenAPI, serde_yaml::Error> {
    serde_yaml::from_str(&code)
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
    let openapi:OpenAPI = code_to_openapi(code)?;
    Ok(extract_project_from_openapi(&openapi))

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

pub fn extract_project_from_openapi(openapi: &OpenAPI) -> Project {
    Project {
        info: extract_info_from_openapi(openapi),
        models: extract_models_from_openapi(openapi),
        requests: extract_requests_from_openapi(openapi),
    }
}

pub fn extract_info_from_openapi(openapi: &OpenAPI) -> cdd::Info {
    let server = extract_server_from_openapi(openapi);

    let url = Url::parse(server.as_str());
    let host:String = url.clone().map(|url| format!("{}://{}", url.scheme(), url.host_str().unwrap_or(""))).unwrap_or(String::new());
    let endpoint:String = url.map(|url| url.path().to_string()).unwrap_or(String::new());

    cdd::Info {
        host,
        endpoint,
    }
}

pub fn extract_server_from_openapi(openapi: &OpenAPI) -> String {
        openapi
            .servers
            .first()
            .map(|s| s.url.clone())
            .unwrap_or_else(|| "".to_string())
}

pub fn extract_models_from_openapi(openapi: &OpenAPI) -> Vec<Model> {
    Vec::new()
}

pub fn extract_requests_from_openapi(openapi: &OpenAPI) -> Vec<Request> {
    Vec::new()
}
