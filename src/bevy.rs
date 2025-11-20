//! Bevy ECS integration for testing Bevy-based TUI applications.
//!
//! This module provides a test harness that wraps both the TUI test harness
//! and a Bevy App, enabling comprehensive testing of applications built with
//! bevy_ratatui.

use crate::error::{Result, TermTestError};
use crate::harness::TuiTestHarness;
use crate::screen::ScreenState;

#[cfg(feature = "sixel")]
use crate::sixel::SixelCapture;

/// Test harness for Bevy-based TUI applications.
///
/// This combines TUI testing with Bevy ECS querying and update cycle control,
/// specifically designed for testing applications built with bevy_ratatui.
///
/// # Example
///
/// ```rust,no_run
/// # #[cfg(feature = "bevy-ratatui")]
/// # {
/// use term_test::BevyTuiTestHarness;
///
/// let mut test = BevyTuiTestHarness::new()?;
/// test.update()?;  // Run one Bevy frame
/// # }
/// # Ok::<(), term_test::TermTestError>(())
/// ```
pub struct BevyTuiTestHarness {
    harness: TuiTestHarness,
    // TODO: Phase 4 - Add Bevy App field
    // app: bevy::app::App,
}

impl BevyTuiTestHarness {
    /// Creates a new Bevy TUI test harness.
    ///
    /// # Errors
    ///
    /// Returns an error if initialization fails.
    pub fn new() -> Result<Self> {
        let harness = TuiTestHarness::new(80, 24)?;

        // TODO: Phase 4 - Initialize Bevy App
        // This will include:
        // - Creating a headless Bevy app
        // - Setting up minimal plugins
        // - Disabling rendering

        Ok(Self { harness })
    }

    /// Creates a Bevy TUI test harness with bevy_ratatui plugin.
    ///
    /// This is a convenience method for the common case of testing
    /// applications built with bevy_ratatui.
    ///
    /// # Errors
    ///
    /// Returns an error if initialization fails.
    #[cfg(feature = "bevy-ratatui")]
    pub fn with_bevy_ratatui() -> Result<Self> {
        // TODO: Phase 4 - Initialize with bevy_ratatui plugin
        Self::new()
    }

    /// Runs one Bevy frame update.
    ///
    /// This executes all Bevy systems for one frame.
    ///
    /// # Errors
    ///
    /// Returns an error if the update fails.
    pub fn update(&mut self) -> Result<()> {
        // TODO: Phase 4 - Implement Bevy update
        // This will call app.update()
        Ok(())
    }

    /// Runs N Bevy frame updates.
    ///
    /// # Arguments
    ///
    /// * `count` - Number of frames to update
    ///
    /// # Errors
    ///
    /// Returns an error if any update fails.
    pub fn update_n(&mut self, count: usize) -> Result<()> {
        for _ in 0..count {
            self.update()?;
        }
        Ok(())
    }

    /// Updates Bevy and renders to the terminal.
    ///
    /// This is equivalent to one complete frame: update ECS, then render to PTY.
    ///
    /// # Errors
    ///
    /// Returns an error if update or render fails.
    pub fn render_frame(&mut self) -> Result<()> {
        // TODO: Phase 4 - Implement frame rendering
        // This will:
        // 1. Run app.update()
        // 2. Trigger bevy_ratatui rendering
        // 3. Update harness screen state
        self.update()?;
        self.harness.update_state()?;
        Ok(())
    }

    /// Sends keyboard input (delegates to inner harness).
    ///
    /// # Arguments
    ///
    /// * `text` - Text to send
    ///
    /// # Errors
    ///
    /// Returns an error if sending fails.
    pub fn send_text(&mut self, text: &str) -> Result<()> {
        self.harness.send_text(text)
    }

    /// Returns the current screen state.
    pub fn state(&self) -> &ScreenState {
        self.harness.state()
    }

    /// Waits for a screen condition (delegates to inner harness).
    ///
    /// # Arguments
    ///
    /// * `condition` - Condition to wait for
    ///
    /// # Errors
    ///
    /// Returns an error if timeout is reached.
    pub fn wait_for<F>(&mut self, condition: F) -> Result<()>
    where
        F: Fn(&ScreenState) -> bool,
    {
        self.harness.wait_for(condition)
    }

    /// Checks if Sixel graphics are present in the current screen state.
    #[cfg(feature = "sixel")]
    pub fn has_sixel_graphics(&self) -> bool {
        // TODO: Phase 3 - Implement Sixel detection
        // This will check the screen state for Sixel sequences
        false
    }

    /// Captures the current Sixel state.
    ///
    /// # Errors
    ///
    /// Returns an error if capture fails.
    #[cfg(feature = "sixel")]
    pub fn capture_sixel_state(&self) -> Result<SixelCapture> {
        // TODO: Phase 3 - Implement Sixel capture
        Ok(SixelCapture::new())
    }

    /// Asserts that all Sixel graphics are within the specified area.
    ///
    /// # Arguments
    ///
    /// * `area` - Area as (row, col, width, height)
    ///
    /// # Errors
    ///
    /// Returns an error if any Sixel is outside the area.
    #[cfg(feature = "sixel")]
    pub fn assert_sixel_within(&self, area: (u16, u16, u16, u16)) -> Result<()> {
        let capture = self.capture_sixel_state()?;
        capture.assert_all_within(area)
    }

    /// Asserts that no Sixel graphics are outside the specified area.
    ///
    /// This is the inverse of `assert_sixel_within`.
    ///
    /// # Arguments
    ///
    /// * `area` - Area as (row, col, width, height)
    ///
    /// # Errors
    ///
    /// Returns an error if any Sixel is outside the area.
    #[cfg(feature = "sixel")]
    pub fn assert_no_sixel_outside(&self, area: (u16, u16, u16, u16)) -> Result<()> {
        self.assert_sixel_within(area)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_bevy_harness() {
        let harness = BevyTuiTestHarness::new();
        assert!(harness.is_ok());
    }

    #[test]
    fn test_update() {
        let mut harness = BevyTuiTestHarness::new().unwrap();
        assert!(harness.update().is_ok());
    }

    #[test]
    fn test_update_n() {
        let mut harness = BevyTuiTestHarness::new().unwrap();
        assert!(harness.update_n(5).is_ok());
    }
}
