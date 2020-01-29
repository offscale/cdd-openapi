
use openapiv3::*;
use url::Url;







fn component_is_valid_object(schema: ReferenceOr<Schema>) -> bool {
    if let ReferenceOr::Item(schema) = schema {
        if let openapiv3::SchemaKind::Type(type_) = schema.schema_kind.clone() {
            if let Type::Object(object_type) = type_ {
                return true;
            }
        }
    }

    false
}























fn parse_type(reference: ReferenceOr<openapiv3::Schema>) -> VariableType {
    match reference {
        ReferenceOr::Reference { reference } => {
            VariableType::ComplexType(reference.split('/').last().unwrap_or("").to_string())
        }
        ReferenceOr::Item(schema) => {
            match &schema.schema_kind {
                openapiv3::SchemaKind::Type(t) => {
                    match t {
                        Type::String(_) => VariableType::StringType,
                        Type::Number(_) => VariableType::FloatType,
                        Type::Integer(_) => VariableType::IntType,
                        Type::Object(_) => {
                            VariableType::ComplexType("Need to implement".to_string())
                        } //Need to implement
                        Type::Array(val) => {
                            let item_type = parse_type(val.items.clone().unbox());
                            VariableType::ArrayType(Box::new(item_type))
                        }
                        Type::Boolean {} => VariableType::BoolType,
                    }
                }
                _ => VariableType::StringType,
            }
        }
    }
}

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


// pub fn extract_variable_from_openapi(name: &str, schema: &ReferenceOr<Schema>) -> Option<Variable> {
//     // let mut variables = Vec::new();

//     match schema {
//         ReferenceOr::Item(schema) => {
//             let optional = &schema.schema_data.nullable;

//             // https://docs.rs/openapiv3/0.3.0/openapiv3/enum.SchemaKind.html
//             let variable_type = match &schema.schema_kind {
//                 // regular types, this is the type we need.
//                 SchemaKind::Type(t) => Some(extract_type_from_openapi(t.clone())),

//                 // unsupported kinds (should return an error, fix this)
//                 SchemaKind::Any(_) | SchemaKind::OneOf{one_of: _} | SchemaKind::AllOf{all_of: _} | SchemaKind::AnyOf{any_of: _} => None,
//             };

//             let variable_name = match &schema.schema_data {

//             }

//             // if let openapiv3::SchemaKind::Type(type_) = schema.schema_kind.clone() {
//             //     if let Type::Array(array_type) = type_ {
//             //         let item_type = Project::parse_type(array_type.items.unbox());
//             //         if let VariableType::ComplexType(reference) = item_type {
//             //             arr_types.insert(name.clone(), reference);
//             //             is_array_type = true
//             //         }
//             //     };
//             // } else {
//             //     // array
//             //     Vec::new()
//             // }
//         },
//         _ => ()
//     }

//     Vec::new()


//     // schema.map(|(name, schema)| {
//     //     match schema {
//     //         Variable {
//     //             name,
//     //             type: parse_type(schema)
//     //         }
//     //     }
//     // })
// }

// let components = openapi.components.clone().unwrap(); // map this
    // let mut models:Vec<Model> = Vec::new();

    // // let mut arr_types = HashMap::new();
            
    // for (name, schema) in components.schemas {
    //     if let ReferenceOr::Item(schema) = schema {
    //         // let mut is_array_type = false;

    //         if let openapiv3::SchemaKind::Type(type_) = schema.schema_kind.clone() {
    //             if let Type::Array(array_type) = type_ {
    //                 let item_type = parse_type(array_type.items.unbox());
    //                 if let VariableType::ComplexType(reference) = item_type {
    //                     // models.insert(name.clone(), reference);
    //                     models.push(Model {
    //                         name,
    //                         vars: vec![],
    //                     });
    //                     // is_array_type = true
    //                 }
    //             };
    //         }

    //         // if !is_array_type {
    //         //     let model = Project::parse_model(name, schema)?;
    //         //     project.models.push(model);
    //         // }
    //     }
    // }
    // models


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

// pub fn extract_variable_from_openapi(variable: Type) -> Variable {
//     match variable {
//         Type::Array(t) => 
//     }
// }

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