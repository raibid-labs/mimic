//! Basic usage example of term-test.
//!
//! This demonstrates the core functionality of spawning a process in a PTY,
//! sending input, and capturing screen state.

use portable_pty::CommandBuilder;
use term_test::{Result, TuiTestHarness};

fn main() -> Result<()> {
    println!("Basic term-test example");

    // Create a test harness with 80x24 terminal
    let mut harness = TuiTestHarness::new(80, 24)?;

    println!("Terminal created: 80x24");

    // Spawn a simple command (echo)
    let mut cmd = CommandBuilder::new("echo");
    cmd.arg("Hello from term-test!");

    harness.spawn(cmd)?;
    println!("Process spawned");

    // Give it a moment to output
    std::thread::sleep(std::time::Duration::from_millis(100));

    // Update screen state
    harness.update_state()?;

    // Get screen contents
    let contents = harness.screen_contents();
    println!("\nScreen contents:");
    println!("{}", contents);

    // Check cursor position
    let (row, col) = harness.cursor_position();
    println!("\nCursor position: row={}, col={}", row, col);

    // Wait for process to exit
    let status = harness.wait_exit()?;
    println!("\nProcess exited with status: {:?}", status);

    Ok(())
}
