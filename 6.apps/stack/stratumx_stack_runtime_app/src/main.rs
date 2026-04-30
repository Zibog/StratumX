use clap::Parser;
use engine_core::EngineCoreResult;
use engine_runtime::{RuntimeConfig, RuntimeKernel, RuntimeProfile};

#[derive(Parser, Debug)]
#[command(name = "stratumx_stack_runtime_app")]
#[command(about = "Thin stack runtime entrypoint for full engine+editor stack")]
struct Args {
    /// Run in validation mode
    #[arg(long)]
    validate: bool,

    /// Number of ticks to validate
    #[arg(long, default_value = "10")]
    ticks: u32,

    /// Enable verbose logging
    #[arg(long)]
    verbose: bool,
}

fn main() -> EngineCoreResult<()> {
    let args = Args::parse();

    if args.verbose {
        println!("🚀 StratumX Stack Runtime starting...");
    }

    // Bootstrap: create startup reference seed
    let seed = engine_startup::launch_startup_reference_seed()?;

    // Initialize runtime kernel with seed world
    let config = RuntimeConfig {
        profile: RuntimeProfile::Interactive60,
        max_apply_segments_per_tick: 16,
        publish_passes: 1,
    };
    let mut runtime = RuntimeKernel::new(seed.world, config);

    if args.validate {
        println!("🔍 Running stack validation: {} ticks", args.ticks);
        for tick in 0..args.ticks {
            runtime.run_tick()?;
            if args.verbose && tick % 3 == 0 {
                println!("  Tick {}/{}", tick + 1, args.ticks);
            }
        }
        println!("✅ Stack validation complete: {} ticks", args.ticks);
    } else {
        println!("✅ Stack runtime initialized successfully");
        println!("\nNote: This is a thin host that validates the full stack.");
        println!("Full editor integration requires 5.editor crates.");
        println!("\nUsage:");
        println!("  cargo run -p stratumx_stack_runtime_app -- --validate --ticks 100");
        println!("  cargo run -p stratumx_stack_runtime_app -- --verbose");
    }

    Ok(())
}
