# Differential Evolution

A quick and dirty implementation of the Differential Evolution algorithm.

## Performance

Quite garbage. As mentioned above, it was haphazardly thrown together with little
regards for efficency. 

## How to run

- Requirements
    - Cargo
    - The rest of the Rust toolchain
    - A computer (Obviously)
    - An internet connection (To download dependencies, otherwise not required)

- Compilation instructions
    - Clone the repo
    - Cd into it
    - `cargo build` (Optionally append the --release flag for better runtime
      performance)

- Execution intruction:
    - `cargo run <population count> <differential weight> <cross compatibility>`

- How to interpret the output
    - The program will output `GENERATIONS` lines to stdout
    - each line contains that generation's fittest candidate's fitness
    - Plug that into your favourite data visualisation software
    
## Special thanks

- Wikipedia, for their [excellent article](https://en.wikipedia.org/wiki/Differential_evolution)
  on the topic.
- People behind the [rust-random](https://github.com/rust-random) organization,
  for the [random number generation library](https://crates.io/crates/rand) used in the project.
