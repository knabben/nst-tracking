extern crate protobuf;
extern crate rustc_serialize;
extern crate sawtooth_sdk;


mod handler;
mod addressing;
mod messages;

use sawtooth_sdk::processor::TransactionProcessor;
use handler::TradeTransactionHandler;


fn main() {
    let endpoint = &"tcp://sawtooth-validator:4004".to_string();
    let mut processor = TransactionProcessor::new(endpoint);

    let handler = TradeTransactionHandler::new();
    
    processor.add_handler(&handler);
    processor.start();
}
