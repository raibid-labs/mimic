//! Error types for term-test.

use std::io;
use thiserror::Error;

/// Result type alias for term-test operations.
pub type Result<T> = std::result::Result<T, TermTestError>;

/// Errors that can occur during TUI testing.
#[derive(Debug, Error)]
pub enum TermTestError {
    /// Error from PTY operations.
    #[error("PTY error: {0}")]
    Pty(String),

    /// I/O error.
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),

    /// Timeout waiting for a condition.
    #[error("Timeout waiting for condition after {timeout_ms}ms")]
    Timeout {
        /// Timeout duration in milliseconds.
        timeout_ms: u64,
    },

    /// Error parsing terminal escape sequences.
    #[error("Parse error: {0}")]
    Parse(String),

    /// Snapshot comparison mismatch.
    #[cfg(feature = "snapshot-insta")]
    #[error("Snapshot mismatch: {0}")]
    SnapshotMismatch(String),

    /// Sixel validation failed.
    #[cfg(feature = "sixel")]
    #[error("Sixel validation failed: {0}")]
    SixelValidation(String),

    /// Process spawn failed.
    #[error("Failed to spawn process: {0}")]
    SpawnFailed(String),

    /// Process already running.
    #[error("Process is already running")]
    ProcessAlreadyRunning,

    /// No process running.
    #[error("No process is running")]
    NoProcessRunning,

    /// Invalid terminal dimensions.
    #[error("Invalid terminal dimensions: width={width}, height={height}")]
    InvalidDimensions {
        /// Terminal width.
        width: u16,
        /// Terminal height.
        height: u16,
    },

    /// Bevy-specific errors.
    #[cfg(feature = "bevy")]
    #[error("Bevy error: {0}")]
    Bevy(String),
}

// Conversion from anyhow::Error (used by portable-pty)
impl From<anyhow::Error> for TermTestError {
    fn from(err: anyhow::Error) -> Self {
        TermTestError::Pty(err.to_string())
    }
}
