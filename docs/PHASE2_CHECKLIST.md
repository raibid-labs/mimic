# Phase 2: Event Simulation & Async Support - Implementation Checklist

**Status**: Ready for Implementation
**Priority**: P0 (Critical - MVP Blocker)
**Dependencies**: Phase 1 Complete (100%)
**Target Duration**: 1-2 weeks
**Assignee**: Implementation Agents

## Overview

Phase 2 delivers rich event simulation and async runtime integration to enable interactive TUI testing. This phase transforms the harness from passive observation to active interaction.

### Goals

1. Implement keyboard event simulation (single keys, sequences, modifiers)
2. Create smart wait conditions with timeout and polling
3. Integrate Tokio async runtime support
4. Enable comprehensive input testing for dgx-pixels navigation

### Success Criteria

- Can simulate keyboard input (keys and text)
- Can wait for screen state conditions with timeout
- Async harness works with Tokio runtime
- Can test dgx-pixels navigation (Tab, number keys, Esc)
- Examples demonstrate all event simulation patterns

---

## Task Breakdown

### 1. Event Simulation Foundation

#### 1.1 Key Event Types

**Priority**: P0
**Estimated Effort**: 2-4 hours

**Tasks**:
- [ ] Create `KeyCode` enum for all keyboard keys
  - Alphanumeric: `Char(char)` for a-z, A-Z, 0-9
  - Special: `Enter`, `Esc`, `Tab`, `Backspace`, `Delete`
  - Navigation: `Up`, `Down`, `Left`, `Right`
  - Function: `F1-F12`
  - Other: `Home`, `End`, `PageUp`, `PageDown`, `Insert`
- [ ] Create `Modifiers` bitflags struct
  - `SHIFT`, `CTRL`, `ALT`, `META`
- [ ] Create `KeyEvent` struct combining `KeyCode` and `Modifiers`
- [ ] Implement `Debug` and `Display` for all types
- [ ] Add unit tests for type construction

**Files to Create**:
- `src/events.rs` - Event types module

**API Design**:
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyCode {
    Char(char),
    Enter,
    Esc,
    Tab,
    Backspace,
    Delete,
    Up,
    Down,
    Left,
    Right,
    Home,
    End,
    PageUp,
    PageDown,
    Insert,
    F(u8), // F1-F12
}

bitflags::bitflags! {
    pub struct Modifiers: u8 {
        const SHIFT = 0b0001;
        const CTRL  = 0b0010;
        const ALT   = 0b0100;
        const META  = 0b1000;
    }
}

#[derive(Debug, Clone, Copy)]
pub struct KeyEvent {
    pub code: KeyCode,
    pub modifiers: Modifiers,
}
```

**Success Criteria**:
- All key types can be constructed
- Modifiers can be combined with bitflags
- Types have clear Debug output

---

#### 1.2 Escape Sequence Generation

**Priority**: P0
**Estimated Effort**: 4-6 hours

**Tasks**:
- [ ] Implement `KeyCode::to_escape_sequence()` method
  - Map each key to its VT100/ANSI escape sequence
  - Handle standard keys (letters, numbers, symbols)
  - Handle special keys (Enter = `\n`, Tab = `\t`, Esc = `\x1b`)
  - Handle navigation keys (Up = `\x1b[A`, Down = `\x1b[B`, etc.)
  - Handle function keys (F1 = `\x1bOP`, etc.)
- [ ] Implement `KeyEvent::to_escape_sequence()` with modifiers
  - Apply modifier prefixes (Ctrl+C = `\x03`, etc.)
  - Handle Alt combinations (Alt+A = `\x1ba`, etc.)
  - Handle complex modifier sequences
- [ ] Create comprehensive test suite
  - Test all key codes
  - Test all modifier combinations
  - Test edge cases (Ctrl+[, etc.)
- [ ] Document escape sequence mapping

**Reference Implementation**:
```rust
impl KeyCode {
    pub fn to_escape_sequence(&self) -> &'static [u8] {
        match self {
            KeyCode::Char(c) => /* return char as bytes */,
            KeyCode::Enter => b"\n",
            KeyCode::Tab => b"\t",
            KeyCode::Esc => b"\x1b",
            KeyCode::Up => b"\x1b[A",
            KeyCode::Down => b"\x1b[B",
            KeyCode::Right => b"\x1b[C",
            KeyCode::Left => b"\x1b[D",
            KeyCode::Home => b"\x1b[H",
            KeyCode::End => b"\x1b[F",
            KeyCode::PageUp => b"\x1b[5~",
            KeyCode::PageDown => b"\x1b[6~",
            KeyCode::F(n) => /* map F1-F12 */,
            // ... more mappings
        }
    }
}
```

**Success Criteria**:
- All keys generate correct escape sequences
- Modifiers are applied correctly
- Tests verify against reference sequences

---

#### 1.3 Harness Event Methods

**Priority**: P0
**Estimated Effort**: 3-4 hours

**Tasks**:
- [ ] Add `send_key(KeyCode)` method to `TuiTestHarness`
  - Convert KeyCode to escape sequence
  - Write to PTY master
  - Update screen state
  - Return Result with proper error handling
- [ ] Add `send_key_with_modifiers(KeyCode, Modifiers)` method
  - Construct KeyEvent
  - Generate modified escape sequence
  - Write to PTY
- [ ] Add `send_keys(&str)` convenience method
  - Convert string to sequence of Char(c) events
  - Send each character individually
  - Alternative name: `type_text(&str)`
- [ ] Add integration tests for each method
  - Test single key sending
  - Test modifier key combinations
  - Test text string typing
  - Verify screen state updates

**API Addition to `harness.rs`**:
```rust
impl TuiTestHarness {
    /// Sends a single key event to the PTY.
    pub fn send_key(&mut self, key: KeyCode) -> Result<()> {
        let sequence = key.to_escape_sequence();
        self.terminal.write(sequence)?;
        self.update_state()?;
        Ok(())
    }

    /// Sends a key with modifiers.
    pub fn send_key_with_modifiers(
        &mut self,
        key: KeyCode,
        modifiers: Modifiers,
    ) -> Result<()> {
        let event = KeyEvent { code: key, modifiers };
        let sequence = event.to_escape_sequence();
        self.terminal.write(sequence)?;
        self.update_state()?;
        Ok(())
    }

    /// Types a text string (sends each character as a key event).
    pub fn send_keys(&mut self, text: &str) -> Result<()> {
        for ch in text.chars() {
            self.send_key(KeyCode::Char(ch))?;
        }
        Ok(())
    }
}
```

**Success Criteria**:
- Methods compile and work correctly
- Integration tests pass
- Error handling is robust

---

### 2. Smart Wait Conditions

#### 2.1 Enhanced Wait API

**Priority**: P0
**Estimated Effort**: 3-4 hours

**Tasks**:
- [ ] Review existing `wait_for()` implementation
- [ ] Add `wait_for_text(text: &str)` convenience method (already exists, verify)
- [ ] Add `wait_for_cursor(row, col)` method
  - Poll until cursor reaches position
  - Timeout if not reached
- [ ] Add `wait_for_state<F>(predicate: F, timeout: Duration)` variant
  - Allow custom timeout per call
- [ ] Improve timeout error messages
  - Include what was being waited for
  - Show current screen state
  - Show elapsed time
- [ ] Add debug logging option
  - Log each polling iteration
  - Log screen state changes
  - Controlled by environment variable or builder option

**API Enhancements**:
```rust
impl TuiTestHarness {
    /// Waits for cursor to reach a specific position.
    pub fn wait_for_cursor(&mut self, row: u16, col: u16) -> Result<()> {
        self.wait_for_with_context(
            |state| state.cursor_position() == (row, col),
            &format!("cursor at ({}, {})", row, col),
        )
    }

    /// Waits for a condition with custom timeout.
    pub fn wait_for_timeout<F>(
        &mut self,
        condition: F,
        timeout: Duration,
    ) -> Result<()>
    where
        F: Fn(&ScreenState) -> bool,
    {
        // Similar to wait_for but uses custom timeout
    }
}
```

**Success Criteria**:
- All wait methods work correctly
- Timeout errors are informative
- Debug logging helps troubleshooting

---

#### 2.2 Common Wait Patterns

**Priority**: P1
**Estimated Effort**: 2-3 hours

**Tasks**:
- [ ] Create module `src/wait_patterns.rs` for common conditions
- [ ] Implement `wait_for_text_at(row, col, text)` helper
- [ ] Implement `wait_for_text_contains(text)` helper (alias to existing)
- [ ] Implement `wait_for_no_text(text)` helper
  - Wait until text disappears
  - Useful for loading spinners
- [ ] Implement `wait_for_change()` helper
  - Wait until screen contents change
  - Compare snapshots
- [ ] Add examples demonstrating each pattern
- [ ] Document common wait patterns in cookbook

**Example API**:
```rust
/// Common wait pattern helpers
pub mod wait {
    pub fn text_at(row: u16, col: u16, text: &str) -> impl Fn(&ScreenState) -> bool {
        move |state| {
            // Check if text appears at position
        }
    }

    pub fn text_disappears(text: &str) -> impl Fn(&ScreenState) -> bool {
        move |state| !state.contains(text)
    }

    pub fn screen_changed(prev: &str) -> impl Fn(&ScreenState) -> bool {
        move |state| state.contents() != prev
    }
}
```

**Success Criteria**:
- Helpers simplify common test patterns
- Examples show practical usage
- Documentation is clear

---

### 3. Tokio Async Integration

#### 3.1 Async Feature Setup

**Priority**: P0
**Estimated Effort**: 1-2 hours

**Tasks**:
- [ ] Verify Cargo.toml has correct tokio dependency (already exists)
- [ ] Verify `async-tokio` feature flag (already exists)
- [ ] Add tokio to dev-dependencies with test feature
- [ ] Configure async runtime in examples
- [ ] Add async examples to CI pipeline

**Cargo.toml verification**:
```toml
[dependencies]
tokio = { version = "1.35", optional = true, features = ["full"] }

[dev-dependencies]
tokio = { version = "1.35", features = ["rt-multi-thread", "macros", "test"] }

[features]
async-tokio = ["tokio"]
```

**Success Criteria**:
- Dependencies are correct
- Feature flags work
- Examples compile with `--features async-tokio`

---

#### 3.2 Async Harness Implementation

**Priority**: P0 (Critical for dgx-pixels)
**Estimated Effort**: 6-8 hours

**Decision**: Implement native async harness vs sync harness in async context

**Option A: Native Async Harness (Recommended)**
- Create `AsyncTuiTestHarness` with async methods
- Use tokio::time for delays
- Use tokio::fs for reading PTY asynchronously
- Better ergonomics for async tests

**Option B: Sync Harness in Async (Current)**
- Use existing `TuiTestHarness` in async functions
- Wrap blocking calls with `tokio::task::spawn_blocking`
- Simpler implementation but less ergonomic

**Recommended: Option A**

**Tasks**:
- [ ] Create `src/async_harness.rs` module
- [ ] Implement `AsyncTuiTestHarness` struct
  - Wrap `TestTerminal` with async I/O
  - Make all methods async
- [ ] Implement async spawn
  ```rust
  pub async fn spawn(&mut self, cmd: CommandBuilder) -> Result<()>
  ```
- [ ] Implement async send methods
  ```rust
  pub async fn send_key(&mut self, key: KeyCode) -> Result<()>
  pub async fn send_keys(&mut self, text: &str) -> Result<()>
  ```
- [ ] Implement async wait methods
  ```rust
  pub async fn wait_for<F>(&mut self, condition: F) -> Result<()>
  where F: Fn(&ScreenState) -> bool
  ```
- [ ] Use `tokio::time::sleep` instead of `std::thread::sleep`
- [ ] Use `tokio::time::timeout` for timeout handling
- [ ] Add async update_state method
- [ ] Write async integration tests using `#[tokio::test]`
- [ ] Create async example demonstrating usage
- [ ] Document async patterns in examples

**API Design**:
```rust
#[cfg(feature = "async-tokio")]
pub struct AsyncTuiTestHarness {
    inner: TuiTestHarness,
}

#[cfg(feature = "async-tokio")]
impl AsyncTuiTestHarness {
    pub async fn new(width: u16, height: u16) -> Result<Self> {
        Ok(Self {
            inner: TuiTestHarness::new(width, height)?,
        })
    }

    pub async fn spawn(&mut self, cmd: CommandBuilder) -> Result<()> {
        // Spawn in blocking task
        let terminal = &mut self.inner.terminal;
        tokio::task::spawn_blocking(move || {
            terminal.spawn(cmd)
        }).await?
    }

    pub async fn send_key(&mut self, key: KeyCode) -> Result<()> {
        self.inner.send_key(key)?;
        Ok(())
    }

    pub async fn wait_for<F>(&mut self, condition: F) -> Result<()>
    where
        F: Fn(&ScreenState) -> bool,
    {
        let timeout = self.inner.timeout;
        tokio::time::timeout(timeout, async {
            loop {
                self.inner.update_state()?;
                if condition(&self.inner.state) {
                    return Ok(());
                }
                tokio::time::sleep(self.inner.poll_interval).await;
            }
        })
        .await
        .map_err(|_| TermTestError::Timeout {
            timeout_ms: timeout.as_millis() as u64,
        })?
    }
}
```

**Success Criteria**:
- Async harness compiles with tokio feature
- All async methods work correctly
- Async tests pass with `#[tokio::test]`
- Examples demonstrate async usage

---

#### 3.3 Async Testing Patterns

**Priority**: P1
**Estimated Effort**: 2-3 hours

**Tasks**:
- [ ] Create comprehensive async testing example
  - Basic async test structure
  - Concurrent test execution
  - Timeout handling patterns
  - Integration with tokio::select!
- [ ] Document async testing best practices
  - When to use sync vs async harness
  - Handling timeouts
  - Concurrent testing strategies
- [ ] Add async tests to test suite
  - Test async spawn
  - Test async wait conditions
  - Test concurrent harness usage
- [ ] Update async_test.rs example with new API

**Example Test Pattern**:
```rust
#[cfg(test)]
mod async_tests {
    use super::*;
    use tokio::test;

    #[tokio::test]
    async fn test_async_wait() -> Result<()> {
        let mut harness = AsyncTuiTestHarness::new(80, 24).await?;
        let mut cmd = CommandBuilder::new("echo");
        cmd.arg("Hello!");

        harness.spawn(cmd).await?;
        harness.wait_for(|state| {
            state.contains("Hello!")
        }).await?;

        Ok(())
    }

    #[tokio::test]
    async fn test_concurrent_harnesses() -> Result<()> {
        let tasks = (0..3).map(|i| {
            tokio::spawn(async move {
                let mut harness = AsyncTuiTestHarness::new(80, 24).await?;
                // ... test logic
                Ok::<(), TermTestError>(())
            })
        });

        for task in tasks {
            task.await??;
        }

        Ok(())
    }
}
```

**Success Criteria**:
- Async patterns are well documented
- Examples cover common scenarios
- Tests demonstrate async capabilities

---

### 4. Integration & Testing

#### 4.1 Event Simulation Tests

**Priority**: P0
**Estimated Effort**: 3-4 hours

**Tasks**:
- [ ] Create `tests/integration/events.rs`
- [ ] Test single key sending
  - Test alphanumeric keys
  - Test special keys (Enter, Tab, Esc)
  - Test navigation keys (arrows)
- [ ] Test key combinations with modifiers
  - Ctrl+C, Ctrl+D
  - Alt+key combinations
  - Multiple modifiers
- [ ] Test text typing
  - Simple strings
  - Strings with special characters
  - Multi-line text
- [ ] Test error handling
  - Invalid key codes
  - PTY write failures
- [ ] Verify screen state updates after input

**Test Structure**:
```rust
#[test]
fn test_send_single_key() -> Result<()> {
    let mut harness = TuiTestHarness::new(80, 24)?;
    let mut cmd = CommandBuilder::new("cat");
    harness.spawn(cmd)?;

    harness.send_key(KeyCode::Char('a'))?;
    harness.wait_for_text("a")?;

    Ok(())
}

#[test]
fn test_send_key_with_ctrl() -> Result<()> {
    // Test Ctrl+D sends EOF
    let mut harness = TuiTestHarness::new(80, 24)?;
    let mut cmd = CommandBuilder::new("cat");
    harness.spawn(cmd)?;

    harness.send_key_with_modifiers(
        KeyCode::Char('d'),
        Modifiers::CTRL
    )?;

    // cat should exit on Ctrl+D
    harness.wait_exit()?;
    Ok(())
}
```

**Success Criteria**:
- All event tests pass
- Coverage includes edge cases
- Tests are reliable and fast

---

#### 4.2 Wait Condition Tests

**Priority**: P0
**Estimated Effort**: 2-3 hours

**Tasks**:
- [ ] Create `tests/integration/wait.rs`
- [ ] Test wait_for with simple condition
- [ ] Test wait_for_text
- [ ] Test wait_for_cursor
- [ ] Test timeout behavior
  - Ensure timeout fires
  - Verify error message quality
- [ ] Test polling interval
  - Verify condition checked multiple times
  - Verify sleep between checks
- [ ] Test immediate success (condition already true)

**Test Examples**:
```rust
#[test]
fn test_wait_for_text() -> Result<()> {
    let mut harness = TuiTestHarness::new(80, 24)?;
    let mut cmd = CommandBuilder::new("sh");
    cmd.arg("-c").arg("sleep 0.1 && echo 'Ready'");
    harness.spawn(cmd)?;

    harness.wait_for_text("Ready")?;
    Ok(())
}

#[test]
fn test_wait_timeout() {
    let mut harness = TuiTestHarness::new(80, 24).unwrap();
    let mut cmd = CommandBuilder::new("cat");
    harness.spawn(cmd).unwrap();

    let result = harness.wait_for(|state| {
        state.contains("NeverAppears")
    });

    assert!(matches!(result, Err(TermTestError::Timeout { .. })));
}
```

**Success Criteria**:
- Wait conditions work reliably
- Timeouts are accurate
- Tests complete quickly

---

#### 4.3 Async Integration Tests

**Priority**: P0
**Estimated Effort**: 3-4 hours

**Tasks**:
- [ ] Create `tests/async_integration.rs` (requires tokio feature)
- [ ] Test async harness creation
- [ ] Test async spawn
- [ ] Test async send_key
- [ ] Test async wait_for
- [ ] Test async timeout handling
- [ ] Test concurrent harness usage
- [ ] Add tests to CI pipeline with async-tokio feature

**Test Examples**:
```rust
#[cfg(feature = "async-tokio")]
mod async_tests {
    use term_test::AsyncTuiTestHarness;

    #[tokio::test]
    async fn test_async_basic() -> term_test::Result<()> {
        let mut harness = AsyncTuiTestHarness::new(80, 24).await?;
        // ... async test logic
        Ok(())
    }

    #[tokio::test]
    async fn test_async_concurrent() -> term_test::Result<()> {
        tokio::join!(
            async { /* harness 1 */ },
            async { /* harness 2 */ },
        );
        Ok(())
    }
}
```

**Success Criteria**:
- Async tests pass with tokio
- Concurrent execution works
- Tests are fast and reliable

---

### 5. Documentation & Examples

#### 5.1 API Documentation

**Priority**: P0
**Estimated Effort**: 3-4 hours

**Tasks**:
- [ ] Document all new public APIs with rustdoc
  - KeyCode enum and variants
  - Modifiers bitflags
  - KeyEvent struct
  - send_key methods
  - wait_for methods
  - AsyncTuiTestHarness (if implemented)
- [ ] Add code examples to each method
- [ ] Document common patterns
- [ ] Add links between related methods
- [ ] Run `cargo doc` and verify output

**Documentation Standards**:
- Each public item must have summary
- Complex items need examples
- Examples must compile and run
- Links to related functionality

**Success Criteria**:
- `cargo doc` builds without warnings
- Documentation is clear and helpful
- Examples demonstrate usage

---

#### 5.2 Usage Examples

**Priority**: P0
**Estimated Effort**: 4-5 hours

**Tasks**:
- [ ] Update `examples/basic_test.rs` with event simulation
- [ ] Create `examples/keyboard_events.rs`
  - Demonstrate all key types
  - Show modifier usage
  - Show text typing
- [ ] Create `examples/wait_patterns.rs`
  - Show all wait conditions
  - Demonstrate timeout handling
  - Show debugging failed waits
- [ ] Update `examples/async_test.rs` with AsyncTuiTestHarness
- [ ] Create `examples/dgx_pixels_navigation.rs`
  - Simulate Tab navigation
  - Simulate number key screen switching
  - Simulate Esc to exit
- [ ] Verify all examples run successfully
- [ ] Add examples to README

**Example Structure**:
```rust
//! examples/keyboard_events.rs
//!
//! Demonstrates keyboard event simulation patterns.

use term_test::{TuiTestHarness, KeyCode, Modifiers};

fn main() -> term_test::Result<()> {
    // Example 1: Single keys
    example_single_keys()?;

    // Example 2: Modifiers
    example_modifiers()?;

    // Example 3: Text typing
    example_text_typing()?;

    Ok(())
}
```

**Success Criteria**:
- Examples cover all major features
- Examples run without errors
- Examples are instructive

---

#### 5.3 User Guide

**Priority**: P1
**Estimated Effort**: 2-3 hours

**Tasks**:
- [ ] Create `docs/EVENT_SIMULATION.md`
  - Keyboard event basics
  - Available key codes
  - Using modifiers
  - Common patterns
- [ ] Create `docs/WAIT_CONDITIONS.md`
  - How wait_for works
  - Built-in wait helpers
  - Custom conditions
  - Timeout strategies
  - Debugging tips
- [ ] Create `docs/ASYNC_TESTING.md`
  - When to use async
  - Async harness vs sync harness
  - Tokio integration
  - Concurrent testing
- [ ] Update main README with Phase 2 features
- [ ] Add cookbook entries for common scenarios

**Documentation Coverage**:
- Getting started tutorials
- API reference
- Common patterns
- Troubleshooting

**Success Criteria**:
- Documentation is comprehensive
- Examples are clear
- Troubleshooting helps resolve issues

---

### 6. dgx-pixels Integration Validation

#### 6.1 Navigation Testing

**Priority**: P0 (MVP Requirement)
**Estimated Effort**: 2-3 hours

**Tasks**:
- [ ] Create dgx-pixels-specific test example
- [ ] Test Tab key navigation between UI elements
- [ ] Test number keys (1-8) for screen switching
- [ ] Test Esc key to exit/cancel
- [ ] Test text input in generation prompts
- [ ] Document dgx-pixels testing patterns

**Example Test**:
```rust
// Example dgx-pixels navigation test
#[test]
fn test_dgx_pixels_screen_navigation() -> Result<()> {
    let mut harness = TuiTestHarness::new(80, 24)?;
    // Spawn dgx-pixels (or mock)
    harness.spawn(/* ... */)?;

    // Wait for main menu
    harness.wait_for_text("Gallery")?;

    // Navigate to Gallery screen with '2'
    harness.send_key(KeyCode::Char('2'))?;
    harness.wait_for_text("Gallery Screen")?;

    // Use Tab to navigate
    harness.send_key(KeyCode::Tab)?;
    // Verify focus change

    // Press Esc to return
    harness.send_key(KeyCode::Esc)?;
    harness.wait_for_text("Main Menu")?;

    Ok(())
}
```

**Success Criteria**:
- All dgx-pixels navigation patterns work
- Tests are reliable
- Documentation helps dgx-pixels integration

---

### 7. Polish & Release

#### 7.1 Error Handling

**Priority**: P0
**Estimated Effort**: 2-3 hours

**Tasks**:
- [ ] Review all error paths
- [ ] Ensure error messages are actionable
- [ ] Add context to errors (what operation failed)
- [ ] Test error scenarios
  - Invalid key codes
  - Timeout errors
  - PTY write failures
- [ ] Document error handling best practices

**Error Message Quality**:
- What went wrong
- Why it went wrong (if knowable)
- How to fix it
- Current state information

**Success Criteria**:
- Error messages help debugging
- Error types are appropriate
- Error handling is consistent

---

#### 7.2 Performance

**Priority**: P1
**Estimated Effort**: 2-3 hours

**Tasks**:
- [ ] Profile wait condition polling overhead
- [ ] Optimize hot paths
  - Escape sequence generation
  - Screen state updates
  - Polling loops
- [ ] Benchmark event sending speed
- [ ] Benchmark wait condition performance
- [ ] Document performance characteristics

**Performance Targets**:
- Event sending: < 1ms per event
- Wait condition check: < 10ms per iteration
- Overall test overhead: < 10% of test time

**Success Criteria**:
- Performance is acceptable
- No obvious bottlenecks
- Benchmarks document performance

---

#### 7.3 CI/CD Integration

**Priority**: P0
**Estimated Effort**: 1-2 hours

**Tasks**:
- [ ] Add Phase 2 tests to CI pipeline
- [ ] Test with async-tokio feature enabled
- [ ] Test without async-tokio feature
- [ ] Verify examples run in CI
- [ ] Check code coverage for Phase 2 code
- [ ] Ensure tests are stable and don't flake

**CI Configuration**:
```yaml
# .github/workflows/ci.yml
- name: Test Phase 2 (sync)
  run: cargo test --lib

- name: Test Phase 2 (async)
  run: cargo test --lib --features async-tokio

- name: Run examples
  run: |
    cargo run --example keyboard_events
    cargo run --example wait_patterns
    cargo run --example async_test --features async-tokio
```

**Success Criteria**:
- All CI tests pass
- Tests run on Linux (primary target)
- No flaky tests
- Coverage maintained > 70%

---

## Timeline Estimate

### Week 1 (Days 1-5)

**Focus**: Core Event Simulation

- Day 1-2: Event types and escape sequence generation (1.1, 1.2)
- Day 3: Harness event methods (1.3)
- Day 4-5: Wait conditions and patterns (2.1, 2.2)

### Week 2 (Days 6-10)

**Focus**: Async Support & Integration

- Day 6-7: Async harness implementation (3.1, 3.2)
- Day 8: Async patterns and testing (3.3)
- Day 9: Integration tests (4.1, 4.2, 4.3)
- Day 10: Documentation and polish (5.1, 5.2, 5.3)

### Buffer (Days 11-14)

**Focus**: Validation & Polish

- Day 11: dgx-pixels integration validation (6.1)
- Day 12: Error handling and performance (7.1, 7.2)
- Day 13: CI/CD integration (7.3)
- Day 14: Final testing and bug fixes

**Total**: 10-14 days (2 weeks nominal, 2.8 weeks with buffer)

---

## Risk Assessment

### High Risk

**Risk**: Escape sequence mapping incompatibilities
- **Mitigation**: Test against multiple terminals, reference VT100 spec
- **Fallback**: Provide escape sequence customization API

**Risk**: Async harness complexity
- **Mitigation**: Start with Option B (sync in async), implement Option A if needed
- **Fallback**: Document sync harness async patterns thoroughly

### Medium Risk

**Risk**: Flaky wait condition tests
- **Mitigation**: Use generous timeouts in tests, mock PTY behavior where possible
- **Fallback**: Add retry logic to tests

**Risk**: Performance overhead from polling
- **Mitigation**: Optimize polling interval, consider event-driven approach
- **Fallback**: Document performance characteristics, let users tune

### Low Risk

**Risk**: Missing key codes
- **Mitigation**: Start with common keys, add more as needed
- **Fallback**: Provide escape sequence pass-through API

---

## Dependencies

### Internal Dependencies (Phase 1)

- [x] TuiTestHarness implemented
- [x] TestTerminal with write capability
- [x] ScreenState with update mechanism
- [x] wait_for infrastructure exists
- [x] Error types defined

### External Dependencies

- [x] tokio = "1.35" (optional, already in Cargo.toml)
- [x] portable-pty = "0.8" (already in use)
- [ ] bitflags = "2.4" (for Modifiers, need to add)

**Action**: Add bitflags to Cargo.toml

---

## Acceptance Criteria

Phase 2 is complete when:

1. **Event Simulation**
   - [ ] Can send single keys with `send_key()`
   - [ ] Can send keys with modifiers
   - [ ] Can type text strings with `send_keys()`
   - [ ] All key codes generate correct escape sequences
   - [ ] Integration tests verify event handling

2. **Wait Conditions**
   - [ ] `wait_for()` works reliably
   - [ ] `wait_for_text()` convenience method works
   - [ ] `wait_for_cursor()` works
   - [ ] Timeouts work correctly
   - [ ] Error messages are helpful

3. **Async Support**
   - [ ] AsyncTuiTestHarness compiles with async-tokio feature
   - [ ] Async methods work correctly
   - [ ] Can use with `#[tokio::test]`
   - [ ] Concurrent harness usage works
   - [ ] Async examples demonstrate patterns

4. **Testing**
   - [ ] All unit tests pass
   - [ ] All integration tests pass
   - [ ] All async tests pass
   - [ ] Code coverage > 70%
   - [ ] CI/CD passes

5. **Documentation**
   - [ ] All public APIs documented
   - [ ] Examples demonstrate all features
   - [ ] User guides complete
   - [ ] dgx-pixels patterns documented

6. **dgx-pixels Validation**
   - [ ] Can simulate Tab navigation
   - [ ] Can simulate number key screen switching
   - [ ] Can simulate Esc key
   - [ ] Can test text input
   - [ ] Example tests written

---

## Next Phase Preview

**Phase 3: Sixel Graphics Support with Position Tracking**

After Phase 2, we'll implement:
- Sixel sequence detection via DCS callbacks
- Position tracking for Sixel graphics
- Bounds checking assertions
- Integration with dgx-pixels preview areas

Phase 3 depends on Phase 2 for:
- Event simulation to trigger Sixel rendering
- Wait conditions to detect Sixel appearance
- Async support for Tokio-based apps

---

## Resources

### Reference Documentation

- VT100 escape sequences: https://vt100.net/docs/vt100-ug/chapter3.html
- ANSI escape codes: https://en.wikipedia.org/wiki/ANSI_escape_code
- Tokio async: https://tokio.rs/tokio/tutorial
- bitflags crate: https://docs.rs/bitflags/

### Related Code

- crossterm KeyEvent: https://docs.rs/crossterm/latest/crossterm/event/struct.KeyEvent.html
- termion Key enum: https://docs.rs/termion/latest/termion/event/enum.Key.html
- term-transcript: https://github.com/slowli/term-transcript (CLI testing reference)

---

**Document Version**: 1.0
**Last Updated**: 2025-11-20
**Status**: Ready for Implementation
**Estimated Duration**: 1-2 weeks (10-14 days)
