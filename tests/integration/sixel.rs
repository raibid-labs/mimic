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

#[test]
fn test_real_sixel_sequence_parsing() -> Result<()> {
    use term_test::ScreenState;

    let mut screen = ScreenState::new(80, 24);

    // Real Sixel sequence with raster attributes
    // Format: ESC P q "Pa;Pb;Ph;Pv [data] ESC \
    screen.feed(b"\x1b[5;10H");              // Position cursor at (5,10) [1-based]
    screen.feed(b"\x1bPq");                   // DCS start with Sixel mode 'q'
    screen.feed(b"\"1;1;100;50");             // Raster: aspect 1:1, 100x50 pixels
    screen.feed(b"#0;2;100;100;100");         // Define color 0 as RGB(100,100,100)
    screen.feed(b"#0~~@@vv");                 // Sixel data
    screen.feed(b"\x1b\\");                   // String terminator

    let regions = screen.sixel_regions();
    assert_eq!(regions.len(), 1, "Should parse exactly one Sixel region");

    let region = &regions[0];
    assert_eq!(region.start_row, 4, "Row should be 4 (0-based)");
    assert_eq!(region.start_col, 9, "Col should be 9 (0-based)");
    assert_eq!(region.width, 100, "Width should match raster attribute");
    assert_eq!(region.height, 50, "Height should match raster attribute");

    Ok(())
}

#[test]
fn test_multiple_sixel_sequences() -> Result<()> {
    use term_test::ScreenState;

    let mut screen = ScreenState::new(120, 40);

    // First Sixel at (10, 10) with 200x150 pixels
    screen.feed(b"\x1b[10;10H\x1bPq\"1;1;200;150#0~\x1b\\");

    // Second Sixel at (20, 60) with 300x200 pixels
    screen.feed(b"\x1b[20;60H\x1bPq\"1;1;300;200#0~\x1b\\");

    let regions = screen.sixel_regions();
    assert_eq!(regions.len(), 2, "Should capture both Sixel sequences");

    // Verify first sequence
    assert_eq!(regions[0].start_row, 9);
    assert_eq!(regions[0].start_col, 9);
    assert_eq!(regions[0].width, 200);
    assert_eq!(regions[0].height, 150);

    // Verify second sequence
    assert_eq!(regions[1].start_row, 19);
    assert_eq!(regions[1].start_col, 59);
    assert_eq!(regions[1].width, 300);
    assert_eq!(regions[1].height, 200);

    Ok(())
}

#[test]
fn test_sixel_abbreviated_raster_format() -> Result<()> {
    use term_test::ScreenState;

    let mut screen = ScreenState::new(80, 24);

    // Abbreviated format: just width;height (no aspect ratio)
    screen.feed(b"\x1b[1;1H\x1bPq\"400;300#0~\x1b\\");

    let regions = screen.sixel_regions();
    assert_eq!(regions.len(), 1);

    let region = &regions[0];
    assert_eq!(region.width, 400, "Should parse abbreviated width");
    assert_eq!(region.height, 300, "Should parse abbreviated height");

    Ok(())
}

#[test]
fn test_sixel_without_raster_attributes() -> Result<()> {
    use term_test::ScreenState;

    let mut screen = ScreenState::new(80, 24);

    // Legacy Sixel without raster attributes
    screen.feed(b"\x1b[5;5H\x1bPq#0;2;100;100;100#0~~@@\x1b\\");

    let regions = screen.sixel_regions();
    assert_eq!(regions.len(), 1, "Should still capture the sequence");

    let region = &regions[0];
    assert_eq!(region.width, 0, "Width should be 0 without raster attributes");
    assert_eq!(region.height, 0, "Height should be 0 without raster attributes");
    assert!(!region.data.is_empty(), "Data should still be captured");

    Ok(())
}

#[test]
fn test_sixel_bounds_validation() -> Result<()> {
    use term_test::ScreenState;

    let mut screen = ScreenState::new(120, 40);

    // Define preview area: rows 5-30, cols 30-100
    let preview_area = (5, 30, 70, 25);

    // Place Sixel within preview area
    screen.feed(b"\x1b[10;40H\x1bPq\"1;1;200;100#0~\x1b\\");

    let capture = SixelCapture::from_screen_state(&screen);

    // Verify bounds
    let sequences = capture.sequences_in_area(preview_area);
    assert_eq!(sequences.len(), 1, "Sixel should be within preview area");

    // Verify no sequences outside preview
    let outside = capture.sequences_outside_area(preview_area);
    assert_eq!(outside.len(), 0, "No Sixel should be outside preview area");

    // Validation should pass
    capture.assert_all_within(preview_area)?;

    Ok(())
}

#[test]
fn test_sixel_out_of_bounds_detection() -> Result<()> {
    use term_test::ScreenState;

    let mut screen = ScreenState::new(120, 40);

    // Define small preview area
    let preview_area = (5, 30, 20, 10);

    // Place Sixel outside preview area
    screen.feed(b"\x1b[1;1H\x1bPq\"1;1;100;50#0~\x1b\\");

    let capture = SixelCapture::from_screen_state(&screen);

    // Verify it's detected as outside
    let outside = capture.sequences_outside_area(preview_area);
    assert_eq!(outside.len(), 1, "Sixel should be detected as outside");

    // Validation should fail
    let result = capture.assert_all_within(preview_area);
    assert!(result.is_err(), "Validation should fail for out-of-bounds Sixel");

    Ok(())
}

#[test]
fn test_sixel_state_comparison() -> Result<()> {
    use term_test::ScreenState;

    // Create first state with Sixel
    let mut screen1 = ScreenState::new(80, 24);
    screen1.feed(b"\x1b[10;10H\x1bPq\"1;1;100;100#0~\x1b\\");
    let capture1 = SixelCapture::from_screen_state(&screen1);

    // Create second state with different Sixel
    let mut screen2 = ScreenState::new(80, 24);
    screen2.feed(b"\x1b[10;10H\x1bPq\"1;1;200;200#0~\x1b\\");
    let capture2 = SixelCapture::from_screen_state(&screen2);

    // States should differ
    assert!(capture1.differs_from(&capture2), "Different Sixel should be detected");

    // Create third state (empty)
    let screen3 = ScreenState::new(80, 24);
    let capture3 = SixelCapture::from_screen_state(&screen3);

    // Should differ from populated states
    assert!(capture1.differs_from(&capture3), "Empty state should differ from populated");

    // Same state should not differ
    assert!(!capture1.differs_from(&capture1), "Same capture should not differ");

    Ok(())
}

#[test]
fn test_complex_sixel_sequence() -> Result<()> {
    use term_test::ScreenState;

    let mut screen = ScreenState::new(150, 50);

    // Complex Sixel with multiple colors and data
    screen.feed(b"\x1b[15;25H");
    screen.feed(b"\x1bPq\"1;1;640;480");
    screen.feed(b"#0;2;0;0;0");           // Black
    screen.feed(b"#1;2;100;0;0");         // Red
    screen.feed(b"#2;2;0;100;0");         // Green
    screen.feed(b"#0~~~");                 // Some black data
    screen.feed(b"#1@@@");                 // Some red data
    screen.feed(b"#2~~~");                 // Some green data
    screen.feed(b"\x1b\\");

    let regions = screen.sixel_regions();
    assert_eq!(regions.len(), 1);

    let region = &regions[0];
    assert_eq!(region.width, 640, "Should parse complex sequence width");
    assert_eq!(region.height, 480, "Should parse complex sequence height");
    assert_eq!(region.start_row, 14);
    assert_eq!(region.start_col, 24);

    Ok(())
}

#[test]
fn test_large_sixel_dimensions() -> Result<()> {
    use term_test::ScreenState;

    let mut screen = ScreenState::new(200, 100);

    // Large Sixel (e.g., 4K image dimensions)
    screen.feed(b"\x1b[1;1H\x1bPq\"1;1;3840;2160#0~\x1b\\");

    let regions = screen.sixel_regions();
    assert_eq!(regions.len(), 1);

    let region = &regions[0];
    assert_eq!(region.width, 3840, "Should handle large width");
    assert_eq!(region.height, 2160, "Should handle large height");

    Ok(())
}

// ========================================================================
// TuiTestHarness Sixel Validation API Tests
// ========================================================================

#[test]
fn test_harness_sixel_count() -> Result<()> {
    use term_test::TuiTestHarness;

    let mut harness = TuiTestHarness::new(80, 24)?;
    assert_eq!(harness.sixel_count(), 0);

    harness.state_mut().feed(b"\x1b[10;10H\x1bPq\"1;1;100;50#0~\x1b\\");
    assert_eq!(harness.sixel_count(), 1);

    harness.state_mut().feed(b"\x1b[15;20H\x1bPq\"1;1;80;60#0~\x1b\\");
    assert_eq!(harness.sixel_count(), 2);

    Ok(())
}

#[test]
fn test_harness_sixel_at() -> Result<()> {
    use term_test::TuiTestHarness;

    let mut harness = TuiTestHarness::new(80, 24)?;

    harness.state_mut().feed(b"\x1b[8;15H\x1bPq\"1;1;120;80#0~\x1b\\");

    let region = harness.sixel_at(7, 14);
    assert!(region.is_some());
    assert_eq!(region.unwrap().width, 120);

    assert!(harness.sixel_at(0, 0).is_none());

    Ok(())
}

#[test]
fn test_harness_assert_within_bounds() -> Result<()> {
    use term_test::TuiTestHarness;

    let mut harness = TuiTestHarness::new(100, 40)?;

    harness.state_mut().feed(b"\x1b[15;30H\x1bPq\"1;1;200;100#0~\x1b\\");

    // Should pass for large area
    let full_screen = (0, 0, 100, 40);
    assert!(harness.assert_sixel_within_bounds(full_screen).is_ok());

    // Should fail for small area
    let small_area = (0, 0, 20, 10);
    assert!(harness.assert_sixel_within_bounds(small_area).is_err());

    Ok(())
}

#[test]
fn test_harness_has_sixel_in_area() -> Result<()> {
    use term_test::TuiTestHarness;

    let mut harness = TuiTestHarness::new(120, 40)?;

    let preview_area = (10, 40, 70, 25);
    assert!(!harness.has_sixel_in_area(preview_area));

    harness.state_mut().feed(b"\x1b[15;60H\x1bPq\"1;1;300;150#0~\x1b\\");
    assert!(harness.has_sixel_in_area(preview_area));

    Ok(())
}

#[test]
fn test_harness_preview_has_sixel() -> Result<()> {
    use term_test::TuiTestHarness;

    let mut harness = TuiTestHarness::new(80, 24)?;

    // Should fail with no Sixel
    assert!(harness.assert_preview_has_sixel().is_err());

    // Place Sixel in preview area (5, 40, 35, 15)
    harness.state_mut().feed(b"\x1b[10;50H\x1bPq\"1;1;150;80#0~\x1b\\");

    // Should now pass
    assert!(harness.assert_preview_has_sixel().is_ok());

    Ok(())
}

#[test]
fn test_harness_preview_has_sixel_custom_area() -> Result<()> {
    use term_test::TuiTestHarness;

    let mut harness = TuiTestHarness::new(120, 40)?;
    let custom_area = (10, 50, 60, 25);

    harness.state_mut().feed(b"\x1b[20;70H\x1bPq\"1;1;400;200#0~\x1b\\");

    assert!(harness.assert_preview_has_sixel_in(custom_area).is_ok());

    let wrong_area = (0, 0, 30, 20);
    assert!(harness.assert_preview_has_sixel_in(wrong_area).is_err());

    Ok(())
}

#[test]
fn test_dgx_pixels_workflow() -> Result<()> {
    use term_test::TuiTestHarness;

    let mut harness = TuiTestHarness::new(80, 24)?;

    // Simulate dgx-pixels showing an image preview
    harness.state_mut().feed(b"\x1b[10;50H\x1bPq\"1;1;200;120#0~\x1b\\");

    // Verify preview detection
    assert_eq!(harness.sixel_count(), 1);
    assert!(harness.assert_preview_has_sixel().is_ok());

    // Verify sidebar is empty
    let sidebar = (0, 0, 40, 24);
    assert!(!harness.has_sixel_in_area(sidebar));

    Ok(())
}

#[test]
fn test_multiple_sixels_boundary_checking() -> Result<()> {
    use term_test::TuiTestHarness;

    let mut harness = TuiTestHarness::new(150, 50)?;

    // Add Sixels at all four corners
    harness.state_mut().feed(b"\x1b[1;1H\x1bPq\"1;1;100;80#0~\x1b\\");        // Top-left
    harness.state_mut().feed(b"\x1b[1;140H\x1bPq\"1;1;80;60#0~\x1b\\");      // Top-right
    harness.state_mut().feed(b"\x1b[45;1H\x1bPq\"1;1;90;70#0~\x1b\\");       // Bottom-left
    harness.state_mut().feed(b"\x1b[45;140H\x1bPq\"1;1;85;65#0~\x1b\\");     // Bottom-right

    assert_eq!(harness.sixel_count(), 4);

    // All should be within full screen
    assert!(harness.assert_sixel_within_bounds((0, 0, 150, 50)).is_ok());

    // Check individual quadrants
    assert!(harness.has_sixel_in_area((0, 0, 75, 25)));        // Top-left quadrant
    assert!(harness.has_sixel_in_area((0, 75, 75, 25)));       // Top-right quadrant
    assert!(harness.has_sixel_in_area((25, 0, 75, 25)));       // Bottom-left quadrant
    assert!(harness.has_sixel_in_area((25, 75, 75, 25)));      // Bottom-right quadrant

    Ok(())
}

#[test]
fn test_sixel_clearing_verification() -> Result<()> {
    use term_test::{ScreenState, TuiTestHarness};

    let mut harness = TuiTestHarness::new(80, 24)?;

    // Add Sixels
    harness.state_mut().feed(b"\x1b[10;20H\x1bPq\"1;1;150;100#0~\x1b\\");
    harness.state_mut().feed(b"\x1b[15;30H\x1bPq\"1;1;100;80#0~\x1b\\");
    assert_eq!(harness.sixel_count(), 2);

    // Simulate screen clear
    *harness.state_mut() = ScreenState::new(80, 24);
    assert_eq!(harness.sixel_count(), 0);

    // Verify empty
    assert!(harness.sixel_regions().is_empty());
    assert!(harness.sixel_at(9, 19).is_none());

    Ok(())
}

#[test]
fn test_overlapping_sixels_detection() -> Result<()> {
    use term_test::TuiTestHarness;

    let mut harness = TuiTestHarness::new(120, 40)?;

    // Two overlapping Sixels
    harness.state_mut().feed(b"\x1b[10;30H\x1bPq\"1;1;200;150#0~\x1b\\");
    harness.state_mut().feed(b"\x1b[15;35H\x1bPq\"1;1;180;140#0~\x1b\\");

    assert_eq!(harness.sixel_count(), 2);

    // Area that overlaps both
    let overlap_area = (8, 28, 220, 160);
    assert!(harness.has_sixel_in_area(overlap_area));

    Ok(())
}

#[test]
fn test_sixel_dimensions_accuracy() -> Result<()> {
    use term_test::TuiTestHarness;

    let mut harness = TuiTestHarness::new(100, 40)?;

    // Test various dimension formats
    harness.state_mut().feed(b"\x1b[5;10H\x1bPq\"1;1;256;128#0~\x1b\\");
    harness.state_mut().feed(b"\x1b[10;20H\x1bPq\"1;1;512;256#0~\x1b\\");
    harness.state_mut().feed(b"\x1b[15;30H\x1bPq\"1;1;1024;768#0~\x1b\\");

    assert_eq!(harness.sixel_count(), 3);

    let region1 = harness.sixel_at(4, 9).unwrap();
    assert_eq!(region1.width, 256);
    assert_eq!(region1.height, 128);

    let region2 = harness.sixel_at(9, 19).unwrap();
    assert_eq!(region2.width, 512);
    assert_eq!(region2.height, 256);

    let region3 = harness.sixel_at(14, 29).unwrap();
    assert_eq!(region3.width, 1024);
    assert_eq!(region3.height, 768);

    Ok(())
}
