use openapiv3::*;
use cdd::*;

mod models;
mod requests;

pub fn extract_project_from_openapi(openapi: &OpenAPI) -> Project {
    Project {
        info: extract_info_from_openapi(openapi),
        models: models::extract_models_from_openapi(openapi),
        requests: requests::extract_requests_from_openapi(openapi),
    }
}

pub fn extract_info_from_openapi(openapi: &OpenAPI) -> Option<cdd::Info> {
    use url::Url;

    let server = extract_server_from_openapi(openapi);
    let url = Url::parse(server.as_str());
    let host:String = url.clone().map(|url| format!("{}://{}", url.scheme(), url.host_str().unwrap_or(""))).unwrap_or(String::new());
    let endpoint:String = url.map(|url| url.path().to_string()).unwrap_or(String::new());

    Some(cdd::Info {
        host,
        endpoint,
    })
}

pub fn extract_server_from_openapi(openapi: &OpenAPI) -> String {
    openapi
        .servers
        .first()
        .map(|s| s.url.clone())
        .unwrap_or_else(|| "".to_string())
}