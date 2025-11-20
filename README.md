# term-test

> A Rust library for integration testing of terminal user interface (TUI) applications with first-class support for Ratatui and graphics protocols like Sixel.

## Overview

`term-test` bridges the gap between unit testing with Ratatui's `TestBackend` and real-world integration testing of TUI applications. It provides a PTY-based test harness that enables testing of features requiring actual terminal escape sequence processing, including Sixel image rendering, mouse events, terminal resize handling, and complex user interaction flows.

### Why term-test?

**Current Limitation**: Ratatui's `TestBackend` is great for unit testing widgets and layouts, but it can't test:
- PTY-specific behavior (terminal size negotiation, TTY detection)
- Graphics protocols (Sixel, iTerm2 images, Kitty graphics)
- Real terminal integration
- User interaction flows
- Event handling in actual terminal context

**Solution**: `term-test` runs your TUI application in a real pseudo-terminal (PTY), captures the output using a terminal emulator, and provides an ergonomic API for assertions and snapshot testing.

### Key Features

- **PTY-Based Testing**: Real terminal environment using `portable-pty`
- **Graphics Support**: First-class Sixel testing with validation and comparison
- **Event Simulation**: Keyboard, mouse, and terminal resize events
- **Smart Waiting**: Condition-based waiting with timeouts
- **Snapshot Testing**: Integration with `insta` and `expect-test`
- **Async Support**: Test async Ratatui apps with Tokio or async-std
- **Ratatui Helpers**: Widget-specific assertions and layout verification
- **Cross-Platform**: Linux, macOS, and Windows support

## Status

**🚧 This project is in the research and design phase. See [ROADMAP.md](./docs/ROADMAP.md) for implementation plan.**

## Quick Example

```rust
use term_test::TuiTestHarness;
use std::process::Command;

#[test]
fn test_navigation() -> Result<()> {
    let mut harness = TuiTestHarness::new(80, 24)?;

    // Spawn your TUI app
    harness.spawn(Command::new("./my-tui-app"))?;

    // Wait for initial render
    harness.wait_for(|state| {
        state.contents().contains("Main Menu")
    })?;

    // Simulate user input
    harness.send_key(Key::Down)?;
    harness.send_key(Key::Enter)?;

    // Verify result
    harness.wait_for(|state| {
        state.contents().contains("Settings")
    })?;

    Ok(())
}
```

## Testing Sixel Graphics

```rust
use term_test::{TuiTestHarness, SixelCapture};

#[test]
fn test_image_rendering() -> Result<()> {
    let mut harness = TuiTestHarness::new(80, 40)?;
    harness.spawn(Command::new("./image-viewer"))?;

    harness.send_text("open test.png\n")?;

    // Capture and validate Sixel output
    let sixel = SixelCapture::from_screen(&harness.state)?;
    sixel.validate()?;

    // Compare with reference
    let expected = SixelCapture::from_file("tests/fixtures/expected.six")?;
    sixel.compare(&expected)?;

    Ok(())
}
```

## Documentation

### 📚 Core Documentation

- **[RESEARCH.md](./docs/RESEARCH.md)** - Comprehensive research on existing terminal testing solutions, parsing libraries (VTE, vt100, termwiz), PTY libraries (portable-pty), snapshot testing frameworks (insta, expect-test), and Sixel testing approaches. Essential background for understanding the problem space.

- **[ARCHITECTURE.md](./docs/ARCHITECTURE.md)** - Complete library architecture including:
  - Layer design (PTY management, terminal emulation, test harness, snapshot integration, Ratatui helpers)
  - Module structure and API design
  - Example usage patterns
  - Dependencies and feature flags
  - Error handling strategy
  - Performance considerations

- **[EXISTING_SOLUTIONS.md](./docs/EXISTING_SOLUTIONS.md)** - Analysis of existing Ratatui testing approaches:
  - Ratatui's TestBackend (unit testing)
  - Snapshot testing with insta/expect-test
  - term-transcript (CLI testing)
  - tui-term (pseudoterminal widget)
  - Comparison matrix showing gaps that term-test fills

- **[TESTING_APPROACHES.md](./docs/TESTING_APPROACHES.md)** - Comprehensive guide to TUI testing methodologies:
  - The testing pyramid for TUI applications
  - Unit testing vs integration testing vs E2E testing
  - Snapshot testing patterns
  - PTY-based testing strategies
  - Sixel/graphics testing
  - Async/event-driven testing
  - Property-based testing for TUIs
  - Testing strategy recommendations for different application types

- **[ROADMAP.md](./docs/ROADMAP.md)** - Detailed implementation roadmap from MVP to 1.0:
  - 8 development phases with clear milestones
  - Version planning (v0.1.0 through v1.0.0)
  - Dependency specifications
  - Risk mitigation strategies
  - Success metrics
  - Future enhancements (record/replay, visual regression, fuzzing)

### 🎯 Quick Navigation

| Topic | Document | Key Sections |
|-------|----------|--------------|
| **Getting Started** | ARCHITECTURE.md | Example Usage, Dependencies |
| **Understand the Problem** | EXISTING_SOLUTIONS.md | Gap Analysis, Comparison Matrix |
| **Testing Strategies** | TESTING_APPROACHES.md | Testing Pyramid, Common Patterns |
| **Technical Research** | RESEARCH.md | VTE vs vt100, PTY Libraries, Sixel Testing |
| **Implementation Plan** | ROADMAP.md | Phases, Milestones, Timeline |

## How term-test Complements Existing Tools

| Testing Level | Use This | For What |
|---------------|----------|----------|
| **Unit Tests** | Ratatui's TestBackend + insta | Individual widgets, layout calculations |
| **Integration Tests** | **term-test** | Full app behavior, PTY interaction, graphics |
| **CLI Tests** | assert_cmd | Binary execution, exit codes |
| **Snapshot Tests** | insta or expect-test | Both unit and integration levels |

`term-test` is **complementary, not competitive** - it fills the integration testing gap that TestBackend cannot address.

## Project Goals

1. **Ease of Use**: Simple API that gets out of your way
2. **Comprehensive**: Test all terminal features including graphics
3. **Cross-Platform**: Reliable on Linux, macOS, and Windows
4. **Well-Documented**: Examples for every use case
5. **Battle-Tested**: High test coverage and production-ready

## Comparison with Ratatui's TestBackend

| Feature | TestBackend | term-test |
|---------|-------------|-----------|
| **Speed** | Very Fast | Moderate |
| **Setup Complexity** | Simple | Moderate |
| **PTY Testing** | ❌ | ✅ |
| **Graphics (Sixel)** | ❌ | ✅ |
| **Widget Unit Tests** | ✅ | ✅ |
| **Integration Tests** | ❌ | ✅ |
| **Event Simulation** | Limited | Full |
| **Async Support** | Basic | Full |
| **Snapshot Testing** | Via insta/expect | Built-in |

**Recommendation**: Use TestBackend for unit tests, term-test for integration tests.

## Architecture Highlights

### Layer Structure

```
┌─────────────────────────────────────────┐
│   Ratatui Integration Helpers (Layer 5) │ Widget assertions, layout verification
├─────────────────────────────────────────┤
│   Snapshot Testing (Layer 4)            │ insta/expect-test integration
├─────────────────────────────────────────┤
│   Test Harness (Layer 3)                │ TuiTestHarness, event simulation
├─────────────────────────────────────────┤
│   Terminal Emulation (Layer 2)          │ vt100 parser, screen state
├─────────────────────────────────────────┤
│   PTY Management (Layer 1)              │ portable-pty wrapper
└─────────────────────────────────────────┘
```

### Core Dependencies

- **portable-pty**: Cross-platform PTY creation (from WezTerm)
- **vt100**: Terminal emulation and escape sequence parsing
- **insta/expect-test**: Snapshot testing (optional)
- **tokio/async-std**: Async runtime support (optional)

See [ARCHITECTURE.md](./docs/ARCHITECTURE.md) for complete details.

## Roadmap Summary

### Near-term Milestones

- **v0.1.0** - Basic PTY harness, text-based testing
- **v0.2.0** - Full event simulation, smart waiting
- **v0.3.0** - Snapshot testing integration
- **v0.4.0** - Async support (Tokio, async-std)
- **v0.5.0** - Sixel and graphics testing
- **v0.6.0** - Ratatui-specific helpers
- **v1.0.0** - Production ready, complete documentation

See [ROADMAP.md](./docs/ROADMAP.md) for the complete implementation plan.

## Contributing

**This project is in the design phase.** Feedback on the architecture and approach is welcome!

Once implementation begins:
- Check the [ROADMAP.md](./docs/ROADMAP.md) for current phase
- Look for "good first issue" labels
- Read CONTRIBUTING.md (to be created)

## Research Acknowledgments

This project builds on excellent work from:
- **Ratatui** - The TUI framework we're testing
- **WezTerm** - Source of portable-pty and termwiz
- **Alacritty** - Source of VTE parser
- **vt100-rust** - Terminal emulation library
- **insta** - Snapshot testing framework
- The broader Rust TUI ecosystem

Special thanks to the maintainers of these projects for their well-documented, reusable components.

## Related Projects

- [ratatui](https://github.com/ratatui/ratatui) - Rust library for cooking up TUIs
- [WezTerm](https://github.com/wez/wezterm) - GPU-accelerated terminal emulator
- [Alacritty](https://github.com/alacritty/alacritty) - GPU-accelerated terminal emulator
- [vt100-rust](https://github.com/doy/vt100-rust) - Parser for terminal byte streams
- [tui-term](https://github.com/a-kenji/tui-term) - Pseudoterminal widget for Ratatui
- [term-transcript](https://github.com/slowli/term-transcript) - CLI/REPL snapshot testing

## License

TBD (likely MIT or MIT/Apache-2.0 dual license)

## Contact

- **Issues**: [GitHub Issues](https://github.com/[user]/term-test/issues)
- **Discussions**: [GitHub Discussions](https://github.com/[user]/term-test/discussions)

---

**Status**: 🚧 Research & Design Phase - See [ROADMAP.md](./docs/ROADMAP.md) for implementation timeline
