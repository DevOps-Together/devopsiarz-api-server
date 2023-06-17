use crate::queuing_strategy::{AddressableQueing};

pub struct RepeatLast {
    address: String,
    value: String,
}

impl AddressableQueing for RepeatLast {
    fn get_address(&self) -> String {
        return self.address;
    }
    fn put(&self, data : String) {
        self.value = data;
    }

    fn take(&self) -> String {
        return self.value;
    }
}