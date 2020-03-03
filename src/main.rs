mod extractor;
mod fixtures;
mod generator;
mod models;
mod parser;
mod rpc;
mod template;
mod util;
mod variable;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server = "127.0.0.1:7777";
    println!("Starting server on {}...", server);
    rpc::start(server);
    Ok(())
}
