#!/bin/bash

TIMES=10
UFs=(
    # "rr"
    # "ap"
    # "ac"
    # "ro"
    "am"
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
    "sc"
    "pr"
    # "ba"
    # "rs"
    "sp"
    "mg"
)

ALG="SA"
VARIANT="greedy" # ou greedy

for uf in "${UFs[@]}"
do
    for ((i=1; i<=TIMES; i++))
    do
        DIR="src/assets/outputs/$ALG/$uf"
        FILE="$DIR/${i}_${VARIANT}.txt"
        
        mkdir -p "$DIR"  # Cria o diretório se não existir
    done
done

for uf in "${UFs[@]}"
do
    for ((i=1; i<=TIMES; i++))
    do
        DIR="src/assets/outputs/$ALG/$uf"
        FILE="$DIR/${i}_${VARIANT}.txt"
        
        mkdir -p "$DIR"  # Cria o diretório se não existir
        
        date > "$FILE"
        echo "Running test $i for UF: $uf with algorithm: $ALG" | tee -a "$FILE"
        RUSTFLAGS="-Awarnings" cargo run -q -- -a "$ALG" -u "$uf" -p | tee -a "$FILE"
        date >> "$FILE"
    done
done
