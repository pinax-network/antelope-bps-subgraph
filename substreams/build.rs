fn main() {
    substreams_antelope::Abigen::new("Contract", "abi/eosio.token.abi.json")
        .expect("failed to load eosio.token abi")
        .generate()
        .expect("failed to generate eosio.token contract code")
        .write_to_file("src/abi/eosio_token.rs")
        .expect("failed to write eosio.token contract code");
    // commented for now - see https://github.com/pinax-network/substreams-antelope/issues/24
    // substreams_antelope::Abigen::new("Contract", "abi/eosio.abi.json")
    //     .expect("failed to load eosio abi")
    //     .generate()
    //     .expect("failed to generate eosio contract code")
    //     .write_to_file("src/abi/eosio.rs")
    //     .expect("failed to write eosio contract code");
}
