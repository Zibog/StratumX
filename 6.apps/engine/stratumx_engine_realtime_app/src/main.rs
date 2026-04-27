use clap::Parser;
use engine_core::EngineCoreResult;
use engine_runtime::{RuntimeKernel, RuntimeConfig, RuntimeProfile};

#[derive(Parser, Debug)]
#[command(name = "stratumx_engine_realtime_app")]
#[command(about = "Thin realtime engine entrypoint for client/presentation")]
struct Args {
    /// Run in headless validation mode
    #[arg(long)]
    headless: bool,

    /// Number of frames to validate in headless mode
    #[arg(long, default_value = "10")]
    frames: u32,

    /// Enable verbose logging
    #[arg(long)]
    verbose: bool,
}

fn main() -> EngineCoreResult<()> {
    let args = Args::parse();

    if args.verbose {
        println!("🚀 StratumX Realtime Engine starting...");
    }

    // Bootstrap: create startup reference seed
    let seed = engine_startup::launch_startup_reference_seed()?;

    // Initialize realtime runtime with seed world
    let config = RuntimeConfig {
        profile: RuntimeProfile::Interactive60,
        max_apply_segments_per_tick: 16,
        publish_passes: 1,
    };
    let mut runtime = RuntimeKernel::new(seed.world, config);

    if args.headless {
        println!("🔍 Running headless validation: {} frames", args.frames);
        for frame in 0..args.frames {
            runtime.run_tick()?;
            if args.verbose && frame % 3 == 0 {
                println!("  Frame {}/{}", frame + 1, args.frames);
            }
        }
        println!("✅ Headless validation complete: {} frames", args.frames);
    } else {
        // Default: show usage (full GUI runtime would go here)
        println!("✅ Runtime initialized successfully");
        println!("\nNote: Full GUI runtime requires eframe/egui integration.");
        println!("This is a thin host that validates the runtime stack.");
        println!("\nUsage:");
        println!("  cargo run -p stratumx_engine_realtime_app -- --headless --frames 100");
        println!("  cargo run -p stratumx_engine_realtime_app -- --verbose");
    }

    Ok(())
}
