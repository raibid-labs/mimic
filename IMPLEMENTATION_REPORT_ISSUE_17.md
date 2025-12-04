# Implementation Report: Issue #17 Mouse Event Simulation

## Summary

Successfully implemented mouse event simulation in `TuiTestHarness`, enabling testing of mouse-driven interactions like clicks, drags, and scrolling.

## Implementation Status: COMPLETE ✅

### Delivered Features

1.  **Harness Methods**
    *   `send_mouse_event(event: MouseEvent)`: Low-level method to send any mouse event.
    *   `mouse_click(x, y, button)`: Convenience method for click (Press + Release).
    *   `mouse_drag(start_x, start_y, end_x, end_y, button)`: Convenience method for drag operations.
    *   `mouse_scroll(x, y, direction)`: Convenience method for scrolling.

2.  **Encoding Logic**
    *   Leveraged existing `encode_mouse_event` from `src/events.rs` which implements SGR mouse protocol (`ESC [ < ... M/m`).
    *   Verified encoding logic via existing unit tests in `src/events.rs`.

3.  **Verification**
    *   **Integration Test**: `tests/mouse_integration.rs` verifies that methods execute without error and interact with a spawned process.
    *   **Demo**: `examples/mouse_demo.rs` spawns `cat -v` and verifies that the correct SGR escape sequences are received by the child process.

### API Usage

```rust
use ratatui_testlib::{MouseButton, TuiTestHarness};

let mut harness = TuiTestHarness::new(80, 24)?;

// Click
harness.mouse_click(10, 5, MouseButton::Left)?;

// Drag
harness.mouse_drag(10, 5, 20, 10, MouseButton::Left)?;

// Scroll
harness.mouse_scroll(10, 5, ratatui_testlib::ScrollDirection::Up)?;
```

### Technical Notes

*   **Protocol**: Uses SGR (Select Graphic Rendition) mouse encoding (`1006` mode), which is the modern standard for terminal mouse reporting.
*   **Coordinates**: API uses 0-indexed coordinates (consistent with Ratatui), which are converted to 1-indexed for the SGR sequence.
*   **Verification Trick**: Using `cat -v` in the demo was crucial to verify the sequences, as standard `cat` output of escape sequences might be interpreted by the harness's terminal emulator (vtparse) as control codes rather than text.

## Files Modified/Created

*   `src/harness.rs`: Added mouse methods.
*   `examples/mouse_demo.rs`: New example.
*   `tests/mouse_integration.rs`: New integration test.
*   `tests/integration/mod.rs`: Updated module list (reverted later as separate test file was used).

## Next Steps

*   Proceed to other Wave 1 issues (e.g., Visual Regression).
