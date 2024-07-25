# Antelope Claim Rewards

> Substreams for Antelope Block Producer claim rewards (Block Pay & Vote Pay).

## Quickstart

```
$ gh repo clone pinax-network/substreams-antelope-claimrewards
$ cd substreams-antelope-claimrewards
$ make
$ make gui
```

## Releases

- https://github.com/pinax-network/substreams-antelope-claimrewards/releases

### Mermaid Graph

```mermaid
graph TD;
  map_pays[map: map_pays];
  common:filtered_transactions --> map_pays;
  graph_out[map: graph_out];
  map_pays --> graph_out;
  common:all_actions --> common:index_actions;
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
```

### Modules

```yaml
Package name: antelope_claimrewards
Version: v0.1.0
Doc: Antelope Block Producer claim rewards (Block Pay & Vote Pay).
Modules:
----
Name: map_pays
Initial block: 0
Kind: map
Input: map: common:filtered_transactions
Output Type: proto:antelope.bps.v1.Pays
Hash: b64fab3af9bcc4bace45dc10a0cc8c496811413e

Name: graph_out
Initial block: 0
Kind: map
Input: map: map_pays
Output Type: proto:sf.substreams.sink.entity.v1.EntityChanges
Hash: 80655d596a546ac022f6ec5ff634a85e9216d7d3

```
