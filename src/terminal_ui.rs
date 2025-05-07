use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph, Gauge};
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
                    .constraints([Constraint::Percentage(20), Constraint::Percentage(30), Constraint::Percentage(50)].as_ref())
                    .split(size);

                let block = Block::default()
                    .title("System Usage")
                    .borders(Borders::ALL)
                    .style(Style::default().fg(Color::Magenta).bg(Color::Black).add_modifier(Modifier::BOLD));
                f.render_widget(block, chunks[0]);

                let cpu_gauge = Gauge::default()
                    .block(Block::default().borders(Borders::ALL).title("CPU Usage"))
                    .ratio((cpu_usage / 100.0).into()) 
                    .style(Style::default().fg(Color::Green).bg(Color::Black));
                f.render_widget(cpu_gauge, chunks[1]);

                let memory_gauge = Gauge::default()
                    .block(Block::default().borders(Borders::ALL).title("Memory Usage"))
                    .ratio((memory_usage / 100.0).into()) 
                    .style(Style::default().fg(Color::Yellow).bg(Color::Black));
                f.render_widget(memory_gauge, chunks[2]);

                let disk_gauge = Gauge::default()
                    .block(Block::default().borders(Borders::ALL).title("Disk Usage"))
                    .ratio((disk_usage / 100.0).into()) 
                    .style(Style::default().fg(Color::Cyan).bg(Color::Black));
                f.render_widget(disk_gauge, chunks[2]);

                let graph_block = Block::default()
                    .title("ASCII Graphs")
                    .borders(Borders::ALL)
                    .style(Style::default().fg(Color::White).bg(Color::Black));
                f.render_widget(graph_block, chunks[2]);

                f.render_widget(Paragraph::new(cpu_graph).style(Style::default().fg(Color::Green)), chunks[2]);
                f.render_widget(Paragraph::new(memory_graph).style(Style::default().fg(Color::Yellow)), chunks[2]);
                f.render_widget(Paragraph::new(disk_graph).style(Style::default().fg(Color::Cyan)), chunks[2]);
            }).unwrap();
            
            thread::sleep(Duration::from_secs(2));
        }
    }
}
