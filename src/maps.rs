use prost_types::Timestamp;
use substreams::errors::Error;
// use substreams::log;
use crate::abi;
use crate::pb::antelope::bps::v1::{Pay, Pays};
use substreams_antelope::decoder::decode;
use substreams_antelope::pb::{ActionTrace, TransactionTraces};

impl Pay {
    fn new(transfer: abi::actions::Transfer, action: &ActionTrace) -> Self {
        Pay {
            bp: transfer.to,
            quantity: transfer.quantity,
            block_num: action.block_num,
            timestamp: Some(Timestamp {
                seconds: action.block_time.as_ref().unwrap().seconds,
                nanos: 0,
            }),
            trx_id: action.transaction_id.clone(),
            action_index: action.action_ordinal,
            ..Default::default()
        }
    }
}

#[substreams::handlers::map]
pub fn map_pays(transactions: TransactionTraces) -> Result<Pays, Error> {
    let pays = transactions.transaction_traces.iter().fold(
        Default::default(),
        |mut pays: Pays, transaction_trace| {
            for action_trace in transaction_trace.action_traces.iter() {
                let action = action_trace.action.as_ref().unwrap();
                if action_trace.receiver != action.account
                    || action.name != "transfer"
                    || action.account != "eosio.token"
                {
                    continue;
                }
                let transfer = decode::<abi::actions::Transfer>(&action.json_data)
                    .expect(&format!("failed to unwrap json for action: {:?}", action));
                match transfer.from.as_str() {
                    "eosio.vpay" => {
                        pays.vpays.push(Pay::new(transfer, action_trace));
                    }
                    "eosio.bpay" => {
                        pays.bpays.push(Pay::new(transfer, action_trace));
                    }
                    _ => {}
                }
            }
            pays
        },
    );

    Ok(pays)
}
