// Tool Session Recovery Integration
//
// Integrates recovery strategies into tool session executors.

/// Error classification for recovery
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorClass {
    ResourceNotFound,
    InvalidParameters,
    ExternalDependencyFailure,
    Timeout,
    Unknown,
}

/// Execute operation with retry logic
pub fn execute_with_recovery<F, T>(operation: &str, max_retries: u32, mut f: F) -> Result<T, String>
where
    F: FnMut() -> Result<T, (String, ErrorClass)>,
{
    let mut retry_count = 0;

    loop {
        match f() {
            Ok(result) => return Ok(result),
            Err((error_message, error_class)) => {
                if retry_count >= max_retries {
                    return Err(format!(
                        "{} (max retries {} exceeded)",
                        error_message, max_retries
                    ));
                }

                match error_class {
                    ErrorClass::ExternalDependencyFailure | ErrorClass::Timeout => {
                        eprintln!(
                            "[RECOVERY][{}] Retry {}/{}: {}",
                            operation,
                            retry_count + 1,
                            max_retries,
                            error_message
                        );
                        std::thread::sleep(std::time::Duration::from_millis(1000));
                        retry_count += 1;
                        continue;
                    }
                    ErrorClass::ResourceNotFound
                    | ErrorClass::InvalidParameters
                    | ErrorClass::Unknown => {
                        return Err(error_message);
                    }
                }
            }
        }
    }
}
