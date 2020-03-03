use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct Variable {
    pub name: String,
    #[serde(rename = "type")]
    pub variable_type: VariableType,
    pub optional: bool,
    pub value: Option<String>,
}

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub enum VariableType {
    #[serde(rename = "String")]
    StringType,
    #[serde(rename = "Int")]
    IntType,
    #[serde(rename = "Bool")]
    BoolType,
    #[serde(rename = "Float")]
    FloatType,
    #[serde(rename = "Array")]
    ArrayType(Box<VariableType>),
    #[serde(rename = "Complex")]
    ComplexType(String),
}

pub(crate) fn extract_variable_from_openapi(class_name: &str, var_name: &str, schema: openapiv3::Schema, optional: bool) -> Option<Variable> {
    if let openapiv3::SchemaKind::Type(schema_type) = schema.schema_kind {
        let variable_type = match schema_type {
            openapiv3::Type::String(_) => VariableType::StringType,
            openapiv3::Type::Number(_) => VariableType::FloatType,
            openapiv3::Type::Integer(_) => VariableType::IntType,
            openapiv3::Type::Array(val) => {
                // let item_type = Project::parse_type(val.items.clone().unbox());
                // VariableType::ArrayType(Box::new(item_type))
                return None; // fix
            }
            openapiv3::Type::Boolean {} => VariableType::BoolType,
            _ => {
            return None
            }
        };

        Some(Variable {
            name: var_name.to_string(),
            optional,
            value: None,
            variable_type: variable_type
        })
    } else {
        return None
    }
}
