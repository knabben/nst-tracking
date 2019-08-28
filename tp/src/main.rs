extern crate protobuf;
extern crate rustc_serialize;
extern crate sawtooth_sdk;

mod addressing;
mod handler;
mod messages;

use handler::TradeTransactionHandler;
use sawtooth_sdk::processor::TransactionProcessor;

fn main() {
    println!("Starting sawtooth validator");

    let endpoint = &"tcp://localhost:4004".to_string();
    let mut processor = TransactionProcessor::new(endpoint);

    let handler = TradeTransactionHandler::new();

    processor.add_handler(&handler);
    processor.start();
}
