//! Sixel graphics testing example.
//!
//! This demonstrates how to test Sixel graphics position and bounds.
//! Full implementation will be completed in Phase 3.

use term_test::{Result, SixelCapture};

fn main() -> Result<()> {
    println!("Sixel testing example");

    // Phase 3 will implement full Sixel parsing
    let capture = SixelCapture::new();

    println!("Sixel sequences captured: {}", capture.sequences().len());
    println!("Is empty: {}", capture.is_empty());

    // Example area: preview panel at (5, 5) with size 40x20
    let preview_area = (5, 5, 40, 20);

    // Check if all Sixel graphics are within the preview area
    match capture.assert_all_within(preview_area) {
        Ok(()) => println!("All Sixel graphics are within the preview area!"),
        Err(e) => println!("Sixel validation failed: {}", e),
    }

    println!(
        "\nNote: Full Sixel parsing will be implemented in Phase 3 after vt100 validation"
    );

    Ok(())
}
