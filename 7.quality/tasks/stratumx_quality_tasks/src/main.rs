mod tasks;

use clap::{Parser, Subcommand};
use std::process::ExitCode;
use tasks::common::QualityContext;

#[derive(Parser)]
#[command(name = "stratumx_quality_tasks")]
#[command(about = "Single entry point for StratumX quality verification")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Verify {
        #[arg(long)]
        verbose: bool,
    },
    Smoke,
    Full,
    Bench,
    Metrics,
    CanonCoverage,
    Evidence,
    Inventory,
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    let ctx = QualityContext::discover();

    let result = match cli.command {
        Commands::Verify { verbose } => tasks::verify::run(&ctx, verbose),
        Commands::Smoke => tasks::smoke::run(&ctx),
        Commands::Full => tasks::full::run(&ctx),
        Commands::Bench => tasks::bench::run(&ctx),
        Commands::Metrics => tasks::metrics::run(&ctx),
        Commands::CanonCoverage => tasks::canon_coverage::run(&ctx),
        Commands::Evidence => tasks::evidence::run(&ctx),
        Commands::Inventory => tasks::inventory::run(&ctx),
    };

    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("{err}");
            ExitCode::from(1)
        }
    }
}
