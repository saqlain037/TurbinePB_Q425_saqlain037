# TurbinePB_Q425_saqlain037

# TurbinePB Project

This repository contains multiple Rust and Solana projects:

## Projects

### 1. Solana Counter Program
A Solana program implementing a simple counter with the following features:
- Initialize counter to zero
- Increment counter by specified value
- Decrement counter with underflow protection
- Located in `/counter` directory

### 2. Rust CLI Applications
Collection of command-line applications in `/rust-cli`:

#### Counter CLI
- Simple command-line counter that prints numbers from 1 to N
- Uses Clap for argument parsing
- Customizable upper limit via `-t/--to` flag

#### RNG Game
- Number guessing game implementation
- Random number generation between 1-100
- Interactive command-line interface
- Provides feedback on guess attempts

#### Hello World
- Basic "Hello, world!" program example

## Building and Running

### Solana Counter
```bash
# Navigate to counter directory
cd counter

# Build the program
anchor build

# Deploy to local test validator
anchor deploy

# Run tests
anchor test

Project Structure
.
├── counter/               # Solana counter program
│   ├── programs/         # Program source code
│   ├── tests/           # Integration tests
│   └── migrations/      # Deployment scripts
└── rust-cli/            # Rust CLI applications
    ├── counter/         # Counter CLI
    ├── rng_game/       # Number guessing game
    └── hello_world/    # Hello world example

Dependencies
Rust
Solana Tool Suite
Anchor Framework
Various Rust crates including:
clap
rand
anchor-lang
