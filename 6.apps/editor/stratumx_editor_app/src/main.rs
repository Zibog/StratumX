#![deny(unused_imports)]
#![deny(unused_variables)]
#![warn(dead_code)]

use stratumx_editor_app::{app_runtime::AppRuntime, shell_bootstrap::ShellBootstrap};

fn main() {
    // Phase 1: Bootstrap
    let mut bootstrap = ShellBootstrap::new();
    let host = match bootstrap.bootstrap() {
        Ok(h) => h,
        Err(e) => {
            eprintln!("\n✗ Bootstrap failed: {}", e);
            std::process::exit(1);
        }
    };

    println!("\n✅ Editor spine ready");

    // Phase 2: Determine run mode
    let args: Vec<String> = std::env::args().collect();

    if args.iter().any(|arg| arg == "--headless") {
        // Headless mode - run update loop for testing
        let mut runtime = AppRuntime::new(host);
        let max_frames = args
            .iter()
            .position(|arg| arg == "--frames")
            .and_then(|i| args.get(i + 1))
            .and_then(|s| s.parse::<u64>().ok());

        if let Err(e) = runtime.run_headless(max_frames) {
            eprintln!("\n✗ Runtime failed: {}", e);
            std::process::exit(1);
        }

        println!("\n✅ Headless run complete");
    } else if args.iter().any(|arg| arg == "--gui") {
        #[cfg(feature = "desktop")]
        {
            println!("\n🖥️  Launching desktop GUI...");
            stratumx_editor_app::desktop_app::run_desktop_app();
        }
        #[cfg(not(feature = "desktop"))]
        {
            eprintln!("\n✗ Desktop feature not enabled");
            std::process::exit(1);
        }
    } else {
        // Default: run quick validation
        println!("\n🔍 Running validation...");
        let mut runtime = AppRuntime::new(host);

        // Run 3 frames to validate update loop
        if let Err(e) = runtime.run_headless(Some(3)) {
            eprintln!("\n✗ Validation failed: {}", e);
            std::process::exit(1);
        }

        println!("\n✅ Validation complete");
        println!("\nUsage:");
        println!("  cargo run -p stratumx_editor_app --features desktop -- --gui");
        println!("  cargo run -p stratumx_editor_app -- --headless --frames 60");
        println!("  cargo run -p stratumx_editor_app");
        println!("    runs quick validation only");
    }
}
