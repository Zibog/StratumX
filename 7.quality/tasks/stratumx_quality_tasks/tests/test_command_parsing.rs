use std::process::Command;

fn run_help(args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_stratumx_quality_tasks"))
        .args(args)
        .output()
        .expect("quality tasks binary should run")
}

#[test]
fn root_help_lists_current_commands() {
    let output = run_help(&["--help"]);
    assert!(output.status.success(), "root help should succeed");

    let stdout = String::from_utf8_lossy(&output.stdout);
    for command in [
        "verify",
        "smoke",
        "full",
        "bench",
        "metrics",
        "canon-coverage",
        "evidence",
        "inventory",
    ] {
        assert!(
            stdout.contains(command),
            "root help should list `{command}`"
        );
    }
}

#[test]
fn verify_help_exposes_verbose_flag() {
    let output = run_help(&["verify", "--help"]);
    assert!(output.status.success(), "verify help should succeed");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("--verbose"),
        "verify help should document `--verbose`"
    );
}

#[test]
fn every_current_subcommand_has_help() {
    for subcommand in [
        "verify",
        "smoke",
        "full",
        "bench",
        "metrics",
        "canon-coverage",
        "evidence",
        "inventory",
    ] {
        let output = run_help(&[subcommand, "--help"]);
        assert!(
            output.status.success(),
            "`{subcommand} --help` should succeed"
        );
    }
}
