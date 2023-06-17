use std::any;
use std::collections::VecDeque;

use crate::config::{EndpointType, EndpointSpec};
pub trait AddressableQueing {
    fn put(&self, data : String);
    fn take(&self) -> String;
    fn get_address(&self) -> String;
}

mod fifo;
use fifo::Fifo;
mod filo;
use filo::Filo;
mod constant;
use constant::Constant;
mod repeat_last;
use repeat_last::RepeatLast;

pub fn create_endpoint(endpoint: &EndpointSpec) -> Box<dyn AddressableQueing> {
    match endpoint.variant {
        EndpointType::Constant => {
            return Box::new(Constant{
                address: endpoint.path,
                value: endpoint.const_response.expect(&format!("Missing const_response in endpoint {}", endpoint.path))
            })
        },
        EndpointType::Fifo => {
            return Box::new(Fifo{
                address: endpoint.path,
                value: VecDeque::new()
            })
        },
        EndpointType::Filo => {
            return Box::new(Filo{
                address: endpoint.path,
                value: Vec::new()
            })
        },
        EndpointType::RepeatLast => {
            return Box::new(RepeatLast{
                address: endpoint.path,
                value: String::from("")
            })
        },
    }
}
