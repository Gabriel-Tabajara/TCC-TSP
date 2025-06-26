#!/bin/bash

<<<<<<< HEAD
TIMES=2
=======
TIMES=1
>>>>>>> ant-colony-optimization
UFs=(
    # "rr"
    # "ap"
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
<<<<<<< HEAD
    # "sc"
=======
    "sc"
>>>>>>> ant-colony-optimization
    # "pr"
    # "ba"
    # "rs"
    # "sp"
    # "mg"
<<<<<<< HEAD
    "brazil"
)

ALG="SA"
VARIANT="generic" # ou greedy
=======
    # "brazil"
)

ALG="ACO"
VARIANT="candidate_list3" # ou greedy
>>>>>>> ant-colony-optimization

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
