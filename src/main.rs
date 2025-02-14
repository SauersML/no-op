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

    /// Runs the given command and returns its exit status and elapsed duration.
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
    fn compare_noop_true_and_colon() {
        // Get the path to our own binary.
        let own_binary = env::var("CARGO_BIN_EXE_no-op")
            .unwrap_or_else(|_| "target/release/no-op".to_string());

        let mut durations_own = Vec::with_capacity(ITERATIONS);
        let mut durations_true = Vec::with_capacity(ITERATIONS);
        let mut durations_colon = Vec::with_capacity(ITERATIONS);

        // Time our own binary ITERATIONS times.
        for _ in 0..ITERATIONS {
            let (status, duration) = time_command(Command::new(&own_binary));
            assert!(status.success(), "Own binary did not execute successfully");
            durations_own.push(duration);
        }

        // Time the "true" command ITERATIONS times.
        for _ in 0..ITERATIONS {
            let (status, duration) = time_command(Command::new("true"));
            assert!(status.success(), "'true' command did not execute successfully");
            durations_true.push(duration);
        }

        // Time the colon command ":" via shell ITERATIONS times.
        for _ in 0..ITERATIONS {
            let cmd_colon = {
                let mut c = Command::new("sh");
                c.arg("-c").arg(":");
                c
            };
            let (status, duration) = time_command(cmd_colon);
            assert!(status.success(), "':' command did not execute successfully");
            durations_colon.push(duration);
        }

        // Compute statistics.
        let (mean_own, stddev_own) = compute_stats(&durations_own);
        let (mean_true, stddev_true) = compute_stats(&durations_true);
        let (mean_colon, stddev_colon) = compute_stats(&durations_colon);

        println!("Over {} iterations:", ITERATIONS);
        println!("Own binary:     mean = {:.6} s, stddev = {:.6} s", mean_own, stddev_own);
        println!("Coreutils true: mean = {:.6} s, stddev = {:.6} s", mean_true, stddev_true);
        println!("Colon command:  mean = {:.6} s, stddev = {:.6} s", mean_colon, stddev_colon);
    }
}
