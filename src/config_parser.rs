use std::{fs, net::IpAddr};
use serde::{Deserialize};

#[derive(Deserialize, PartialEq, Eq)]
pub enum EndpointType {
    FIFO,
    FILO,
    CONST,
    REPEAT_LAST
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
    port: u16,
    endpoints: Vec<EndpointSpec>
}

fn verify_endpoint(endpoint: &EndpointSpec) {
    if endpoint.authentication_enabled {
       endpoint.username.as_ref().expect(&format!("Endpoint {} must have non-empty username!", endpoint.path));
       endpoint.password.as_ref().expect(&format!("Endpoint {} must have non-empty password!", endpoint.path));
    }
    if endpoint.variant == EndpointType::CONST {
        endpoint.const_response.as_ref().expect(&format!("Endpoint {} must have non-empty const_response!", endpoint.path));
    }
}

fn verify_config(config: &Configuration) {
    config.endpoints.iter().for_each(|endpoint| verify_endpoint(endpoint));
    config.address.parse::<IpAddr>().expect("Not a valid ip address");
}

pub fn parse_config(path: &str) -> Configuration {
    let contents: String = fs::read_to_string(path)
        .expect("Something went wrong reading the configuration file");
    let config: Configuration = serde_json::from_str(&contents).unwrap();
    verify_config(&config);
    return config;
}
