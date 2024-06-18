.PHONY: all
all:
	make build
	make pack
	make graph
	make info

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
	substreams run -e eos.substreams.pinax.network:443 graph_out -s -80000 --production-mode

.PHONY: gui
gui:
	substreams gui -e eos.substreams.pinax.network:443 graph_out -s -80000 --production-mode