#!/bin/bash
amount_of_runs="$1";
command="$2";
file="$3";

for ((i = 1; i <= $amount_of_runs; i++))
do 
 $command $file
done