#!/usr/bin/env bash

# [ This is for regular benchmarks, 20-30 characters long is the average secure size for most
#   modern websites, although this number really was arbitrarily chosen. Typical entropy at this
#   size is high enough for general use without stress testing the program. ] 
GENERAL_STRING_SIZE=30

if ! [[ -f ./target/release/genpassrs ]];then
  printf "error: binary does not does not exist!\n"
  printf "creating binary..." 
  cargo build --release
fi

printf "[ Beginning normal tests... ]" 

# [ Generate a benchmark performance -- requires hyperfine ]
hyperfine "./target/release/genpassrs string -l $GENERAL_STRING_SIZE"
sleep 0.8

# [ Generate a flamegraph -- requires cargo-flamegraph]
cargo flamegraph -o genpass_string"$GENERAL_STRING_SIZE".svg --bin genpassrs -- string -l "$GENERAL_STRING_SIZE" > /dev/null 2>/dev/null
sleep 0.8

# [ Stress testing-- max string size;]
printf "[ Beginning stress test... ]" 
STRESS_TEST_STRING_SIZE=255

# [ test performance ]
hyperfine "./target/release/genpassrs string -l $STRESS_TEST_STRING_SIZE"
sleep 0.8

# [ check flamegraph ]
cargo flamegraph -o genpass_stress_test.svg --bin genpassrs -- string -l "$STRESS_TEST_STRING_SIZE" > /dev/null 2>/dev/null
sleep 0.8
