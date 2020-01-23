mod rpc;
mod fixtures;
mod parse;
mod variable;
mod models;
mod generate;

fn main() {
    rpc::start_server();
}
