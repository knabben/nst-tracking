// From GRID Project

use sawtooth_sdk::messaging::stream::{MessageConnection, MessageSender};
use sawtooth_sdk::messaging::zmq_stream::{ZmqMessageConnection, ZmqMessageSender};

#[derive(Clone)]
pub struct SawtoothConnection {
    pub sender: ZmqMessageSender,
}

impl SawtoothConnection {
    pub fn new(validator_address: &str) -> SawtoothConnection {
        let zmq_connection = ZmqMessageConnection::new(&validator_address);
        let (sender, _receiver) = zmq_connection.create();
        SawtoothConnection { sender }
    }

    pub fn get_sender(&self) -> Box<dyn MessageSender> {
        Box::new(self.sender.clone())
    }
}