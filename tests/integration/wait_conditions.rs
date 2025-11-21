//! Integration tests for wait condition functionality.
//!
//! These tests verify that the wait condition APIs work correctly with real
//! processes and terminal output.

use portable_pty::CommandBuilder;
use std::time::Duration;
use term_test::{Result, TermTestError, TuiTestHarness};

#[test]
fn test_wait_for_text_with_echo() -> Result<()> {
    let mut harness = TuiTestHarness::new(80, 24)?
        .with_timeout(Duration::from_secs(2))
        .with_poll_interval(Duration::from_millis(50));

    let mut cmd = CommandBuilder::new("echo");
    cmd.arg("Hello from echo!");
    harness.spawn(cmd)?;

    // Wait for the output
    harness.wait_for_text("Hello")?;
    harness.wait_for_text("echo")?;

    let contents = harness.screen_contents();
    assert!(contents.contains("Hello from echo!"));

    Ok(())
}

#[test]
fn test_wait_for_text_multiline() -> Result<()> {
    let mut harness = TuiTestHarness::new(80, 24)?
        .with_timeout(Duration::from_secs(3))
        .with_poll_interval(Duration::from_millis(50));

    let mut cmd = CommandBuilder::new("sh");
    cmd.arg("-c");
    cmd.arg("printf 'First\\nSecond\\nThird\\n'");
    harness.spawn(cmd)?;

    // Wait for each line
    harness.wait_for_text("First")?;
    harness.wait_for_text("Second")?;
    harness.wait_for_text("Third")?;

    let contents = harness.screen_contents();
    assert!(contents.contains("First"));
    assert!(contents.contains("Second"));
    assert!(contents.contains("Third"));

    Ok(())
}

#[test]
fn test_wait_for_text_timeout_behavior() {
    let mut harness = TuiTestHarness::new(80, 24)
        .unwrap()
        .with_timeout(Duration::from_millis(500))
        .with_poll_interval(Duration::from_millis(50));

    let mut cmd = CommandBuilder::new("sleep");
    cmd.arg("5");
    harness.spawn(cmd).unwrap();

    // This should timeout
    let result = harness.wait_for_text("this_text_never_appears");
    assert!(result.is_err());

    match result {
        Err(TermTestError::Timeout { timeout_ms }) => {
            assert_eq!(timeout_ms, 500);
        }
        _ => panic!("Expected Timeout error"),
    }
}

#[test]
fn test_wait_for_text_with_custom_timeout() -> Result<()> {
    let mut harness = TuiTestHarness::new(80, 24)?;

    let mut cmd = CommandBuilder::new("echo");
    cmd.arg("fast output");
    harness.spawn(cmd)?;

    // Use a very short custom timeout for fast commands
    harness.wait_for_text_timeout("fast", Duration::from_millis(800))?;

    assert!(harness.screen_contents().contains("fast output"));
    Ok(())
}

#[test]
fn test_wait_for_cursor_movement() -> Result<()> {
    let mut harness = TuiTestHarness::new(80, 24)?
        .with_timeout(Duration::from_secs(2));

    // Manually feed cursor movement sequences
    harness.state_mut().feed(b"\x1b[5;10H"); // Move to row 5, col 10 (1-based)
    harness.update_state()?;

    // Wait for cursor (expecting 0-based coordinates)
    harness.wait_for_cursor((4, 9))?;

    let pos = harness.cursor_position();
    assert_eq!(pos, (4, 9));

    Ok(())
}

#[test]
fn test_wait_for_cursor_with_text_output() -> Result<()> {
    let mut harness = TuiTestHarness::new(80, 24)?
        .with_timeout(Duration::from_secs(2));

    let mut cmd = CommandBuilder::new("echo");
    cmd.arg("test");
    harness.spawn(cmd)?;

    // After "test" is printed, cursor should move
    harness.wait_for_text("test")?;

    // Cursor should have moved past the text
    let (row, col) = harness.cursor_position();
    assert!(col > 0 || row > 0); // Cursor moved from (0,0)

    Ok(())
}

#[test]
fn test_wait_for_custom_predicate_count() -> Result<()> {
    let mut harness = TuiTestHarness::new(80, 24)?
        .with_timeout(Duration::from_secs(2));

    let mut cmd = CommandBuilder::new("sh");
    cmd.arg("-c");
    cmd.arg("echo '1 2 3 4 5'");
    harness.spawn(cmd)?;

    // Wait for at least 3 digits to appear
    harness.wait_for(|state| {
        state.contents().chars().filter(|c| c.is_numeric()).count() >= 3
    })?;

    let contents = harness.screen_contents();
    let digit_count = contents.chars().filter(|c| c.is_numeric()).count();
    assert!(digit_count >= 3);

    Ok(())
}

#[test]
fn test_wait_for_pattern_matching() -> Result<()> {
    let mut harness = TuiTestHarness::new(80, 24)?
        .with_timeout(Duration::from_secs(2));

    let mut cmd = CommandBuilder::new("echo");
    cmd.arg("Status: 100% complete");
    harness.spawn(cmd)?;

    // Wait for pattern: "Status" followed by a percentage
    harness.wait_for(|state| {
        let contents = state.contents();
        contents.contains("Status") && contents.contains("%")
    })?;

    assert!(harness.screen_contents().contains("100%"));

    Ok(())
}

#[test]
fn test_wait_for_screen_contains_sequence() -> Result<()> {
    let mut harness = TuiTestHarness::new(80, 24)?
        .with_timeout(Duration::from_secs(3));

    let mut cmd = CommandBuilder::new("sh");
    cmd.arg("-c");
    cmd.arg("echo 'Loading...'; sleep 0.1; echo 'Done!'");
    harness.spawn(cmd)?;

    // Wait for loading message
    harness.wait_for_text("Loading")?;

    // Then wait for completion
    harness.wait_for_text("Done!")?;

    let contents = harness.screen_contents();
    assert!(contents.contains("Loading..."));
    assert!(contents.contains("Done!"));

    Ok(())
}

#[test]
fn test_wait_for_empty_screen_initially() -> Result<()> {
    let harness = TuiTestHarness::new(80, 24)?;

    // Screen should be empty initially
    let contents = harness.screen_contents();
    assert!(contents.trim().is_empty() || contents.chars().all(|c| c.is_whitespace()));

    Ok(())
}

#[test]
fn test_wait_for_with_builder_pattern() -> Result<()> {
    let mut harness = TuiTestHarness::builder()
        .with_size(100, 30)
        .with_timeout(Duration::from_secs(3))
        .with_poll_interval(Duration::from_millis(50))
        .build()?;

    let mut cmd = CommandBuilder::new("echo");
    cmd.arg("builder test");
    harness.spawn(cmd)?;

    harness.wait_for_text("builder")?;

    assert!(harness.screen_contents().contains("builder test"));

    Ok(())
}

#[test]
fn test_wait_for_specific_row_content() -> Result<()> {
    let mut harness = TuiTestHarness::new(80, 24)?
        .with_timeout(Duration::from_secs(2));

    let mut cmd = CommandBuilder::new("echo");
    cmd.arg("row test");
    harness.spawn(cmd)?;

    // Wait for text to appear in first row
    harness.wait_for(|state| {
        state.row_contents(0).contains("row test")
    })?;

    assert!(harness.state().row_contents(0).contains("row test"));

    Ok(())
}

#[test]
fn test_wait_for_cursor_timeout() {
    let mut harness = TuiTestHarness::new(80, 24)
        .unwrap()
        .with_timeout(Duration::from_millis(400));

    // Wait for cursor at impossible position
    let result = harness.wait_for_cursor((100, 100));
    assert!(result.is_err());

    match result {
        Err(TermTestError::Timeout { .. }) => {
            // Expected
        }
        _ => panic!("Expected Timeout error"),
    }
}

#[test]
fn test_update_state_reads_all_output() -> Result<()> {
    let mut harness = TuiTestHarness::new(80, 24)?;

    let mut cmd = CommandBuilder::new("sh");
    cmd.arg("-c");
    cmd.arg("for i in 1 2 3; do echo line$i; done");
    harness.spawn(cmd)?;

    // Give it time to complete
    std::thread::sleep(Duration::from_millis(200));

    // Single update should read all available output
    harness.update_state()?;

    let contents = harness.screen_contents();
    assert!(contents.contains("line1"));
    assert!(contents.contains("line2"));
    assert!(contents.contains("line3"));

    Ok(())
}

#[test]
fn test_wait_for_handles_rapid_output() -> Result<()> {
    let mut harness = TuiTestHarness::new(80, 24)?
        .with_timeout(Duration::from_secs(3))
        .with_poll_interval(Duration::from_millis(25));

    let mut cmd = CommandBuilder::new("sh");
    cmd.arg("-c");
    cmd.arg("for i in $(seq 1 10); do echo item$i; done");
    harness.spawn(cmd)?;

    // Wait for first and last items
    harness.wait_for_text("item1")?;
    harness.wait_for_text("item10")?;

    let contents = harness.screen_contents();
    assert!(contents.contains("item1"));
    assert!(contents.contains("item10"));

    Ok(())
}

#[test]
fn test_wait_for_case_sensitive() -> Result<()> {
    let mut harness = TuiTestHarness::new(80, 24)?
        .with_timeout(Duration::from_secs(2));

    let mut cmd = CommandBuilder::new("echo");
    cmd.arg("CaseSensitive");
    harness.spawn(cmd)?;

    // Should find exact case
    harness.wait_for_text("CaseSensitive")?;

    // Should not find different case (would timeout if we tried)
    let contents = harness.screen_contents();
    assert!(contents.contains("CaseSensitive"));
    assert!(!contents.contains("casesensitive"));

    Ok(())
}

#[test]
fn test_wait_for_partial_text() -> Result<()> {
    let mut harness = TuiTestHarness::new(80, 24)?
        .with_timeout(Duration::from_secs(2));

    let mut cmd = CommandBuilder::new("echo");
    cmd.arg("The quick brown fox");
    harness.spawn(cmd)?;

    // Can wait for partial matches
    harness.wait_for_text("quick")?;
    harness.wait_for_text("brown")?;
    harness.wait_for_text("fox")?;

    Ok(())
}

#[test]
fn test_multiple_wait_operations_in_sequence() -> Result<()> {
    let mut harness = TuiTestHarness::new(80, 24)?
        .with_timeout(Duration::from_secs(3));

    let mut cmd = CommandBuilder::new("sh");
    cmd.arg("-c");
    cmd.arg("echo 'Step 1'; echo 'Step 2'; echo 'Step 3'");
    harness.spawn(cmd)?;

    // Sequential waits should all succeed
    harness.wait_for_text("Step 1")?;
    harness.wait_for_text("Step 2")?;
    harness.wait_for_text("Step 3")?;

    let contents = harness.screen_contents();
    assert!(contents.contains("Step 1"));
    assert!(contents.contains("Step 2"));
    assert!(contents.contains("Step 3"));

    Ok(())
}
