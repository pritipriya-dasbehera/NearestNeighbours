#!/bin/bash
# This is to find time for different particle number at the best cell per axis 
trap "exit" INT
n=( 1000 5000 10000 25000 50000 75000 100000 )
g=( 9 16 20 28 35 40 44 )
for i in {0..6}
do
    for j in 1 2 3
    do
        sed -i "11 c\const NUM_OF_PARTICLES: usize = ${n[$i]};" src/main.rs
        sed -i "14 c\const GRID_SIZE: usize = ${g[$i]};" src/main.rs
        sed -i "15 c\const GRID_LEN: f64 = 1.0/${g[$i]}.0;" src/main.rs
        timeout 20s cargo run
        # echo ${n[$i]}
    done
done