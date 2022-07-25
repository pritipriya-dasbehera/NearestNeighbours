#!/bin/bash
# This is to find time for different particle number at a range of cell per axis 
trap "exit" INT
for n in 900
do
    for i in {21..25}
    do
        for j in 1 2 3
        do
            sed -i "11 c\const NUM_OF_PARTICLES: usize = $n;" src/main.rs
            sed -i "14 c\const GRID_SIZE: usize = $i;" src/main.rs
            sed -i "15 c\const GRID_LEN: f64 = 1.0/$i.0;" src/main.rs
            timeout 10s cargo run
        done
    done
done