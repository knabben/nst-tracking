use protobuf;

use sawtooth_sdk::messages::processor::TpProcessRequest;
use sawtooth_sdk::processor::handler::{ApplyError, TransactionContext, TransactionHandler};
use std::collections::HashMap;

use crate::addressing::*;
use crate::messages::*;
use crate::protobuf::Message;

#[derive(Debug, Clone)]
enum Action {
    CreateAgent(payload::CreateAgentAction),
    CreateRecord(payload::CreateRecordAction),
    UpdateRecord(payload::UpdateRecordAction),
    TransferRecord(payload::TransferRecordAction),
}

pub struct TradeState<'a> {
    context: &'a mut TransactionContext,
}

impl<'a> TradeState<'a> {
    pub fn new(context: &'a mut TransactionContext) -> TradeState {
        TradeState { context: context }
    }

    pub fn get_agent(&mut self, agent_id: &str) -> Result<Option<agent::Agent>, ApplyError> {
        let address = make_agent_address(agent_id);
        let d = self.context.get_state(vec![address])?;
        match d {
            Some(packed) => {
                let agents: agent::AgentContainer =
                    match protobuf::parse_from_bytes(packed.as_slice()) {
                        Ok(agents) => agents,
                        Err(_) => {
                            return Err(ApplyError::InternalError(String::from(
                                "Cannot deserialize agent container",
                            )))
                        }
                    };

                for agent in agents.get_entries() {
                    if agent.public_key == agent_id {
                        return Ok(Some(agent.clone()));
                    }
                }
                Ok(None)
            }
            None => Ok(None),
        }
    }

    pub fn set_agent(&mut self, agent_id: &str, agent: agent::Agent) -> Result<(), ApplyError> {
        let address = make_agent_address(agent_id);
        let d = self.context.get_state(vec![address.clone()])?;
        let mut agents = match d {
            Some(packed) => match protobuf::parse_from_bytes(packed.as_slice()) {
                Ok(agents) => agents,
                Err(_) => {
                    return Err(ApplyError::InternalError(String::from(
                        "Cannot deserialize agent container",
                    )))
                }
            },
            None => agent::AgentContainer::new(),
        };

        agents.entries.push(agent);
        agents.entries.sort_by_key(|a| a.clone().public_key);
        let serialized = match agents.write_to_bytes() {
            Ok(serialized) => serialized,
            Err(_) => {
                return Err(ApplyError::InternalError(String::from(
                    "Cannot serialize agent container",
                )))
            }
        };
        let mut sets = HashMap::new();
        sets.insert(address, serialized);
        self.context
            .set_state(sets)
            .map_err(|err| ApplyError::InternalError(format!("{}", err)))?;
        Ok(())
    }

    pub fn get_product(&mut self, record_id: &str) -> Result<Option<record::Record>, ApplyError> {
        let address = make_record_address(record_id);
        let d = self.context.get_state(vec![address])?;
        match d {
            Some(packed) => {
                let records: record::RecordContainer =
                    match protobuf::parse_from_bytes(packed.as_slice()) {
                        Ok(records) => records,
                        Err(_) => {
                            return Err(ApplyError::InternalError(String::from(
                                "Cannot deserialize record container",
                            )))
                        }
                    };

                for record in records.get_entries() {
                    if record.record_id == record_id {
                        return Ok(Some(record.clone()));
                    }
                }
                Ok(None)
            }
            None => Ok(None),
        }
    }

    pub fn set_product(
        &mut self,
        record_id: &str,
        product: record::Record,
    ) -> Result<(), ApplyError> {
        let address = make_record_address(record_id);
        let d = self.context.get_state(vec![address.clone()])?;
        let mut record_container = match d {
            Some(packed) => match protobuf::parse_from_bytes(packed.as_slice()) {
                Ok(records) => records,
                Err(_) => {
                    return Err(ApplyError::InternalError(String::from(
                        "Cannot deserialize record container",
                    )))
                }
            },
            None => record::RecordContainer::new(),
        };

        let records = record_container.get_entries().to_vec();
        let mut index = None;
        let mut count = 0;
        
        for record in records.clone() {
            if record.record_id == record_id {
                index = Some(count);
                break;
            }
            count = count + 1;
        }

        match index {
            Some(x) => {
                record_container.entries.remove(x);
            }
            None => (),
        };
        record_container.entries.push(product);
        record_container
            .entries
            .sort_by_key(|r| r.clone().record_id);
        let serialized = match record_container.write_to_bytes() {
            Ok(serialized) => serialized,
            Err(_) => {
                return Err(ApplyError::InternalError(String::from(
                    "Cannot serialize record container",
                )))
            }
        };
        let mut sets = HashMap::new();
        sets.insert(address, serialized);
        self.context
            .set_state(sets)
            .map_err(|err| ApplyError::InternalError(format!("{}", err)))?;

        println!("Product state saved");
        Ok(())
    }
}

struct TradePayload {
    action: Action,
    timestamp: String,
}

impl TradePayload {
    pub fn new(payload: &[u8]) -> Result<Option<TradePayload>, ApplyError> {
        let payload: payload::SimpleSupplyPayload = match protobuf::parse_from_bytes(payload) {
            Ok(payload) => payload,
            Err(_) => {
                return Err(ApplyError::InvalidTransaction(String::from(
                    "Cannot deserialize payload",
                )))
            }
        };

        let trade_action = payload.get_action();
        let action = match trade_action {
            payload::SimpleSupplyPayload_Action::CREATE_AGENT => {
                let create_agent = payload.get_create_agent();
                if create_agent.get_name() == "" {
                    return Err(ApplyError::InvalidTransaction(String::from(
                        "Agent name cannot be an empty string",
                    )));
                }
                Action::CreateAgent(create_agent.clone())
            }

            payload::SimpleSupplyPayload_Action::CREATE_RECORD => {
                let create_record = payload.get_create_record();
                if create_record.get_record_id() == "" {
                    return Err(ApplyError::InvalidTransaction(String::from(
                        "Record id cannot be empty string",
                    )));
                }
                Action::CreateRecord(payload.get_create_record().clone())
            }

            payload::SimpleSupplyPayload_Action::UPDATE_RECORD => {
                Action::UpdateRecord(payload.get_update_record().clone())
            }

            payload::SimpleSupplyPayload_Action::TRANSFER_RECORD => {
                Action::TransferRecord(payload.get_transfer_record().clone())
            }
        };

        let timestamp = match payload.get_timestamp() {
            "" => {
                return Err(ApplyError::InvalidTransaction(String::from(
                    "Timestamp is not set",
                )))
            }
            x => x,
        };

        Ok(Some(TradePayload {
            action: action,
            timestamp: timestamp.to_string(),
        }))
    }

    pub fn get_action(&self) -> Action {
        self.action.clone()
    }

    pub fn get_timestamp(&self) -> String {
        self.timestamp.clone()
    }
}
pub struct TradeTransactionHandler {
    family_name: String,
    family_versions: Vec<String>,
    namespaces: Vec<String>,
}

impl TradeTransactionHandler {
    pub fn new() -> TradeTransactionHandler {
        TradeTransactionHandler {
            family_name: "trade".to_string(),
            family_versions: vec!["1.0".to_string()],
            namespaces: vec![get_supply_chain_prefix().to_string()],
        }
    }

    fn _create_agent(
        &self,
        payload: payload::CreateAgentAction,
        mut state: TradeState,
        signer: &str,
        timestamp: String,
    ) -> Result<(), ApplyError> {
        let name = payload.get_name();
        println!("Initial agent -- {}", signer);

        match state.get_agent(signer) {
            Ok(Some(_)) => {
                return Err(ApplyError::InvalidTransaction(format!(
                    "Agent already exists: {}",
                    name
                )))
            }
            Ok(None) => (),
            Err(err) => return Err(err),
        }

        let mut new_agent = agent::Agent::new();
        new_agent.set_public_key(signer.to_string());
        new_agent.set_name(name.to_string());
        new_agent.set_timestamp(0);

        state.set_agent(signer, new_agent)?;
        println!("Created agent");
        Ok(())
    }

    fn _create_record(
        &self,
        payload: payload::CreateRecordAction,
        mut state: TradeState,
        signer: &str,
        timestamp: String,
    ) -> Result<(), ApplyError> {
        match state.get_agent(signer) {
            Ok(Some(_)) => (),
            Ok(None) => {
                return Err(ApplyError::InvalidTransaction(format!(
                    "Agent is not registered: {}",
                    signer
                )))
            }
            Err(err) => return Err(err),
        }

        let record_id = payload.get_record_id();
        match state.get_product(record_id) {
            Ok(Some(_)) => {
                return Err(ApplyError::InvalidTransaction(format!(
                    "Record already exists: {}",
                    record_id
                )))
            }
            Ok(None) => (),
            Err(err) => return Err(err),
        }

        let mut coords = record::Record_Location::new();
        coords.set_latitude(payload.get_latitude());
        coords.set_longitude(payload.get_longitude());

        let mut owner = record::Record_Owner::new();
        owner.set_agent_id(signer.to_string());
        owner.set_timestamp(0);

        let mut record = record::Record::new();
        record.set_title(payload.get_title().to_string());
        record.set_price(payload.get_price());

        record.owners.push(owner.clone());
        record.locations.push(coords.clone());

        state.set_product(record_id, record)?;
        
        Ok(())
    }
}

impl TransactionHandler for TradeTransactionHandler {
    fn family_name(&self) -> String {
        return self.family_name.clone();
    }

    fn family_versions(&self) -> Vec<String> {
        return self.family_versions.clone();
    }

    fn namespaces(&self) -> Vec<String> {
        return self.namespaces.clone();
    }

    fn apply(
        &self,
        request: &TpProcessRequest,
        context: &mut TransactionContext,
    ) -> Result<(), ApplyError> {
        let payload = TradePayload::new(request.get_payload());
        let payload = match payload {
            Err(e) => return Err(e),
            Ok(payload) => payload,
        };

        let payload = match payload {
            Some(x) => x,
            None => {
                return Err(ApplyError::InvalidTransaction(String::from(
                    "Request must contain a payload",
                )))
            }
        };

        println!(
            "payload: {:?} {} {} {}",
            payload.get_action(),
            payload.get_timestamp(),
            request.get_header().get_inputs()[0],
            request.get_header().get_outputs()[0]
        );

        let signer = request.get_header().get_signer_public_key();
        let state = TradeState::new(context);

        match payload.get_action() {
            Action::CreateAgent(agent_payload) => {
                self._create_agent(agent_payload, state, signer, payload.get_timestamp())?
            },
            Action::CreateRecord(record_payload) => {
                self._create_record(record_payload, state, signer, payload.get_timestamp());
            },
            _ => ()
        }
        Ok(())
    }
}
