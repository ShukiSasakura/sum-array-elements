#!/bin/bash

echo "table:result"
for t in $(seq 1 30)
do
    echo -en "\t$t\t"
    wasmer target/wasm32-wasmer-wasi/release/sum-array-elements.wasm -- -l 1000000 -c 1000 -t $t -r 100000
done
