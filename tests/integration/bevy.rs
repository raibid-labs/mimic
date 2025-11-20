//! Bevy feature integration tests.

use term_test::{BevyTuiTestHarness, Result};

#[test]
fn test_bevy_harness_creation() -> Result<()> {
    let harness = BevyTuiTestHarness::new()?;
    // Just verify it creates successfully
    Ok(())
}

#[test]
fn test_bevy_update() -> Result<()> {
    let mut harness = BevyTuiTestHarness::new()?;
    harness.update()?;
    Ok(())
}

#[test]
fn test_bevy_update_n() -> Result<()> {
    let mut harness = BevyTuiTestHarness::new()?;
    harness.update_n(10)?;
    Ok(())
}

#[test]
fn test_bevy_render_frame() -> Result<()> {
    let mut harness = BevyTuiTestHarness::new()?;
    harness.render_frame()?;
    Ok(())
}

#[test]
#[cfg(feature = "bevy-ratatui")]
fn test_bevy_ratatui_harness() -> Result<()> {
    let harness = BevyTuiTestHarness::with_bevy_ratatui()?;
    // Verify it creates successfully with bevy_ratatui plugin
    Ok(())
}
