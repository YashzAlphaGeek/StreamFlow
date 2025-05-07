use sysinfo::{Disks, System, CpuRefreshKind, RefreshKind};


pub struct DataProvider {
    system: System,
}

impl DataProvider {
    pub fn new() -> Self {
        let refresh = RefreshKind::everything();
        let mut system = System::new_with_specifics(refresh);
        system.refresh_all();
        DataProvider { system }
    }

    pub fn get_cpu_usage(&mut self) -> f32 {
        self.system.refresh_cpu_specifics(CpuRefreshKind::everything());
        let cpus = self.system.cpus();
        if cpus.is_empty() {
            return 0.0;
        }
        let total_usage: f32 = cpus.iter().map(|cpu| cpu.cpu_usage()).sum();
        total_usage / cpus.len() as f32
    }

    pub fn get_memory_usage(&mut self) -> f32 {
        self.system.refresh_memory();
        let total_memory = self.system.total_memory() as f32;
        let used_memory = self.system.used_memory() as f32;
        if total_memory == 0.0 {
            return 0.0;
        }
        (used_memory / total_memory) * 100.0
    }

    pub fn get_disk_usage(&self) -> f32 {
        let disks = Disks::new_with_refreshed_list();
        if let Some(disk) = disks.iter().next() {
            let total = disk.total_space() as f32;
            let available = disk.available_space() as f32;
            let used = total - available;
            if total == 0.0 {
                return 0.0;
            }
            return (used / total) * 100.0;
        }
        0.0
    }

    pub fn generate_ascii_graph(value: f32) -> String {
        let total_bars: f32 = 30.0; 
        let filled_bars = (value * total_bars / 100.0).round() as usize; 
        let empty_bars = total_bars as usize - filled_bars; 
    
        let filled = "█".repeat(filled_bars);
        let empty = " ".repeat(empty_bars);
    
        format!("[{}{}] {:.2}%", filled, empty, value)
    }
    

    pub fn subscribe(&mut self, stream_name: &str) -> String {
        format!("subscription_key_for_{}", stream_name)
    }
}
