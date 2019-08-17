mod utils;
pub mod transaction;

use std::str;
use protobuf::RepeatedField;
use crate::payload::{CreateAgentAction, SimpleSupplyPayload, SimpleSupplyPayload_Action};

use sawtooth_sdk::signing;
use sawtooth_sdk::messages::transaction::{TransactionHeader, Transaction};
use sawtooth_sdk::messages::batch::{BatchHeader, Batch};


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
    constants: &transaction::BCTransaction,
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
