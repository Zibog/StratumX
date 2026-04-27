use clap::Parser;
use engine_core::EngineCoreResult;
use engine_runtime::{RuntimeKernel, RuntimeConfig, RuntimeProfile};

#[derive(Parser, Debug)]
#[command(name = "stratumx_engine_headless_app")]
#[command(about = "Thin headless engine entrypoint for server/simulation")]
struct Args {
    /// Run in benchmark mode with fixed tick count
    #[arg(long)]
    benchmark: bool,

    /// Number of ticks to run in benchmark mode
    #[arg(long, default_value = "100")]
    ticks: u32,

    /// Enable verbose logging
    #[arg(long)]
    verbose: bool,
}

fn main() -> EngineCoreResult<()> {
    let args = Args::parse();

    if args.verbose {
        println!("🚀 StratumX Headless Engine starting...");
    }

    // Bootstrap: create startup reference seed
    let seed = engine_startup::launch_startup_reference_seed()?;

    // Initialize headless runtime with seed world
    let config = RuntimeConfig {
        profile: RuntimeProfile::Headless20,
        max_apply_segments_per_tick: 16,
        publish_passes: 1,
    };
    let mut runtime = RuntimeKernel::new(seed.world, config);

    if args.benchmark {
        println!("📊 Running benchmark: {} ticks", args.ticks);
        for tick in 0..args.ticks {
            runtime.run_tick()?;
            if args.verbose && tick % 10 == 0 {
                println!("  Tick {}/{}", tick + 1, args.ticks);
            }
        }
        println!("✅ Benchmark complete: {} ticks executed", args.ticks);
    } else {
        // Default: run validation smoke
        println!("🔍 Running validation smoke...");
        runtime.run_tick()?;
        runtime.run_tick()?;
        runtime.run_tick()?;
        println!("✅ Validation smoke complete");
        println!("\nUsage:");
        println!("  cargo run -p stratumx_engine_headless_app -- --benchmark --ticks 1000");
        println!("  cargo run -p stratumx_engine_headless_app -- --verbose");
    }

    Ok(())
}
