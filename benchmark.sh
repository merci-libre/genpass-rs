#!/usr/bin/env bash

if ! [[ -f ./target/release/genpassrs ]];then
  printf "error: binary does not does not exist!\n"
  printf "creating binary..." 
  cargo build --release
fi

# [ Generate a benchmark performance -- requires hyperfine ]
hyperfine "./target/release/genpassrs string -l 30"

# [ Generate a flamegraph -- requires cargo-flamegraph]
cargo flamegraph --bin genpassrs -- string -l 255

