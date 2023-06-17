use std::collections::VecDeque;
use crate::queuing_strategy::{AddressableQueing};

pub struct Fifo {
    address: String,
    value: VecDeque<String>,
}

impl AddressableQueing for Fifo {
    fn get_address(&self) -> String {
        return self.address;
    }
    fn put(&self, data : String) {
        self.value.push_back(data);
    }

    fn take(&self) -> String {
        return self.value.pop_front().unwrap_or(String::from(""));
    }
}
