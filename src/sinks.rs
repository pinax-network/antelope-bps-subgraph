use substreams::{errors::Error, Hex};
use substreams_antelope::pb::TransactionTraces;
use substreams_entity_change::pb::entity::{entity_change, EntityChanges};

use crate::keys::to_key;

#[substreams::handlers::map]
pub fn graph_out(map_events: TransactionTraces) -> Result<EntityChanges, Error> {
    let mut entity_changes: EntityChanges = Default::default();

    for transaction_trace in map_events.transaction_traces {
        for action_trace in transaction_trace.clone().action_traces {
            let action = action_trace.clone().action.unwrap();
            if action_trace.receiver != action.account { continue; } // filter out extra inline actions

            entity_changes
                .push_change(action.name.clone(), to_key(&action_trace), 0, entity_change::Operation::Create)

                // transaction
                .change("block_num", transaction_trace.block_num)
                .change("timestamp", transaction_trace.clone().block_time.unwrap().seconds.to_string())
                .change("transaction_id", action_trace.transaction_id)
                .change("action_ordinal", action_trace.action_ordinal.to_string())

                // action
                .change("account", action.account)
                .change("name", action.name)
                .change("json_data", action.json_data)
                .change("raw_data", Hex::encode(action.raw_data));
        }
    }

    Ok(entity_changes)
}
