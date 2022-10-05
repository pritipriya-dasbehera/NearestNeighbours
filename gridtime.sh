#!/bin/bash
# This is to find time for different particle number at the best cell per axis 
trap "exit" INT
n=( 1000 2500 5000 7500 10000 )
g=( 9 13 16 19 20 )
for i in {0..4}
do
    for j in 1 2 3
    do
        sed -i "11 c\const NUM_OF_PARTICLES: usize = ${n[$i]};" src/main.rs
        sed -i "14 c\const GRID_SIZE: usize = ${g[$i]};" src/main.rs
        sed -i "15 c\const GRID_LEN: f64 = 1.0/${g[$i]}.0;" src/main.rs
        timeout 40s cargo run
        # echo ${n[$i]}
    done
done