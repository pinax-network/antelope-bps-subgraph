use substreams::errors::Error;
use substreams::pb::sf::substreams::index::v1::Keys;
use substreams_antelope::pb::TransactionTraces;

#[substreams::handlers::map]
fn index_events(transaction_traces: TransactionTraces) -> Result<Keys, Error> {
    Ok(match transaction_traces.transaction_traces.is_empty() {
        true => Keys::default(),
        false => Keys {
            keys: vec!["claimrewards".to_string()],
        },
    })
}
