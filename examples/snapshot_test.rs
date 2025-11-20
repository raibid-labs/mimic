//! Snapshot testing example using insta.
//!
//! This demonstrates how to use term-test with the insta snapshot testing library.

use portable_pty::CommandBuilder;
use term_test::{Result, TuiTestHarness};

fn main() -> Result<()> {
    println!("Snapshot testing example");

    let mut harness = TuiTestHarness::new(80, 24)?;

    // Spawn a command that produces predictable output
    let mut cmd = CommandBuilder::new("echo");
    cmd.arg("Snapshot test output");

    harness.spawn(cmd)?;

    // Wait for output
    std::thread::sleep(std::time::Duration::from_millis(100));
    harness.update_state()?;

    let contents = harness.screen_contents();

    println!("\nCaptured screen contents:");
    println!("{}", contents);

    // In a real test, you would use insta::assert_snapshot!
    // insta::assert_snapshot!(contents);

    println!("\nNote: In a real test, use insta::assert_snapshot! to capture this output");

    Ok(())
}
