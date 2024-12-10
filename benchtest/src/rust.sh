#!/bin/bash
amount_of_runs="$1";
file="$2";

for ((i = 1; i <= $amount_of_runs; i++))
do 
 $file
done