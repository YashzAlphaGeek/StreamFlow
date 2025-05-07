use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph};
use std::io; 

use crate::data_provider::DataProvider;

pub struct TerminalUI {
    data_provider: Arc<Mutex<DataProvider>>,
}

impl TerminalUI {
    pub fn new(data_provider: Arc<Mutex<DataProvider>>) -> Self {
        TerminalUI { data_provider }
    }

    pub fn run(&self) {
        let stdout = io::stdout(); 
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend).unwrap();

        loop {
            terminal.clear().unwrap();

            let mut data_provider = self.data_provider.lock().unwrap();
            let cpu_usage = data_provider.get_cpu_usage();
            let memory_usage = data_provider.get_memory_usage();
            let disk_usage = data_provider.get_disk_usage();
            
            let cpu_graph = DataProvider::generate_ascii_graph(cpu_usage);
            let memory_graph = DataProvider::generate_ascii_graph(memory_usage);
            let disk_graph = DataProvider::generate_ascii_graph(disk_usage);

            terminal.draw(|f| {
                let size = f.area(); 

                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Percentage(33), Constraint::Percentage(33), Constraint::Percentage(33)].as_ref())
                    .split(size);

                let block = Block::default()
                    .title("System Usage")
                    .borders(Borders::ALL);

                f.render_widget(block, chunks[0]); 

                f.render_widget(Paragraph::new(cpu_graph), chunks[1]);
                f.render_widget(Paragraph::new(memory_graph), chunks[2]);
                f.render_widget(Paragraph::new(disk_graph), chunks[2]); 
            }).unwrap();
            
            thread::sleep(Duration::from_secs(2));
        }
    }
}
