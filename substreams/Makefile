.PHONY: all
all:
	make protogen
	make build
	make pack
	make graph
	make info

.PHONY: protogen
protogen:
	substreams protogen --exclude-paths sf/substreams,google

.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release

.PHONY: pack
pack:
	substreams pack

.PHONY: graph
graph:
	substreams graph

.PHONY: info
info:
	substreams info

.PHONY: run
run:
	substreams run -e eos.substreams.pinax.network:443 map_bps -s 1000000 -t 1100000 --production-mode

.PHONY: gui
gui:
	substreams gui -e eos.substreams.pinax.network:443 map_bps -s 1000000 -t 1100000 --production-mode