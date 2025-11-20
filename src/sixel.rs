//! Sixel graphics testing support.
//!
//! This module provides functionality for detecting, parsing, and validating
//! Sixel escape sequences in terminal output, with a focus on position tracking
//! and bounds checking.

use crate::error::{Result, TermTestError};

/// Represents a captured Sixel sequence with position information.
///
/// This is the core type for Sixel testing in the MVP, tracking where
/// Sixel graphics are rendered on the screen.
#[derive(Debug, Clone, PartialEq)]
pub struct SixelSequence {
    /// Raw Sixel escape sequence bytes
    pub raw: Vec<u8>,
    /// Cursor position when the Sixel was rendered (row, col)
    pub position: (u16, u16),
    /// Calculated bounding rectangle (row, col, width, height)
    pub bounds: (u16, u16, u16, u16),
}

impl SixelSequence {
    /// Creates a new Sixel sequence.
    ///
    /// # Arguments
    ///
    /// * `raw` - Raw escape sequence bytes
    /// * `position` - Cursor position when rendered
    /// * `bounds` - Bounding rectangle (row, col, width, height)
    pub fn new(raw: Vec<u8>, position: (u16, u16), bounds: (u16, u16, u16, u16)) -> Self {
        Self {
            raw,
            position,
            bounds,
        }
    }

    /// Checks if this Sixel is within the specified area.
    ///
    /// # Arguments
    ///
    /// * `area` - Area as (row, col, width, height)
    pub fn is_within(&self, area: (u16, u16, u16, u16)) -> bool {
        let (row, col, width, height) = self.bounds;
        let (area_row, area_col, area_width, area_height) = area;

        row >= area_row
            && col >= area_col
            && (row + height) <= (area_row + area_height)
            && (col + width) <= (area_col + area_width)
    }

    /// Checks if this Sixel overlaps with the specified area.
    ///
    /// # Arguments
    ///
    /// * `area` - Area as (row, col, width, height)
    pub fn overlaps(&self, area: (u16, u16, u16, u16)) -> bool {
        let (row, col, width, height) = self.bounds;
        let (area_row, area_col, area_width, area_height) = area;

        !(row + height <= area_row
            || col + width <= area_col
            || row >= area_row + area_height
            || col >= area_col + area_width)
    }
}

/// Captures all Sixel sequences from terminal output.
///
/// This type provides methods for querying and validating Sixel graphics
/// in the terminal screen state.
#[derive(Debug, Clone, PartialEq)]
pub struct SixelCapture {
    /// All captured Sixel sequences
    sequences: Vec<SixelSequence>,
}

impl SixelCapture {
    /// Creates a new empty Sixel capture.
    pub fn new() -> Self {
        Self {
            sequences: Vec::new(),
        }
    }

    /// Creates a Sixel capture from raw terminal output.
    ///
    /// This parses the output and extracts all Sixel sequences with their positions.
    ///
    /// # Arguments
    ///
    /// * `output` - Raw terminal output bytes
    /// * `cursor_positions` - Cursor positions corresponding to each sequence
    ///
    /// # Note
    ///
    /// Phase 1 implementation is a stub. Full Sixel parsing will be implemented
    /// in Phase 3 after validating vt100 capabilities.
    pub fn from_output(_output: &[u8], _cursor_positions: &[(u16, u16)]) -> Self {
        // TODO: Phase 3 - Implement Sixel sequence detection and parsing
        // This requires:
        // 1. Scanning for Sixel escape sequences (ESC P ... ESC \)
        // 2. Parsing Sixel data to extract dimensions
        // 3. Associating cursor positions with sequences
        // 4. Calculating bounding rectangles
        Self::new()
    }

    /// Returns all captured sequences.
    pub fn sequences(&self) -> &[SixelSequence] {
        &self.sequences
    }

    /// Checks if any Sixel sequences were captured.
    pub fn is_empty(&self) -> bool {
        self.sequences.is_empty()
    }

    /// Returns sequences within the specified area.
    ///
    /// # Arguments
    ///
    /// * `area` - Area as (row, col, width, height)
    pub fn sequences_in_area(&self, area: (u16, u16, u16, u16)) -> Vec<&SixelSequence> {
        self.sequences
            .iter()
            .filter(|seq| seq.is_within(area))
            .collect()
    }

    /// Returns sequences outside the specified area.
    ///
    /// # Arguments
    ///
    /// * `area` - Area as (row, col, width, height)
    pub fn sequences_outside_area(&self, area: (u16, u16, u16, u16)) -> Vec<&SixelSequence> {
        self.sequences
            .iter()
            .filter(|seq| !seq.is_within(area))
            .collect()
    }

    /// Asserts that all Sixel sequences are within the specified area.
    ///
    /// # Arguments
    ///
    /// * `area` - Area as (row, col, width, height)
    ///
    /// # Errors
    ///
    /// Returns an error if any sequence is outside the area.
    pub fn assert_all_within(&self, area: (u16, u16, u16, u16)) -> Result<()> {
        let outside = self.sequences_outside_area(area);
        if !outside.is_empty() {
            return Err(TermTestError::SixelValidation(format!(
                "Found {} Sixel sequence(s) outside area {:?}: {:?}",
                outside.len(),
                area,
                outside.iter().map(|s| s.position).collect::<Vec<_>>()
            )));
        }
        Ok(())
    }

    /// Checks if this capture differs from another.
    ///
    /// Useful for detecting Sixel clearing on screen transitions.
    ///
    /// # Arguments
    ///
    /// * `other` - Other capture to compare with
    pub fn differs_from(&self, other: &SixelCapture) -> bool {
        self.sequences != other.sequences
    }
}

impl Default for SixelCapture {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sixel_sequence_within() {
        let seq = SixelSequence::new(vec![], (5, 5), (5, 5, 10, 10));
        assert!(seq.is_within((0, 0, 20, 20)));
        assert!(!seq.is_within((0, 0, 10, 10)));
    }

    #[test]
    fn test_sixel_sequence_overlaps() {
        let seq = SixelSequence::new(vec![], (5, 5), (5, 5, 10, 10));
        assert!(seq.overlaps((0, 0, 10, 10)));
        assert!(seq.overlaps((10, 10, 10, 10)));
        assert!(!seq.overlaps((0, 0, 5, 5)));
    }

    #[test]
    fn test_sixel_capture_empty() {
        let capture = SixelCapture::new();
        assert!(capture.is_empty());
        assert_eq!(capture.sequences().len(), 0);
    }

    #[test]
    fn test_sixel_capture_filtering() {
        let mut capture = SixelCapture::new();
        capture.sequences.push(SixelSequence::new(vec![], (5, 5), (5, 5, 10, 10)));
        capture.sequences.push(SixelSequence::new(vec![], (20, 20), (20, 20, 10, 10)));

        let area = (0, 0, 15, 15);
        assert_eq!(capture.sequences_in_area(area).len(), 1);
        assert_eq!(capture.sequences_outside_area(area).len(), 1);
    }
}
