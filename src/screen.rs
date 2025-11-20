//! Terminal screen state management using VT100 parser.

use crate::error::Result;

/// Represents the current state of the terminal screen.
///
/// This wraps a VT100 parser and provides methods to query screen contents,
/// cursor position, and cell attributes.
pub struct ScreenState {
    parser: vt100::Parser,
    width: u16,
    height: u16,
}

impl ScreenState {
    /// Creates a new screen state with the specified dimensions.
    ///
    /// # Arguments
    ///
    /// * `width` - Screen width in columns
    /// * `height` - Screen height in rows
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            parser: vt100::Parser::new(height, width, 0),
            width,
            height,
        }
    }

    /// Feeds data from the PTY to the parser.
    ///
    /// This processes escape sequences and updates the screen state.
    ///
    /// # Arguments
    ///
    /// * `data` - Raw bytes from PTY output
    pub fn feed(&mut self, data: &[u8]) {
        self.parser.process(data);
    }

    /// Returns the screen contents as a string.
    ///
    /// This includes all visible characters, preserving layout.
    pub fn contents(&self) -> String {
        self.parser.screen().contents()
    }

    /// Returns the contents of a specific row.
    ///
    /// # Arguments
    ///
    /// * `row` - Row index (0-based)
    ///
    /// # Returns
    ///
    /// The contents of the row, or an empty string if the row is out of bounds.
    pub fn row_contents(&self, row: u16) -> String {
        if row >= self.height {
            return String::new();
        }

        let screen = self.parser.screen();
        (0..self.width)
            .map(|col| {
                screen
                    .cell(row, col)
                    .map(|c| c.contents().to_string())
                    .unwrap_or_else(|| " ".to_string())
            })
            .collect()
    }

    /// Returns the character at a specific position.
    ///
    /// # Arguments
    ///
    /// * `row` - Row index (0-based)
    /// * `col` - Column index (0-based)
    ///
    /// # Returns
    ///
    /// The character at the position, or None if out of bounds.
    pub fn char_at(&self, row: u16, col: u16) -> Option<char> {
        if row >= self.height || col >= self.width {
            return None;
        }

        self.parser
            .screen()
            .cell(row, col)
            .and_then(|c| c.contents().chars().next())
    }

    /// Returns the current cursor position as (row, col).
    ///
    /// Both row and column are 0-based.
    pub fn cursor_position(&self) -> (u16, u16) {
        let screen = self.parser.screen();
        (screen.cursor_position().0, screen.cursor_position().1)
    }

    /// Returns the screen dimensions as (width, height).
    pub fn size(&self) -> (u16, u16) {
        (self.width, self.height)
    }

    /// Checks if the screen contains the specified text anywhere.
    ///
    /// # Arguments
    ///
    /// * `text` - Text to search for
    pub fn contains(&self, text: &str) -> bool {
        self.contents().contains(text)
    }

    /// Checks if the screen contains the specified text at a specific position.
    ///
    /// # Arguments
    ///
    /// * `row` - Starting row (0-based)
    /// * `col` - Starting column (0-based)
    /// * `text` - Text to match
    pub fn text_at(&self, row: u16, col: u16, text: &str) -> bool {
        if row >= self.height || col >= self.width {
            return false;
        }

        let row_contents = self.row_contents(row);
        if col as usize >= row_contents.len() {
            return false;
        }

        row_contents[col as usize..].starts_with(text)
    }

    /// Returns the screen contents formatted for display with line numbers.
    ///
    /// Useful for debugging and snapshot tests.
    pub fn debug_contents(&self) -> String {
        (0..self.height)
            .map(|row| format!("{:3} | {}", row, self.row_contents(row)))
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// Access to the underlying vt100 screen for advanced queries.
    pub fn screen(&self) -> &vt100::Screen {
        self.parser.screen()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_screen() {
        let screen = ScreenState::new(80, 24);
        assert_eq!(screen.size(), (80, 24));
    }

    #[test]
    fn test_feed_simple_text() {
        let mut screen = ScreenState::new(80, 24);
        screen.feed(b"Hello, World!");
        assert!(screen.contains("Hello, World!"));
    }

    #[test]
    fn test_cursor_position() {
        let screen = ScreenState::new(80, 24);
        let (row, col) = screen.cursor_position();
        assert_eq!(row, 0);
        assert_eq!(col, 0);
    }

    #[test]
    fn test_text_at() {
        let mut screen = ScreenState::new(80, 24);
        screen.feed(b"Hello");
        assert!(screen.text_at(0, 0, "Hello"));
        assert!(!screen.text_at(0, 0, "World"));
    }
}
