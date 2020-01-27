use super::*;

pub fn extract_models_from_openapi(openapi: &OpenAPI) -> Vec<Model> {
    // note from api documentation:
    // All objects defined within the components object will have no effect on the API
    // unless they are explicitly referenced from properties outside the components object.

    let mut models = Vec::new();

    openapi.components.clone().map(|components|
        // https://docs.rs/openapiv3/0.3.0/openapiv3/struct.Components.html

        components.schemas.into_iter().map(|(component_name, schema)| {
            // IndexMap<String, ReferenceOr<Schema>>
            // represents each model, wrapped in a schema. could be an array type,
            // which doesn't translate to a class model.
            // we are only looking for 'object' types.

            // println!("SCEHMA: {} {:?}", component_name, schema);

            if let Some(object_type) = extract_object_type_from_openapi(schema) {
                models.push(Model {
                    name: component_name,
                    vars: extract_variables_from_openapi(object_type).into_iter().map(|v| Box::new(v)).collect(),
                    // vars: Vec::new(),
                });
            };

        }).collect()
    ).unwrap_or(Vec::new());

    models
}

fn extract_object_type_from_openapi(schema: openapiv3::ReferenceOr<Schema>) -> Option<ObjectType> {
    if let ReferenceOr::Item(schema) = schema {
        if let openapiv3::SchemaKind::Type(type_) = schema.schema_kind.clone() {
            if let Type::Object(object_type) = type_ {
                return Some(object_type);
            }
        }
    }
    None
}

fn extract_variables_from_openapi(object_type: ObjectType) -> Vec<Variable> {
    let mut variables = Vec::new();

    // object_type.properties.into_iter().map(|(name, property)| {
    for (name, property) in object_type.properties {
        // IndexMap<String, ReferenceOr<Schema>>

        if let Some(variable_type) = extract_variable_type_from_openapi(property) {
            variables.push(Variable {
                name,
                optional: false, // write function for this
                value: None,
                variable_type
            });
        }
    };

    variables
}

pub fn extract_variable_type_from_openapi(schema: ReferenceOr<Box<Schema>>) -> Option<VariableType> {
    if let ReferenceOr::Item(schema) = schema {
        if let SchemaKind::Type(variable_type) = schema.schema_kind {
            return Some(convert_openapi_type_to_variable_type(variable_type));
        }
    }
    None
}

pub fn convert_openapi_type_to_variable_type(t: Type) -> VariableType {
    match t {
        Type::String(_) => VariableType::StringType,
        Type::Number(_) => VariableType::FloatType,
        Type::Integer(_) => VariableType::IntType,
        Type::Object(_) => {
            VariableType::ComplexType("Need to implement".to_string())
        } //Need to implement
        Type::Array(val) => {
            // let item_type = extract_variable_type_from_openapi(val.items.clone().unbox());
            // VariableType::ArrayType(Box::new(item_type))

            VariableType::BoolType // fix this
        }
        Type::Boolean {} => VariableType::BoolType,
    }
}