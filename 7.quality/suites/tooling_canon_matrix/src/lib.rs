// integration-test only harness

// __stratumx_ready_smoke_test__
#[cfg(test)]
mod __ready_smoke_tests {
    #[test]
    fn crate_smoke() {
        assert_eq!(env!("CARGO_PKG_NAME"), "tooling_canon_matrix");
    }
}
