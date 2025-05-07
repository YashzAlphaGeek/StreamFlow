use crate::data_provider::DataProvider;

#[derive(Clone)]
pub struct DataReceiver;

impl DataReceiver {
    pub fn new() -> Self {
        DataReceiver
    }

    pub fn receive_data(&self, cpu_data: f32, memory_data: f32, disk_data: f32) {
        println!("CPU Usage: {}", DataProvider::generate_ascii_graph(cpu_data));
        println!("Memory Usage: {}", DataProvider::generate_ascii_graph(memory_data));
        println!("Disk Usage: {}", DataProvider::generate_ascii_graph(disk_data));
    }
}
