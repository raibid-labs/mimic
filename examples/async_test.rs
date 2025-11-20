//! Async/Tokio testing example.
//!
//! This demonstrates async testing patterns with Tokio runtime.
//! Full async implementation will be completed in Phase 2.

use portable_pty::CommandBuilder;
use term_test::{Result, TuiTestHarness};

#[tokio::main]
async fn main() -> Result<()> {
    println!("Async testing example with Tokio");

    // For now, we use the sync harness
    // Phase 2 will implement AsyncTuiTestHarness
    let mut harness = TuiTestHarness::new(80, 24)?;

    let mut cmd = CommandBuilder::new("echo");
    cmd.arg("Async test");

    harness.spawn(cmd)?;

    // Simulate async wait
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    harness.update_state()?;
    let contents = harness.screen_contents();

    println!("\nScreen contents:");
    println!("{}", contents);

    println!("\nNote: Full async/await API will be implemented in Phase 2");

    Ok(())
}
