//! Basic integration tests for term-test.

use term_test::{Result, TuiTestHarness};

#[test]
fn test_create_harness() -> Result<()> {
    let harness = TuiTestHarness::new(80, 24)?;
    assert_eq!(harness.state().size(), (80, 24));
    Ok(())
}

#[test]
fn test_invalid_dimensions() {
    let result = TuiTestHarness::new(0, 24);
    assert!(result.is_err());

    let result = TuiTestHarness::new(80, 0);
    assert!(result.is_err());
}

#[test]
fn test_screen_state() -> Result<()> {
    let harness = TuiTestHarness::new(80, 24)?;
    let state = harness.state();

    // Initially empty
    assert!(state.contents().is_empty() || state.contents().trim().is_empty());

    // Cursor at origin
    let (row, col) = state.cursor_position();
    assert_eq!(row, 0);
    assert_eq!(col, 0);

    Ok(())
}

#[test]
fn test_timeout_configuration() -> Result<()> {
    use std::time::Duration;

    let harness = TuiTestHarness::new(80, 24)?
        .with_timeout(Duration::from_secs(10))
        .with_poll_interval(Duration::from_millis(5));

    // Just verify it doesn't panic
    Ok(())
}
