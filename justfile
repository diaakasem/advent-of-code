set export

@day n:
  cargo run --package day_$n -- --data $PWD/day_$n/input.txt

@test n:
  cargo test --package day_$n


test_rucksacks:
  cargo test --package rucksacks
