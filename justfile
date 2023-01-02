set export

build:
  cargo build

@day n:
  cargo run --package day_$n -- --data $PWD/day_$n/input.txt

@test p:
  cargo test --package $p
