

#[derive(serde::Deserialize, Debug)]
pub struct ParseRequest {
    code: String,
}

#[derive(serde::Deserialize, Debug)]
pub struct UpdateRequest {
    pub code: String,
    pub project: Project,
}

#[derive(serde::Deserialize, Debug)]
pub struct Project {
    models: Vec<Model>,
    requests: Vec<Request>,
}

#[derive(serde::Deserialize, Debug)]
pub struct Model {
    name: String,
    vars: Vec<Variable>,
}

#[derive(serde::Deserialize, Debug)]
pub struct Variable {
    name: String,
    ty: String,
}

#[derive(serde::Deserialize, Debug)]
pub struct Request {
}