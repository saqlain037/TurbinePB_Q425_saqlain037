use clap::Parser;

/// Simple counter: prints numbers from 1..=n
#[derive(Parser, Debug)]
struct Args {
    /// Count up to this number
    #[arg(short, long, default_value_t = 10)]
    to: u64,
}

fn main() {
    let args = Args::parse();
    for i in 1..=args.to {
        println!("{i}");
    }
}