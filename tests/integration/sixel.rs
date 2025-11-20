//! Sixel feature integration tests.

use term_test::{Result, SixelCapture, SixelSequence};

#[test]
fn test_sixel_capture_empty() {
    let capture = SixelCapture::new();
    assert!(capture.is_empty());
    assert_eq!(capture.sequences().len(), 0);
}

#[test]
fn test_sixel_sequence_bounds() {
    let seq = SixelSequence::new(vec![0x1b, 0x50], (5, 5), (5, 5, 10, 10));

    // Within bounds
    assert!(seq.is_within((0, 0, 20, 20)));

    // Outside bounds
    assert!(!seq.is_within((0, 0, 10, 10)));
    assert!(!seq.is_within((10, 10, 10, 10)));
}

#[test]
fn test_sixel_sequence_overlaps() {
    let seq = SixelSequence::new(vec![], (5, 5), (5, 5, 10, 10));

    assert!(seq.overlaps((0, 0, 10, 10)));
    assert!(seq.overlaps((10, 10, 10, 10)));
    assert!(seq.overlaps((5, 5, 10, 10)));
    assert!(!seq.overlaps((0, 0, 5, 5)));
    assert!(!seq.overlaps((15, 15, 5, 5)));
}

#[test]
fn test_sixel_capture_filtering() -> Result<()> {
    let mut capture = SixelCapture::new();

    // Manually add sequences for testing
    let seq1 = SixelSequence::new(vec![], (5, 5), (5, 5, 10, 10));
    let seq2 = SixelSequence::new(vec![], (20, 20), (20, 20, 10, 10));

    // Since SixelCapture fields are private, we can't directly modify them
    // This test structure is a placeholder for Phase 3

    let area = (0, 0, 15, 15);
    assert_eq!(capture.sequences_in_area(area).len(), 0);

    Ok(())
}

#[test]
fn test_sixel_validation() -> Result<()> {
    let capture = SixelCapture::new();
    let area = (0, 0, 80, 24);

    // Empty capture should pass validation
    capture.assert_all_within(area)?;

    Ok(())
}
