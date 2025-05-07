#!/bin/bash

TIMES=15
UFs=(
    "rr"
    "ap"
    "ac"
    "ro"
    "am"
    "se"
    "es"
    "ms"
    "rj"
    "al"
    "to"
    "mt"
    "pa"
    "rn"
    "pe"
    "ce"
    "pb"
    "ma"
    "pi"
    "go"
    "sc"
    "pr"
    "ba"
    "rs"
    "sp"
    "mg"
)

for uf in "${UFs[@]}"
do
    for ((i=1; i<=TIMES; i++))
    do
        echo "Running test $i for UF: $uf"
        RUSTFLAGS="-Awarnings" cargo run -q -- -u "$uf" -p
    done
done
