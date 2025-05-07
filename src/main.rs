mod data_provider;
mod data_receiver;
mod interval_support;
mod stream_service;
mod terminal_ui;

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use crate::data_provider::DataProvider;
use crate::data_receiver::DataReceiver;
use crate::interval_support::IntervalSupport;
use crate::stream_service::StreamService;
use crate::terminal_ui::TerminalUI;

fn main() {
    let data_provider = Arc::new(Mutex::new(DataProvider::new()));

    let interval_support = IntervalSupport::new(2);
    let data_receiver = DataReceiver::new();

    let stream_service = StreamService::new(data_provider.clone(), interval_support, data_receiver);

    stream_service.subscribe_to_data_stream("system_metrics");

    println!("Started stream, waiting for data...");
    thread::sleep(Duration::from_secs(10));

    println!("Adjusting polling interval to 1s...");
    stream_service.update_polling_interval(1);

    thread::sleep(Duration::from_secs(10));

    let ui = TerminalUI::new(data_provider);
    ui.run();
}
