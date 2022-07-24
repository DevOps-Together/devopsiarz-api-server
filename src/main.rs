use std::env;
mod config;
use config::{Configuration};
mod queuing_strategy;

const DEFAULT_PATH: &str = "config.json";

fn main() {
    let config: Configuration = parse_config();

    
}

fn parse_config() -> Configuration {
    let args: Vec<String> = env::args().collect();
    let path: &str = args.last().map_or(DEFAULT_PATH, |path| path.as_str());
    return config::parse_config(path);
}