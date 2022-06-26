use std::fs;
use serde::{Deserialize};

#[derive(Deserialize)]
pub enum EndpointType {
    FIFO,
    FILO,
    CONST
}

#[derive(Deserialize)]
pub struct EndpointSpec {
    path: String,
    variant: EndpointType,
    authentication_enabled: bool,
    username: Option<String>,
    password: Option<String>,
    const_response: Option<String>,
}

#[derive(Deserialize)]
pub struct Configuration {
    address: String,
    port: i32,
    endpoints: Vec<EndpointSpec>
}

pub fn parse_config(path: &str) -> Configuration {
    let contents: String = fs::read_to_string(path)
        .expect("Something went wrong reading the configuration file");
    let config: Configuration = serde_json::from_str(&contents).unwrap();
    return config;
}