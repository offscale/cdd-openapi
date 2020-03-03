use crate::util::*;
use openapiv3::*;

pub(crate) fn fetch_template_to_openapi(name: &str) -> Result<OpenAPI, failure::Error> {
    let file_content: String = read_file(&format!("templates/{}.yaml", name))?;
    Ok(crate::parser::parse_yaml_to_openapi(&file_content)?)
}
