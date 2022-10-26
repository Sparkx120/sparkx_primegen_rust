// Crate
use clap::Parser;

// Local Module
use sparkx_primegen::{driver, common::Config};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The maximum number to search for primes under
    #[arg(short='e', long, default_value_t = 1000000)]
    range_end: u64
}

fn main() {
    let args = Args::parse();
 
    let config = Config::build(args.range_end);

    driver::run(config);
}
