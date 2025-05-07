use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub struct IntervalSupport {
    interval_secs: Arc<Mutex<u64>>,
}

impl IntervalSupport {
    pub fn new(interval_secs: u64) -> Self {
        IntervalSupport {
            interval_secs: Arc::new(Mutex::new(interval_secs)),
        }
    }

    pub fn update_interval(&self, new_interval: u64) {
        let mut interval = self.interval_secs.lock().unwrap();
        *interval = new_interval;
    }

    pub fn start_polling<F>(
        &self,
        _subscription_key: String,
        mut data_handler: F,
        data_provider: Arc<Mutex<crate::data_provider::DataProvider>>,
    )
    where
        F: FnMut(f32, f32, f32) + Send + 'static,
    {
        let interval_secs = Arc::clone(&self.interval_secs);

        thread::spawn(move || loop {
            let interval = *interval_secs.lock().unwrap();

            let mut provider = data_provider.lock().unwrap();
            let cpu = provider.get_cpu_usage();
            let memory = provider.get_memory_usage();
            let disk = provider.get_disk_usage();

            data_handler(cpu, memory, disk);

            thread::sleep(Duration::from_secs(interval));
        });
    }
}
