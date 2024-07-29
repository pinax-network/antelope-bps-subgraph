# Antelope BP registrations and rewards substreams

> Substreams for Antelope Block Producer registrations and rewards.

## Quickstart

```
$ git clone https://github.com/pinax-network/antelope-bps-subgraph
$ cd antelope-bps-subgraph/substreams
$ make
$ make gui
```

## Releases

- https://github.com/pinax-network/antelope-bps-subgraph/releases

### Mermaid Graph

```mermaid
graph TD;
  map_bps[map: map_bps];
  common:filtered_transactions --> map_bps;
  common:all_actions --> common:index_actions;
  common:all_actions --> common:index_actions_extra;
  common:all_transactions[map: common:all_transactions];
  sf.antelope.type.v1.Block[source: sf.antelope.type.v1.Block] --> common:all_transactions;
  common:all_actions[map: common:all_actions];
  common:all_transactions --> common:all_actions;
  common:filtered_transactions[map: common:filtered_transactions];
  common:filtered_transactions:params[params] --> common:filtered_transactions;
  common:all_transactions --> common:filtered_transactions;
  common:filtered_actions[map: common:filtered_actions];
  common:filtered_actions:params[params] --> common:filtered_actions;
  common:all_actions --> common:filtered_actions;
  common:filtered_transactions_extra[map: common:filtered_transactions_extra];
  common:filtered_transactions_extra:params[params] --> common:filtered_transactions_extra;
  common:all_transactions --> common:filtered_transactions_extra;
  common:filtered_actions_extra[map: common:filtered_actions_extra];
  common:filtered_actions_extra:params[params] --> common:filtered_actions_extra;
  common:all_actions --> common:filtered_actions_extra;
```

### Modules

```yaml
Package name: antelope_bps
Version: v0.3.0
Doc: Antelope Block Producer claim rewards (Block Pay & Vote Pay).
Modules:
----
Name: map_bps
Initial block: 0
Kind: map
Input: map: common:filtered_transactions
Output Type: proto:antelope.bps.v1.Bps
Hash: a7dbfa1576c3ff4035ad0bdeb2e932e5029c9c6a

```
