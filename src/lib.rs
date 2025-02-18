mod pb;
mod abi;

use pb::starknet::v1::*;
use crate::abi::usdc_contract::Event as UsdcEvent;

use substreams::Hex;
use cainome::cairo_serde::CairoSerde;
use starknet::core::types::Felt;
use substreams::log;
use crate::pb::sf::substreams::starknet::r#type::v1::Transactions;
use num_traits::cast::ToPrimitive;
use starknet::core::types::EmittedEvent;
#[substreams::handlers::map]
fn map_usdc_events(transactions: Transactions) -> Result<Events, substreams::errors::Error> {
    let mut proto_events = Events::default();
    for transaction in transactions.transactions_with_receipt {
        let data = transaction.receipt.unwrap();

        let data_events = data.events;

        for event in data_events {
            let event_from_address = Hex(event.from_address.as_slice()).to_string();

            if event_from_address != "053c91253bc9682c04929ca02ed00b3e423f6710d2ee7e0d5ebb06f3ecf368a8" {
                continue;
            }

            let mut data_felts = vec![];
            let mut keys_felts = vec![];
            for key in event.keys {
                let key = Felt::from_bytes_be_slice(key.as_slice());
                keys_felts.push(key);
            }

            for bytes in event.data {
                let felt = Felt::from_bytes_be_slice(bytes.as_slice());
                data_felts.push(felt);
            }

            let emitted_event = EmittedEvent {
                from_address: Felt::from_bytes_be_slice(event.from_address.as_slice()),
                keys: keys_felts,
                data: data_felts,
                block_hash: None,
                block_number: None,
                transaction_hash: Felt::default(),
            };

            if let usdc_event = UsdcEvent::try_from(emitted_event).unwrap() {
                let event_json = serde_json::to_string(&usdc_event).unwrap();
                let event = Event {
                    json_description: event_json
                };

                proto_events.events.push(event);
            }
        }
    }

    Ok(proto_events)
}
