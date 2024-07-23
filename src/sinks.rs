use substreams::errors::Error;
use substreams_entity_change::pb::entity::{entity_change, EntityChanges};

use crate::pb::antelope::bps::v1::Pays;

#[substreams::handlers::map]
pub fn graph_out(pays: Pays) -> Result<EntityChanges, Error> {
    let mut entity_changes: EntityChanges = Default::default();

    for pay in pays.vpays {
        entity_changes
            .push_change(
                "pay".to_string(),
                format!("{}-{}", pay.trx_id, pay.action_index),
                pay.action_index as u64,
                entity_change::Operation::Create,
            )
            .change("type", "vote".to_string())
            .change("block_num", pay.block_num)
            .change("timestamp", pay.timestamp.unwrap().seconds.to_string())
            .change("trx_id", pay.trx_id)
            .change("bp", pay.bp)
            .change("quantity", pay.quantity);
    }
    for pay in pays.bpays {
        entity_changes
            .push_change(
                "pay".to_string(),
                format!("{}-{}", pay.trx_id, pay.action_index),
                pay.action_index as u64,
                entity_change::Operation::Create,
            )
            .change("type", "block".to_string())
            .change("block_num", pay.block_num)
            .change("timestamp", pay.timestamp.unwrap().seconds.to_string())
            .change("trx_id", pay.trx_id)
            .change("bp", pay.bp)
            .change("quantity", pay.quantity);
    }

    Ok(entity_changes)
}
