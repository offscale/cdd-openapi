mod extractor;
mod fixtures;
mod generator;
mod models;
mod parser;
mod rpc;
mod template;
mod util;
mod variable;

fn main() {
    rpc::start_server();
}
