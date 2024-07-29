use antelope::Asset;
use prost_types::Timestamp;
use substreams::errors::Error;
// use substreams::log;
use crate::abi;
use crate::pb::antelope::bps::v1::{Bps, Pay, Reg};
use substreams_antelope::decoder::decode;
use substreams_antelope::pb::{ActionTrace, TransactionTraces};

impl Pay {
    fn from_transfer(transfer: abi::eosio_token::actions::Transfer, action: &ActionTrace) -> Self {
        let quantity = Asset::from(transfer.quantity.as_str());
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
            amount: quantity.amount,
            value: quantity.value(),
        }
    }
}

impl Reg {
    fn from_regproducer(reg: abi::eosio::actions::Regproducer, action: &ActionTrace) -> Self {
        Reg {
            bp: reg.producer,
            url: reg.url,
            location: reg.location as u32,
            public_key: reg.producer_key,
            block_num: action.block_num,
            timestamp: Some(Timestamp {
                seconds: action.block_time.as_ref().unwrap().seconds,
                nanos: 0,
            }),
            trx_id: action.transaction_id.clone(),
            action_index: action.action_ordinal,
        }
    }
    fn from_regproducer2(reg: abi::eosio::actions::Regproducer2, action: &ActionTrace) -> Self {
        // collect keys into a string separated by ";" for simplicity
        let keys = reg
            .producer_authority
            .keys
            .iter()
            .map(|key| format!("{}-{}", key.weight, key.key))
            .collect::<Vec<String>>()
            .join(";");

        Reg {
            bp: reg.producer,
            url: reg.url,
            location: reg.location as u32,
            public_key: keys,
            block_num: action.block_num,
            timestamp: Some(Timestamp {
                seconds: action.block_time.as_ref().unwrap().seconds,
                nanos: 0,
            }),
            trx_id: action.transaction_id.clone(),
            action_index: action.action_ordinal,
        }
    }
}

#[substreams::handlers::map]
pub fn map_bps(transactions: TransactionTraces) -> Result<Bps, Error> {
    let bps = transactions.transaction_traces.iter().fold(
        Default::default(),
        |mut bps: Bps, transaction_trace| {
            for action_trace in transaction_trace.action_traces.iter() {
                let action = action_trace.action.as_ref().unwrap();
                if action_trace.receiver != action.account {
                    continue;
                }
                match (action.account.as_str(), action.name.as_str()) {
                    ("eosio.token", "transfer") => {
                        let transfer =
                            decode::<abi::eosio_token::actions::Transfer>(&action.json_data)
                                .expect(&format!("failed to unwrap json for action: {:?}", action));
                        match transfer.from.as_str() {
                            "eosio.vpay" => {
                                bps.vpays.push(Pay::from_transfer(transfer, action_trace));
                            }
                            "eosio.bpay" => {
                                bps.bpays.push(Pay::from_transfer(transfer, action_trace));
                            }
                            _ => {}
                        }
                    }
                    ("eosio", "regproducer") => {
                        let reg =
                            decode::<abi::eosio::actions::Regproducer>(&action.json_data).expect(
                                &format!("failed to unwrap json for action: {}", action.name),
                            );
                        bps.regs.push(Reg::from_regproducer(reg, action_trace));
                    }
                    ("eosio", "regproducer2") => {
                        let reg =
                            decode::<abi::eosio::actions::Regproducer2>(&action.json_data).expect(
                                &format!("failed to unwrap json for action: {}", action.name),
                            );
                        bps.regs.push(Reg::from_regproducer2(reg, action_trace));
                    }
                    _ => continue,
                }
            }
            bps
        },
    );

    Ok(bps)
}
