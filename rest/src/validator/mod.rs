// From GRID Project

use sawtooth_sdk::messaging::stream::{MessageConnection, MessageReceiver, MessageSender};
use sawtooth_sdk::messaging::zmq_stream::{ZmqMessageConnection, ZmqMessageSender};

pub struct SawtoothConnection {
    sender: ZmqMessageSender,
    receiver: MessageReceiver,
}

impl SawtoothConnection {
    pub fn new(validator_address: &str) -> SawtoothConnection {
        let zmq_connection = ZmqMessageConnection::new(&validator_address);
        let (sender, receiver) = zmq_connection.create();
        SawtoothConnection { sender, receiver }
    }

    pub fn get_sender(&self) -> Box<dyn MessageSender + Send> {
        Box::new(self.sender.clone())
    }

    pub fn get_receiver(&self) -> &MessageReceiver {
        &self.receiver
    }
}