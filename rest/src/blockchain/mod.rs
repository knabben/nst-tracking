pub mod transaction;
pub mod utils;

use crate::payload::{
    CreateAgentAction, CreateBidAction, CreateRecordAction, SimpleSupplyPayload, SimpleSupplyPayload_Action,
};
use protobuf::RepeatedField;
use std::str;

use sawtooth_sdk::messages::batch::{Batch, BatchHeader};
use sawtooth_sdk::messages::transaction::{Transaction, TransactionHeader};
use sawtooth_sdk::signing;

fn serialize_agent_payload(username: String) -> SimpleSupplyPayload {
    let mut create_agent = CreateAgentAction::new();
    create_agent.set_name(username.to_string());

    let timestamp = time::get_time();
    let mills = timestamp.sec as u64 + timestamp.nsec as u64 / 1000 / 1000;
    debug!("timestamp {:?}", mills);

    let action_msg = match protobuf::Message::write_to_bytes(&create_agent) {
        Ok(b) => b,
        Err(error) => {
            error!("Error serializing request: {:?}", error);
            return SimpleSupplyPayload::new();
        }
    };
    debug!(
        "create_agent - {:?} {:?}",
        action_msg,
        str::from_utf8(&action_msg)
    );

    let mut agent_payload = SimpleSupplyPayload::new();
    agent_payload.set_action(SimpleSupplyPayload_Action::CREATE_AGENT);
    agent_payload.set_create_agent(create_agent);
    agent_payload.set_timestamp(mills.to_string());

    agent_payload
}

pub fn serialize_product_payload(
    record_id: String,
    title: String, 
    price: i64,
    latitude: i64,
    longitude: i64,
) -> SimpleSupplyPayload {
    let timestamp = time::get_time();
    let mills = timestamp.sec as u64 + timestamp.nsec as u64 / 1000 / 1000;

    let mut create_product = CreateRecordAction::new();
    create_product.set_record_id(record_id);
    create_product.set_title(title);
    create_product.set_price(price);
    create_product.set_latitude(latitude);
    create_product.set_longitude(longitude);

    let _product_msg = match protobuf::Message::write_to_bytes(&create_product) {
        Ok(b) => b,
        Err(error) => {
            error!("Error serializing request: {:?}", error);
            return SimpleSupplyPayload::new();
        }
    };

    let mut product_payload = SimpleSupplyPayload::new();
    product_payload.set_action(SimpleSupplyPayload_Action::CREATE_RECORD);
    product_payload.set_create_record(create_product);
    product_payload.set_timestamp(mills.to_string());

    product_payload
}

pub fn serialize_bid_payload(
    product_id: i64,
    price: i64
) -> SimpleSupplyPayload {
    let timestamp = time::get_time();
    let mills = timestamp.sec as u64 + timestamp.nsec as u64 / 1000 / 1000;

    let mut bid_product = CreateBidAction::new();
    bid_product.set_product_id(product_id);
    bid_product.set_price(price);

    let _product_msg = match protobuf::Message::write_to_bytes(&bid_product) {
        Ok(b) => b,
        Err(error) => {
            error!("Error serializing request: {:?}", error);
            return SimpleSupplyPayload::new();
        }
    };

    let mut bid_payload = SimpleSupplyPayload::new();
    bid_payload.set_action(SimpleSupplyPayload_Action::CREATE_BID);
    bid_payload.set_create_bid(bid_product);
    bid_payload.set_timestamp(mills.to_string());

    bid_payload
}

pub fn serialize_transaction_payload(
    payload: SimpleSupplyPayload,
    public_key: &str,
    constants: &transaction::BCTransaction,
    agent_address: String,
    signer: signing::Signer,
) -> Batch {
    let agent_msg = match protobuf::Message::write_to_bytes(&payload) {
        Ok(b) => b,
        Err(error) => {
            error!("Error serializing request: {:?}", error);
            return Batch::new();
        }
    };

    let agent_string = match str::from_utf8(&agent_msg) {
        Ok(b) => b,
        Err(error) => {
            error!("Bytes error: {:?}", error);
            return Batch::new();
        }
    };
    debug!("agent - {:?}", agent_msg);

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
            error!("Error serializing request: {:?}", error);
            return Batch::new();
        }
    };
    debug!("transaction_header: {:?}", transaction_msg);

    // Transaction
    let mut transaction = Transaction::new();

    let transaction_header_msg = match protobuf::Message::write_to_bytes(&transaction_header) {
        Ok(b) => b,
        Err(error) => {
            error!("Error serializing request: {:?}", error);
            return Batch::new();
        }
    };
    let signature = signer.sign(&transaction_header_msg).unwrap();
    transaction.set_header(transaction_header_msg);
    transaction.set_header_signature(signature);
    transaction.set_payload(agent_msg);
    debug!("transaction {:?}", transaction);

    // Batch header
    let mut batch_header = BatchHeader::new();
    batch_header.set_signer_public_key(public_key.to_string());
    batch_header.set_transaction_ids(RepeatedField::from_vec(vec![transaction
        .get_header_signature()
        .to_string()]));

    // Batch protobuf
    let mut batch = Batch::new();
    let batch_header_msg = match protobuf::Message::write_to_bytes(&batch_header) {
        Ok(b) => b,
        Err(error) => {
            error!("Error serializing request: {:?}", error);
            return Batch::new();
        }
    };
    let sign_batch = signer.sign(&batch_header_msg).unwrap();

    batch.set_header(batch_header_msg);
    batch.set_header_signature(sign_batch);
    batch.set_transactions(RepeatedField::from_vec(vec![transaction]));

    batch
}
