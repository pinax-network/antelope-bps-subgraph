# Antelope BP registrations and rewards subgraph

> Subgraph for Antelope Block Producer registrations and rewards.

## Quickstart

```
$ git clone https://github.com/pinax-network/antelope-bps-subgraph
$ cd antelope-bps-subgraph/subgraph
$ yarn install
$ yarn codegen
$ yarn build
```

## Development
```
$ cd graph-node
$ ./up.sh -c
$ cd ..
$ yarn create-local
$ yarn deploy-local
```

Query: [localhost](http://localhost:8000/subgraphs/name/eos-bps/graphql?query=%7B%0A++_meta%7Bblock%7Bnumber%7D%7D%0A++bps%28first%3A+10+orderBy%3ApaidCount+orderDirection%3Adesc%29%7B%0A++++name%0A++++registrations%7B%0A++++++location%0A++++++blockNum%0A++++++timestamp%0A++++++url%0A++++++publicKey%0A++++%7D%0A++++pays%28first%3A5+orderBy%3AblockNum+orderDirection%3Adesc%29%7B%0A++++++quantity%0A++++%7D%0A++++%0A++++paidQuantity%0A++++paidValue%0A++++paidCount%0A++%7D%0A%7D)

