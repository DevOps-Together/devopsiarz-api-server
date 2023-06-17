use crate::queuing_strategy::{AddressableQueing};

pub struct Filo {
    address: String,
    value: Vec<String>,
}

impl AddressableQueing for Filo {
    fn get_address(&self) -> String {
        return self.address;
    }
    fn put(&self, data : String) {
        self.value.push(data);
    }

    fn take(&self) -> String {
        return self.value.pop().unwrap_or(String::from(""));
    }
}
