use cdd::*;
use openapiv3::*;

pub(crate) fn parse_yaml_to_openapi(yaml: &str) -> Result<OpenAPI, serde_yaml::Error> {
    println!("parse_yaml_to_openapi--> {:?}", yaml);
    serde_yaml::from_str(&yaml)
}

pub(crate) fn parse_json_to_yaml(json: &serde_json::Value) -> Result<String, serde_yaml::Error> {
    println!("parse_json_to_yaml--> {:?}", json);
    serde_yaml::to_string(&json)
}

pub(crate) fn parse_yaml_to_project(code: &str) -> Result<Project, failure::Error> {
    let openapi: OpenAPI = parse_yaml_to_openapi(code)?;
    Ok(crate::extractor::extract_project_from_openapi(&openapi))
}
