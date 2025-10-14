pub mod decoder;
pub mod elf;
pub mod guest;
pub mod memory;
pub mod registers;

use std::{path::PathBuf, process};

use clap::{Parser, ValueEnum};
use guest::GuestCPU;
use tracing::{Level, error, info};
use tracing_subscriber::{EnvFilter, FmtSubscriber};

/// Risc-V-jit - RISCV jitted emulator.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Arguments {
    /// The target file to be recompiled.
    #[arg(short)]
    target: PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let filter = EnvFilter::new("riscvjit=trace");

    // create a new subscriber for tracing.
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .without_time()
        .with_env_filter(filter)
        .finish();

    // set it so that the newly created subscriber is the global default.
    tracing::subscriber::set_global_default(subscriber)?;

    let args = Arguments::parse();

    let cpu = match GuestCPU::from_elf(args.target) {
        Err(e) => {
            error!("{e}");
            process::exit(-1);
        }
        Ok(f) => f,
    };

    info!("Goodbye, world.");

    Ok(())
}
