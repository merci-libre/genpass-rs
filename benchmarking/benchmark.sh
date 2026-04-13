#!/usr/bin/env bash

# This is for regular benchmarks, 20-30 characters long is the average secure size for most
# modern websites, although this number really was arbitrarily chosen. Typical entropy at this
# size is high enough for general use without stress testing the program.
GENERAL_STRING_SIZE=30

BINPATH=../target/release/genpassrs

# This tests the program at maximum string size.
STRESS_TEST_STRING_SIZE=255

function check_binary(){
  if ! [[ -f $BINPATH ]];then
    printf "error: binary does not does not exist!\n"
    printf "creating binary..." 
    cargo build --release
  fi
}

function benchmarks(){
    # [ Generate a benchmark performance -- requires hyperfine ]
  if ! hyperfine "$BINPATH string -l $1" 2> /dev/null;then
    printf "You don't have hyperfine installed. No metrics will be generated.\n"
    exit 1
  fi

  # [ Generate a flamegraph -- requires cargo-flamegraph]
  if ! cargo flamegraph -o "$2".svg --bin genpassrs -- string -l "$1" > /dev/null 2>/dev/null;then
    printf "You don't have cargo flamegraph installed. No flamegraph will be generated.\n"
  fi

}


function main(){
  check_binary

  printf "Performing normal tests...\n" 
  benchmarks $GENERAL_STRING_SIZE "genpass-flamegraph-normal"

  printf "Performing stress tests...\n" 
  benchmarks $STRESS_TEST_STRING_SIZE "genpass-flamegraph-stress-test"
}
main

