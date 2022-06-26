#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde;
mod config_parser;

fn main() {
    let path = "config.json";
    config_parser::parse_config(path);
    /*
    Load configuration
    For each endpoint create a queue

    */
}
