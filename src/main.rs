// src/main.rs
/*
 * Main executable for BlockchainNFTRegistryLabs
 */

use clap::Parser;
use blockchainnftregistrylabs::{Result, run};

#[derive(Parser)]
#[command(version, about = "BlockchainNFTRegistryLabs - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
