# Advent of Code 2020

Use `cargo-aoc` to run:
```
cargo install cargo-aoc
cargo aoc -d [day] -p [part]
```

# Tricks

## Recap

Parse a string into a struct using a Regex
```rust
use recap::Recap; 
use serde::Deserialize;

#[derive(Debug, Deserialize, Recap)]
#[recap(regex = r"^(?P<min>\d+)-(?P<max>\d+) (?P<chr>[a-z]): (?P<word>[a-z]+) / (?P<csv>.*?)$")]
pub struct MyStruct {
    min: usize,
    max: usize,
    chr: char,
    word: String,
    csv: Vec<String>,  // supports parsing comma separated values as a Vec<T>
}

fn main() {
    let my_struct = "01-99 x: hello / I,am,a,Vec".parse::<MyStruct>();
    println!("{:?}", my_struct)
}
```
