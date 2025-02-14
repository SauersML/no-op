// src/main.rs

fn main() {
    // Do nothing.
}

#[cfg(test)]
mod tests {
    use std::env;
    use std::process::Command;
    use std::time::{Duration, Instant};

    const ITERATIONS: usize = 100;
    const WARMUP_ITERATIONS: usize = 100;

    /// Runs the given command (by value) and returns its exit status and elapsed duration.
    fn time_command(mut cmd: Command) -> (std::process::ExitStatus, Duration) {
        let start = Instant::now();
        let status = cmd.status().expect("Failed to execute command");
        let elapsed = start.elapsed();
        (status, elapsed)
    }

    /// Computes the mean and standard deviation (in seconds) for a slice of Durations.
    fn compute_stats(durations: &[Duration]) -> (f64, f64) {
        let n = durations.len() as f64;
        let sum: f64 = durations.iter().map(|d| d.as_secs_f64()).sum();
        let mean = sum / n;
        let variance = durations
            .iter()
            .map(|d| {
                let diff = d.as_secs_f64() - mean;
                diff * diff
            })
            .sum::<f64>()
            / n;
        (mean, variance.sqrt())
    }

    #[test]
    fn compare_commands() {
        // Determine the path to our own binary.
        let own_binary = env::var("CARGO_BIN_EXE_no-op")
            .unwrap_or_else(|_| "target/release/no-op".to_string());

        // Warmup each command to remove startup bias.
        for _ in 0..WARMUP_ITERATIONS {
            let _ = Command::new(&own_binary).status();
        }
        for _ in 0..WARMUP_ITERATIONS {
            let _ = Command::new("true").status();
        }
        for _ in 0..WARMUP_ITERATIONS {
            let mut warmup_colon = Command::new("sh");
            warmup_colon.arg("-c").arg(":");
            let _ = warmup_colon.status();
        }
        for _ in 0..WARMUP_ITERATIONS {
            let mut warmup_rm = Command::new("rm");
            warmup_rm.arg("-f").arg("/nonexistent-file");
            let _ = warmup_rm.status();
        }

        // Vectors to record the durations.
        let mut durations_own = Vec::with_capacity(ITERATIONS);
        let mut durations_true = Vec::with_capacity(ITERATIONS);
        let mut durations_colon = Vec::with_capacity(ITERATIONS);
        let mut durations_rm = Vec::with_capacity(ITERATIONS);

        // Time our own binary.
        for _ in 0..ITERATIONS {
            let (status, duration) = time_command(Command::new(&own_binary));
            assert!(status.success(), "Own binary did not execute successfully");
            durations_own.push(duration);
        }

        // Time the "true" command.
        for _ in 0..ITERATIONS {
            let (status, duration) = time_command(Command::new("true"));
            assert!(status.success(), "'true' command did not execute successfully");
            durations_true.push(duration);
        }

        // Time the colon command (":") via shell.
        for _ in 0..ITERATIONS {
            let mut cmd_colon = Command::new("sh");
            cmd_colon.arg("-c").arg(":");
            let (status, duration) = time_command(cmd_colon);
            assert!(status.success(), "':' command did not execute successfully");
            durations_colon.push(duration);
        }

        // Time the rm command (rm -f /nonexistent-file).
        for _ in 0..ITERATIONS {
            let mut cmd_rm = Command::new("rm");
            cmd_rm.arg("-f").arg("/nonexistent-file");
            let (status, duration) = time_command(cmd_rm);
            assert!(status.success(), "'rm' command did not execute successfully");
            durations_rm.push(duration);
        }

        // Compute statistics.
        let (mean_own, stddev_own) = compute_stats(&durations_own);
        let (mean_true, stddev_true) = compute_stats(&durations_true);
        let (mean_colon, stddev_colon) = compute_stats(&durations_colon);
        let (mean_rm, stddev_rm) = compute_stats(&durations_rm);

        println!("Over {} iterations:", ITERATIONS);
        println!("Own binary:     mean = {:.6} s, stddev = {:.6} s", mean_own, stddev_own);
        println!("Coreutils true: mean = {:.6} s, stddev = {:.6} s", mean_true, stddev_true);
        println!("Colon command:  mean = {:.6} s, stddev = {:.6} s", mean_colon, stddev_colon);
        println!("Rm command:     mean = {:.6} s, stddev = {:.6} s", mean_rm, stddev_rm);
    }
}
