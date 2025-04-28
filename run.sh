#!/bin/bash

TIMES=5

for ((i=1; i<=TIMES; i++))
do
    cargo run -q -- -u to -p
done
