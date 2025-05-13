#!/bin/bash
# get times from command line argument
TIMES=${1:-1}
UFs=(
    # "rr"
    "ap"
    # "ac"
    # "ro"
    # "am"
    # "se"
    # "es"
    # "ms"
    # "rj"
    # "al"
    # "to"
    # "mt"
    # "pa"
    # "rn"
    # "pe"
    # "ce"
    # "pb"
    # "ma"
    # "pi"
    # "go"
    # "sc"
    # "pr"
    # "ba"
    # "rs"
    # "sp"
    # "mg"
)

for uf in "${UFs[@]}"
do
    for ((i=1; i<=TIMES; i++))
    do
        echo "Running test $i for UF: $uf"
        # RUST_BACKTRACE=1 cargo run -q -- --algorithm BB -u "$uf"  
        RUSTFLAGS="-Awarnings" cargo run -q -- --algorithm BB -u "$uf" -p
    done
done
