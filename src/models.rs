use cdd::*;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ParseRequest {
    code: String,
}

#[derive(Deserialize, Debug)]
pub struct UpdateRequest {
    pub code: String,
    pub project: Project,
}
