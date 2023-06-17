use crate::queuing_strategy::{AddressableQueing};

pub struct Constant {
    address: String,
    value: String,
}

impl AddressableQueing for Constant {
    fn get_address(&self) -> String {
        return self.address;
    }
    fn put(&self, data : String) {
        return;
    }

    fn take(&self) -> String {
        return self.value;
    }
}
