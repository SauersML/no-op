// src/main.rs

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
        // When testing a binary named "no-op", Cargo sets CARGO_BIN_EXE_no-op.
        let own_binary = env::var("CARGO_BIN_EXE_no-op")
            .unwrap_or_else(|_| "target/release/no-op".to_string());

        // Measure the execution time for our own binary.
        let (status_own, duration_own) = time_command(Command::new(&own_binary));
        assert!(status_own.success(), "Own binary did not execute successfully");

        // Measure the execution time for the "true" command.
        let (status_true, duration_true) = time_command(Command::new("true"));
        assert!(status_true.success(), "'true' command did not execute successfully");

        // For the colon command, we must build the command step by step because chaining .arg() calls
        // yields a &mut Command. Create a new Command variable and then pass it by value.
        let cmd_colon = {
            let mut c = Command::new("sh");
            c.arg("-c").arg(":");
            c
        };
        let (status_colon, duration_colon) = time_command(cmd_colon);
        assert!(status_colon.success(), "':' command did not execute successfully");

        println!("Execution times:");
        println!("Own binary:     {:?}", duration_own);
        println!("Coreutils true: {:?}", duration_true);
        println!("Colon command:  {:?}", duration_colon);
    }
}
