use std::sync::{Arc, Mutex};

use crate::data_provider::DataProvider;
use crate::data_receiver::DataReceiver;
use crate::interval_support::IntervalSupport;

pub struct StreamService {
    data_provider: Arc<Mutex<DataProvider>>,
    interval_support: IntervalSupport,
    data_receiver: DataReceiver,
}

impl StreamService {
    pub fn new(
        data_provider: Arc<Mutex<DataProvider>>,
        interval_support: IntervalSupport,
        data_receiver: DataReceiver,
    ) -> Self {
        StreamService {
            data_provider,
            interval_support,
            data_receiver,
        }
    }

    pub fn subscribe_to_data_stream(&self, stream_name: &str) {
        let key = self
            .data_provider
            .lock()
            .unwrap()
            .subscribe(stream_name);

        let receiver = self.data_receiver.clone();
        let provider_clone = Arc::clone(&self.data_provider);

        self.interval_support.start_polling(
            key,
            move |cpu, mem, disk| {
                receiver.receive_data(cpu, mem, disk);
            },
            provider_clone,
        );
    }

    pub fn update_polling_interval(&self, new_interval: u64) {
        self.interval_support.update_interval(new_interval);
    }
}
