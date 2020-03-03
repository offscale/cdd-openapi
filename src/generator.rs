use cdd::*;

pub(crate) fn update(project: Project, code: &str) -> Result<String, failure::Error> {
    // convert code to openapi object
    let openapi = crate::parser::parse_yaml_to_openapi(code)?;
    // traverse models in project, check they exist nondestructively.
    // let comparison_result = crate::comparison::compare_project_to_openapi(project, openapi);
    // traverse routes in project
    Ok(code.into())
}
