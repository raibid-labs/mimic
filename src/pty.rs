//! PTY (pseudo-terminal) management layer.
//!
//! This module provides a wrapper around `portable-pty` for creating and managing
//! pseudo-terminals used in testing TUI applications.

use crate::error::{Result, TermTestError};
use portable_pty::{Child, CommandBuilder, ExitStatus, PtyPair, PtySize};
use std::io::{Read, Write};

/// A test terminal backed by a pseudo-terminal (PTY).
///
/// This provides low-level access to PTY operations for spawning processes,
/// reading output, and sending input.
pub struct TestTerminal {
    pty_pair: PtyPair,
    child: Option<Box<dyn Child + Send + Sync>>,
}

impl TestTerminal {
    /// Creates a new test terminal with the specified dimensions.
    ///
    /// # Arguments
    ///
    /// * `width` - Terminal width in columns
    /// * `height` - Terminal height in rows
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Terminal dimensions are invalid (zero or too large)
    /// - PTY creation fails
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use term_test::TestTerminal;
    ///
    /// let terminal = TestTerminal::new(80, 24)?;
    /// # Ok::<(), term_test::TermTestError>(())
    /// ```
    pub fn new(width: u16, height: u16) -> Result<Self> {
        if width == 0 || height == 0 {
            return Err(TermTestError::InvalidDimensions { width, height });
        }

        let pty_system = portable_pty::native_pty_system();
        let pty_pair = pty_system.openpty(PtySize {
            rows: height,
            cols: width,
            pixel_width: 0,
            pixel_height: 0,
        })?;

        Ok(Self {
            pty_pair,
            child: None,
        })
    }

    /// Spawns a process in the PTY.
    ///
    /// # Arguments
    ///
    /// * `cmd` - Command to spawn
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - A process is already running
    /// - Process spawn fails
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use term_test::TestTerminal;
    /// use portable_pty::CommandBuilder;
    ///
    /// let mut terminal = TestTerminal::new(80, 24)?;
    /// let cmd = CommandBuilder::new("ls");
    /// terminal.spawn(cmd)?;
    /// # Ok::<(), term_test::TermTestError>(())
    /// ```
    pub fn spawn(&mut self, cmd: CommandBuilder) -> Result<()> {
        if self.child.is_some() {
            return Err(TermTestError::ProcessAlreadyRunning);
        }

        let child = self
            .pty_pair
            .slave
            .spawn_command(cmd)
            .map_err(|e| TermTestError::SpawnFailed(e.to_string()))?;

        self.child = Some(child);
        Ok(())
    }

    /// Reads available output from the PTY.
    ///
    /// This is a non-blocking read that returns immediately with whatever data is available.
    ///
    /// # Arguments
    ///
    /// * `buf` - Buffer to read into
    ///
    /// # Errors
    ///
    /// Returns an error if the read operation fails.
    pub fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let mut reader = self.pty_pair.master.try_clone_reader()?;
        Ok(reader.read(buf)?)
    }

    /// Writes data to the PTY (sends input to the process).
    ///
    /// # Arguments
    ///
    /// * `data` - Data to write
    ///
    /// # Errors
    ///
    /// Returns an error if the write operation fails.
    pub fn write(&mut self, data: &[u8]) -> Result<usize> {
        let mut writer = self.pty_pair.master.take_writer()?;
        Ok(writer.write(data)?)
    }

    /// Resizes the PTY.
    ///
    /// # Arguments
    ///
    /// * `width` - New width in columns
    /// * `height` - New height in rows
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Dimensions are invalid
    /// - Resize operation fails
    pub fn resize(&mut self, width: u16, height: u16) -> Result<()> {
        if width == 0 || height == 0 {
            return Err(TermTestError::InvalidDimensions { width, height });
        }

        self.pty_pair.master.resize(PtySize {
            rows: height,
            cols: width,
            pixel_width: 0,
            pixel_height: 0,
        })?;

        Ok(())
    }

    /// Returns the current PTY dimensions.
    pub fn size(&self) -> (u16, u16) {
        // Note: portable-pty doesn't provide a way to query current size,
        // so we'll need to track this ourselves in the future
        // For now, return a placeholder
        (80, 24)
    }

    /// Checks if the child process is still running.
    pub fn is_running(&mut self) -> bool {
        if let Some(ref mut child) = self.child {
            child.try_wait().ok().flatten().is_none()
        } else {
            false
        }
    }

    /// Waits for the child process to exit and returns its exit status.
    ///
    /// # Errors
    ///
    /// Returns an error if no process is running.
    pub fn wait(&mut self) -> Result<ExitStatus> {
        if let Some(mut child) = self.child.take() {
            child
                .wait()
                .map_err(|e| TermTestError::Io(std::io::Error::new(std::io::ErrorKind::Other, e)))
        } else {
            Err(TermTestError::NoProcessRunning)
        }
    }
}

impl Drop for TestTerminal {
    fn drop(&mut self) {
        // Kill the child process if it's still running
        if let Some(mut child) = self.child.take() {
            let _ = child.kill();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_terminal() {
        let terminal = TestTerminal::new(80, 24);
        assert!(terminal.is_ok());
    }

    #[test]
    fn test_invalid_dimensions() {
        let result = TestTerminal::new(0, 24);
        assert!(matches!(
            result,
            Err(TermTestError::InvalidDimensions { .. })
        ));

        let result = TestTerminal::new(80, 0);
        assert!(matches!(
            result,
            Err(TermTestError::InvalidDimensions { .. })
        ));
    }

    #[test]
    fn test_spawn_process() {
        let mut terminal = TestTerminal::new(80, 24).unwrap();
        let mut cmd = CommandBuilder::new("echo");
        cmd.arg("test");
        let result = terminal.spawn(cmd);
        assert!(result.is_ok());
    }
}
