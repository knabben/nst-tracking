mod utils;

use serde::ser;
use uuid::Uuid;
use crate::payload::{CreateAgentAction, SimpleSupplyPayload, SimpleSupplyPayload_Action};
use std::convert::TryInto;

use std::str;
use protobuf::Message as M;
use protobuf::RepeatedField;

use crate::sawtooth_sdk::messaging::stream::MessageConnection;

use sawtooth_sdk::messages::transaction::{TransactionHeader, Transaction};
use sawtooth_sdk::messaging::zmq_stream::{ZmqMessageConnection};
use sawtooth_sdk::messaging::stream::MessageSender;
use sawtooth_sdk::messages::validator::Message_MessageType;
use sawtooth_sdk::messages::client_batch_submit::{ClientBatchSubmitRequest};
use sawtooth_sdk::messages::batch::{BatchHeader, Batch};
use sawtooth_sdk::signing;

use sawtooth_sdk::signing::{Context, PrivateKey, PublicKey};

pub struct BCTransaction {
    family_name: String,
    family_version: String,
    agent_prefix: String,
    pub context: Box<Context>,
}

impl BCTransaction {
    pub fn new (
        family_name: String,
        family_version: String,
        agent_prefix: String
    ) -> BCTransaction {
        let context = signing::create_context("secp256k1").unwrap();
        BCTransaction {
            context: context,
            family_name: family_name,
            family_version: family_version,
            agent_prefix: agent_prefix
        }
    }

    // Context and key pair
    pub fn generate_key_pair(&self, context: &Context) -> (Box<dyn PrivateKey>, Box<dyn PublicKey>){
        let private_key = context.new_random_private_key().unwrap();
        let public_key = context.get_public_key(&*private_key).unwrap();

        println!("DEBUG: private_key: {:?}\npublic_key: {:?}", &*private_key.as_hex(), &*public_key.as_hex());

        (private_key, public_key)
    }

    pub fn store(
        &self,
        signer: signing::Signer,
        public_key: String,
        username: String
    ) {

        // Calculate agent address
        let hashed_family = utils::hashed_value(&self.family_name);
        let _namespace = &hashed_family[0..6];
        let hashed_pk = utils::hashed_value(&*public_key);

        // Agent address
        let agent_address = &format!("{}{}{}", _namespace, self.agent_prefix, &hashed_pk[0..62]);

        let agent_payload = serialize_payload(username.to_string());
        let batch = serialize_tp_payload(
            agent_payload,
            &*public_key,
            self,
            agent_address.to_string(),
            signer,
        );

        println!("DEBUG: batch - {:?}", batch);
        // ------------------

        let mut submit_request = ClientBatchSubmitRequest::new();
        submit_request.set_batches(RepeatedField::from_vec(vec![batch]));

        let connection = ZmqMessageConnection::new(&"tcp://localhost:4004");
        let (sender, receiver) = connection.create();
        
        let correlation_id = Uuid::new_v4().to_string();

        let msg_bytes = match protobuf::Message::write_to_bytes(&submit_request) {
            Ok(b) => b,
            Err(error) => {
                println!("Error serializing request: {:?}", error);
                return;
            },
        };

        let mut future = match sender.send(Message_MessageType::CLIENT_BATCH_SUBMIT_REQUEST, &correlation_id, &msg_bytes) {
            Ok(f) => f,
            Err(error) => {
                println!("Error unwrapping future: {:?}", error);
                return;
            },
        };
        println!("Client batch submit");
        let response_msg = match future.get() {
            Ok(m) => m,
            Err(error) => {
                println!("Error getting future: {:?}", error);
                return;
            },
        };

        println!("{:?}", response_msg);
    }

}

fn serialize_payload(username: String) -> SimpleSupplyPayload {
    let mut create_agent = CreateAgentAction::new();
    create_agent.set_name(username.to_string());

    let timestamp = time::get_time();
    let mills = timestamp.sec as u64 + timestamp.nsec as u64 / 1000 / 1000;
    println!("timestamp {:?}", mills);

    let action_msg = match protobuf::Message::write_to_bytes(&create_agent) {
        Ok(b) => b,
        Err(error) => {
            println!("Error serializing request: {:?}", error);
            return SimpleSupplyPayload::new();
        },
    };

    println!("DEBUG: create_agent - {:?} {:?}", action_msg, str::from_utf8(&action_msg));

    let mut agent_payload = SimpleSupplyPayload::new();
    agent_payload.set_action(SimpleSupplyPayload_Action::CREATE_AGENT);
    agent_payload.set_create_agent(create_agent);
    agent_payload.set_timestamp(mills.to_string());

    agent_payload
}


fn serialize_tp_payload(
    agent_payload: SimpleSupplyPayload,
    public_key: &str,
    constants: &BCTransaction,
    agent_address: String,
    signer: signing::Signer,
) -> Batch {
    let agent_msg = match protobuf::Message::write_to_bytes(&agent_payload) {
        Ok(b) => b,
        Err(error) => {
            println!("Error serializing request: {:?}", error);
            return Batch::new();
        },
    };

    let agent_string = match str::from_utf8(&agent_msg) {
        Ok(b) => b,
        Err(error) => {
            println!("Bytes error: {:?}", error);
            return Batch::new();
        }
    };
    println!("DEBUG: agent - {:?}", agent_msg);

    let inputs = vec![agent_address.to_string()];
    let outputs = vec![agent_address.to_string()];

    // Transaction header
    let mut transaction_header = TransactionHeader::new();

    transaction_header.set_family_name(constants.family_name.clone());
    transaction_header.set_family_version(constants.family_version.clone());
    transaction_header.set_inputs(RepeatedField::from_vec(inputs));
    transaction_header.set_outputs(RepeatedField::from_vec(outputs));
    transaction_header.set_signer_public_key(public_key.to_string());
    transaction_header.set_batcher_public_key(public_key.to_string());
    transaction_header.set_dependencies(RepeatedField::from_vec(vec![]));
    transaction_header.set_payload_sha512(utils::hashed_value(agent_string));

    let transaction_msg = match protobuf::Message::write_to_bytes(&transaction_header) {
        Ok(b) => b,
        Err(error) => {
            println!("Error serializing request: {:?}", error);
            return Batch::new();
        },
    };

    println!("DEBUG: transaction_header: {:?}", transaction_msg);

    // Transaction
    let mut transaction = Transaction::new();

    let transaction_header_msg = match protobuf::Message::write_to_bytes(&transaction_header) {
        Ok(b) => b,
        Err(error) => {
            println!("Error serializing request: {:?}", error);
            return Batch::new();
        },
    };
    let signature = signer.sign(&transaction_header_msg).unwrap();
    transaction.set_header(transaction_header_msg);
    transaction.set_header_signature(signature);
    transaction.set_payload(agent_msg);

    println!("DEBUG: transaction {:?}", transaction);

    // Batch header
    let mut batch_header = BatchHeader::new();
    batch_header.set_signer_public_key(public_key.to_string());
    batch_header.set_transaction_ids(RepeatedField::from_vec(
        vec![transaction.get_header_signature().to_string()])
    );

    // Batch
    let mut batch = Batch::new();
    let batch_header_msg = match protobuf::Message::write_to_bytes(&batch_header) {
        Ok(b) => b,
        Err(error) => {
            println!("Error serializing request: {:?}", error);
            return Batch::new();
        },
    };
    let sign_batch = signer.sign(&batch_header_msg).unwrap();

    batch.set_header(batch_header_msg);
    batch.set_header_signature(sign_batch);
    batch.set_transactions(RepeatedField::from_vec(vec![transaction]));

    batch
}
