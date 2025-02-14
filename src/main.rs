// main.rs

// The main function for our binary.
// This does nothing.
fn main() {
    // Do nothing.
}

#[cfg(test)]
mod tests {
    use std::env;
    use std::process::Command;
    use std::time::Instant;

    /// Helper function that runs a given command and returns its exit status and elapsed duration.
    fn time_command(mut cmd: Command) -> (std::process::ExitStatus, std::time::Duration) {
        let start = Instant::now();
        let status = cmd.status().expect("Failed to execute command");
        let elapsed = start.elapsed();
        (status, elapsed)
    }

    #[test]
    fn compare_noop_true_and_colon() {
        // Get the path to our own binary.
        // Cargo sets CARGO_BIN_EXE_no-op when running integration tests for a binary named "no-op".
        // If not found, we fallback to a hard-coded path.
        let own_binary = env::var("CARGO_BIN_EXE_no-op")
            .unwrap_or_else(|_| "target/release/no-op".to_string());

        // Measure the execution time for our own binary.
        let (status_own, duration_own) = time_command(Command::new(&own_binary));
        assert!(status_own.success(), "Own binary did not execute successfully");

        // Measure the execution time for the "true" command.
        let (status_true, duration_true) = time_command(Command::new("true"));
        assert!(status_true.success(), "'true' command did not execute successfully");

        // The colon command ":" is a shell built-in that does nothing.
        // We invoke it via a shell.
        let (status_colon, duration_colon) = time_command(
            Command::new("sh")
                .arg("-c")
                .arg(":")
        );
        assert!(status_colon.success(), "':' command did not execute successfully");

        println!("Execution times:");
        println!("Own binary:     {:?}", duration_own);
        println!("Coreutils true: {:?}", duration_true);
        println!("Colon command:  {:?}", duration_colon);
    }
}
