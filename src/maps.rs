use std::collections::HashSet;

use substreams::errors::Error;
// use substreams::log;
use substreams_antelope::pb::TransactionTraces;
use substreams_antelope::Block;

#[substreams::handlers::map]
fn map_events(block: Block) -> Result<TransactionTraces, Error> {
    let mut allowed_actions = HashSet::new();

    allowed_actions.insert("eosio::claimrewards".to_string());

    let transaction_traces = block.into_transaction_traces().filter_map(|transaction_trace| {
        for action_trace in transaction_trace.clone().action_traces {
            let action = action_trace.clone().action.unwrap();
            let key = format!("{}::{}", action.account.clone().to_string(), action.name.to_string());
            if !allowed_actions.contains(&key) { continue; }
            return Some(transaction_trace)
        }
        return None;

    }).collect::<Vec<_>>();

    Ok( TransactionTraces{transaction_traces} )
}
