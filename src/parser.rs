use cdd::*;
use openapiv3::*;

pub(crate) fn parse_yaml_to_openapi(code: &str) -> Result<OpenAPI, serde_yaml::Error> {
    serde_yaml::from_str(&code)
}

pub(crate) fn parse_yaml_to_project(code: &str) -> Result<Project, failure::Error> {
    let openapi: OpenAPI = parse_yaml_to_openapi(code)?;
    Ok(crate::extractor::extract_project_from_openapi(&openapi))
}
