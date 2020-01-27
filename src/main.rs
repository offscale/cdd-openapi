mod rpc;
mod fixtures;
mod parse;
mod variable;
mod models;
mod generator;
mod extractor;
mod template;
mod util;
mod parser;

fn main() {
    rpc::start_server();
}
