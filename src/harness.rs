//! High-level test harness for TUI applications.

use crate::error::{Result, TermTestError};
use crate::pty::TestTerminal;
use crate::screen::ScreenState;
use portable_pty::{CommandBuilder, ExitStatus};
use std::time::{Duration, Instant};

/// Default timeout for wait operations (5 seconds).
const DEFAULT_TIMEOUT: Duration = Duration::from_secs(5);

/// Default polling interval for wait operations (10ms).
const DEFAULT_POLL_INTERVAL: Duration = Duration::from_millis(10);

/// High-level test harness for TUI applications.
///
/// This combines PTY management and terminal emulation to provide
/// an ergonomic API for testing TUI applications.
///
/// # Example
///
/// ```rust,no_run
/// use term_test::TuiTestHarness;
/// use portable_pty::CommandBuilder;
///
/// let mut harness = TuiTestHarness::new(80, 24)?;
/// let mut cmd = CommandBuilder::new("my-app");
/// harness.spawn(cmd)?;
/// harness.wait_for(|state| state.contains("Ready"))?;
/// # Ok::<(), term_test::TermTestError>(())
/// ```
pub struct TuiTestHarness {
    terminal: TestTerminal,
    state: ScreenState,
    timeout: Duration,
    poll_interval: Duration,
}

impl TuiTestHarness {
    /// Creates a new test harness with the specified dimensions.
    ///
    /// # Arguments
    ///
    /// * `width` - Terminal width in columns
    /// * `height` - Terminal height in rows
    ///
    /// # Errors
    ///
    /// Returns an error if terminal creation fails.
    pub fn new(width: u16, height: u16) -> Result<Self> {
        let terminal = TestTerminal::new(width, height)?;
        let state = ScreenState::new(width, height);

        Ok(Self {
            terminal,
            state,
            timeout: DEFAULT_TIMEOUT,
            poll_interval: DEFAULT_POLL_INTERVAL,
        })
    }

    /// Sets the timeout for wait operations.
    ///
    /// # Arguments
    ///
    /// * `timeout` - Timeout duration
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Sets the polling interval for wait operations.
    ///
    /// # Arguments
    ///
    /// * `interval` - Polling interval
    pub fn with_poll_interval(mut self, interval: Duration) -> Self {
        self.poll_interval = interval;
        self
    }

    /// Spawns a process in the PTY.
    ///
    /// # Arguments
    ///
    /// * `cmd` - Command to spawn
    ///
    /// # Errors
    ///
    /// Returns an error if spawning fails.
    pub fn spawn(&mut self, cmd: CommandBuilder) -> Result<()> {
        self.terminal.spawn(cmd)
    }

    /// Sends text to the PTY.
    ///
    /// # Arguments
    ///
    /// * `text` - Text to send
    ///
    /// # Errors
    ///
    /// Returns an error if the write fails.
    pub fn send_text(&mut self, text: &str) -> Result<()> {
        self.terminal.write(text.as_bytes())?;
        self.update_state()?;
        Ok(())
    }

    /// Updates the screen state by reading from the PTY.
    ///
    /// This is called automatically by other methods but can be called
    /// manually if needed.
    ///
    /// # Errors
    ///
    /// Returns an error if reading from the PTY fails.
    pub fn update_state(&mut self) -> Result<()> {
        let mut buf = [0u8; 4096];
        loop {
            match self.terminal.read(&mut buf) {
                Ok(0) => break, // No more data
                Ok(n) => self.state.feed(&buf[..n]),
                Err(e) if e.to_string().contains("WouldBlock") => break,
                Err(e) => return Err(e),
            }
        }
        Ok(())
    }

    /// Waits for a condition to be true, with timeout.
    ///
    /// # Arguments
    ///
    /// * `condition` - Condition to wait for
    ///
    /// # Errors
    ///
    /// Returns an error if the timeout is reached.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use term_test::TuiTestHarness;
    /// # let mut harness = TuiTestHarness::new(80, 24)?;
    /// harness.wait_for(|state| {
    ///     state.contains("Ready")
    /// })?;
    /// # Ok::<(), term_test::TermTestError>(())
    /// ```
    pub fn wait_for<F>(&mut self, condition: F) -> Result<()>
    where
        F: Fn(&ScreenState) -> bool,
    {
        let start = Instant::now();

        loop {
            self.update_state()?;

            if condition(&self.state) {
                return Ok(());
            }

            if start.elapsed() >= self.timeout {
                return Err(TermTestError::Timeout {
                    timeout_ms: self.timeout.as_millis() as u64,
                });
            }

            std::thread::sleep(self.poll_interval);
        }
    }

    /// Returns the current screen contents as a string.
    pub fn screen_contents(&self) -> String {
        self.state.contents()
    }

    /// Returns the current cursor position as (row, col).
    pub fn cursor_position(&self) -> (u16, u16) {
        self.state.cursor_position()
    }

    /// Returns the current screen state.
    pub fn state(&self) -> &ScreenState {
        &self.state
    }

    /// Returns a mutable reference to the screen state.
    pub fn state_mut(&mut self) -> &mut ScreenState {
        &mut self.state
    }

    /// Resizes the terminal.
    ///
    /// # Arguments
    ///
    /// * `width` - New width in columns
    /// * `height` - New height in rows
    ///
    /// # Errors
    ///
    /// Returns an error if the resize fails.
    pub fn resize(&mut self, width: u16, height: u16) -> Result<()> {
        self.terminal.resize(width, height)?;
        self.state = ScreenState::new(width, height);
        Ok(())
    }

    /// Checks if the child process is still running.
    pub fn is_running(&mut self) -> bool {
        self.terminal.is_running()
    }

    /// Waits for the child process to exit.
    ///
    /// # Errors
    ///
    /// Returns an error if no process is running.
    pub fn wait_exit(&mut self) -> Result<ExitStatus> {
        self.terminal.wait()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_harness() {
        let harness = TuiTestHarness::new(80, 24);
        assert!(harness.is_ok());
    }

    #[test]
    fn test_with_timeout() {
        let harness = TuiTestHarness::new(80, 24)
            .unwrap()
            .with_timeout(Duration::from_secs(10));
        assert_eq!(harness.timeout, Duration::from_secs(10));
    }
}
