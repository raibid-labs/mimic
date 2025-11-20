# CLAUDE.md - AI Collaboration Log

## Project Overview

**term-test** is a Rust library for integration testing of terminal user interface (TUI) applications, particularly those built with Ratatui. The library enables testing features that require actual terminal escape sequence processing, including Sixel image rendering, which cannot be tested with Ratatui's built-in `TestBackend`.

## Project Genesis

### Original Problem Statement

In a previous session, the user mentioned:

> "Regarding tests: The tests I added earlier focused on state management and UI layout (text-based). Sixel rendering involves actual terminal escape sequences that can't be easily tested in unit tests without a real terminal. The proper way to test this would be integration tests with a terminal emulator, which is beyond the scope of the current testing infrastructure."

This led to the creation of this repository to build the testing infrastructure that was previously out of scope.

### User Requirements

The user requested:
1. Create a Rust library for integration testing TUI behavior with WezTerm (or its underlying terminal engine)
2. Make it importable into other rust-ratatui projects
3. Ultra-deep research and propose a path forward
4. Document research and design in `./docs`
5. Update README.md with summary and TOC
6. Initialize this CLAUDE.md file
7. Commit and push all changes

## Work Completed (Session 2025-11-19)

### Phase 1: Research (Completed)

Conducted comprehensive research across multiple domains:

1. **WezTerm Architecture**
   - Discovered modular architecture with reusable crates (portable-pty, termwiz, vtparse)
   - Identified portable-pty as ideal for cross-platform PTY support
   - Found that WezTerm components are designed to be used independently

2. **Terminal Parsing Libraries**
   - **VTE (from Alacritty)**: Low-level parser, table-driven, very fast
   - **vt100-rust**: Higher-level with built-in screen buffer, ideal for testing
   - **termwiz (from WezTerm)**: Comprehensive but tightly coupled to WezTerm

3. **Existing Ratatui Testing Solutions**
   - **TestBackend**: Built-in, fast, but limited to in-memory text rendering
   - **insta/expect-test**: Snapshot testing frameworks, recommended by Ratatui
   - **term-transcript**: CLI testing tool, but can't handle complex TUIs
   - **tui-term**: Pseudoterminal widget (for embedding terminals, not testing)
   - **Ratatui.cs**: .NET binding with headless testing (demonstrates the concept)

4. **PTY Libraries**
   - **portable-pty**: Cross-platform, from WezTerm, well-maintained
   - Other options exist but are less popular/maintained

5. **Sixel Testing**
   - Identified test suites: libsixel images, Jexer test suite
   - Tools: lsix for detection, arewesixelyet.com for compatibility tracking
   - Programmatic detection methods from Julia and R packages

6. **Testing Methodologies**
   - Unit testing with TestBackend
   - Snapshot testing for visual regression
   - PTY-based integration testing
   - Graphics protocol testing
   - Async/event-driven testing
   - Property-based testing

### Phase 2: Architecture Design (Completed)

Designed a 5-layer architecture:

1. **Layer 1 - PTY Management**: portable-pty wrapper for creating PTYs
2. **Layer 2 - Terminal Emulation**: vt100 parser for screen state
3. **Layer 3 - Test Harness**: High-level API for spawning apps and assertions
4. **Layer 4 - Snapshot Integration**: insta/expect-test integration
5. **Layer 5 - Ratatui Helpers**: Optional widget-specific utilities

Key design decisions:
- Use vt100 over VTE for higher-level abstractions
- Use portable-pty for proven cross-platform support
- Integrate (not replace) existing snapshot testing frameworks
- Provide async support for modern Ratatui apps
- Make Sixel testing a first-class feature

### Phase 3: Documentation (Completed)

Created comprehensive documentation:

1. **RESEARCH.md** (3,800+ words)
   - Detailed analysis of all researched libraries
   - Comparison of terminal parsers
   - PTY library evaluation
   - Snapshot testing frameworks
   - Sixel testing approaches
   - Key insights and recommendations

2. **ARCHITECTURE.md** (4,200+ words)
   - Complete layer-by-layer design
   - Module structure
   - API examples for all major use cases
   - Dependencies and feature flags
   - Error handling strategy
   - Performance considerations
   - Alternative approaches considered

3. **EXISTING_SOLUTIONS.md** (5,000+ words)
   - Deep dive into Ratatui's TestBackend
   - Snapshot testing with insta/expect-test
   - Analysis of term-transcript
   - Analysis of tui-term
   - Comparison matrix
   - Gap analysis showing what's missing
   - How term-test fits into the ecosystem

4. **TESTING_APPROACHES.md** (5,500+ words)
   - Testing pyramid for TUI applications
   - 7 different testing approaches with examples
   - Testing strategy recommendations
   - Common patterns (TDD, BDD, AAA)
   - Test organization best practices
   - Debugging failed tests
   - CI/CD integration

5. **ROADMAP.md** (4,000+ words)
   - 8 implementation phases with clear deliverables
   - Version milestones (v0.1.0 through v1.0.0)
   - Dependency specifications
   - Risk mitigation strategies
   - Success metrics
   - Future enhancements

6. **README.md** (Updated)
   - Project overview and motivation
   - Status and quick examples
   - Documentation table of contents
   - Comparison matrices
   - Architecture highlights
   - Roadmap summary
   - Related projects and acknowledgments

7. **CLAUDE.md** (This file)
   - Project context
   - Work completed
   - Key decisions
   - Next steps

## Key Decisions and Rationale

### 1. Use vt100 over VTE

**Decision**: Use vt100-rust for terminal emulation

**Rationale**:
- Higher-level API with built-in screen buffer management
- Designed for the "parse and verify" use case
- Simpler integration than VTE which requires custom Perform trait implementation
- Trade-off: Less popular than VTE, but better fit for testing

### 2. Use portable-pty

**Decision**: Use portable-pty for PTY management

**Rationale**:
- Battle-tested in WezTerm
- Cross-platform (Linux, macOS, Windows)
- Actively maintained
- Runtime-selectable implementations via traits
- No need to reinvent this wheel

### 3. Integrate, Don't Replace

**Decision**: Integrate with insta/expect-test rather than building custom snapshot framework

**Rationale**:
- Both frameworks are well-established
- Users already familiar with them
- Don't reinvent snapshot testing
- Focus on the unique value: PTY + graphics testing

### 4. Complementary, Not Competitive

**Decision**: Position term-test as complementary to TestBackend

**Rationale**:
- TestBackend is excellent for unit tests
- term-test fills the integration testing gap
- Different tools for different jobs
- Reduces adoption friction

### 5. Sixel as First-Class Feature

**Decision**: Make Sixel testing a core feature, not an afterthought

**Rationale**:
- No existing solution tests graphics protocols
- Sixel support is growing in terminal ecosystem
- This is a unique differentiator
- Aligns with original problem statement

## Gap Analysis: What Was Missing

Before term-test, the Ratatui testing ecosystem had these gaps:

1. **No PTY-based integration testing** - TestBackend is in-memory only
2. **No graphics protocol testing** - Sixel rendering untestable
3. **Poor integration testing UX** - TestBackend requires manual buffer inspection
4. **No full E2E testing** - Can't test complete user flows
5. **Limited async support** - No async-aware test harness

term-test addresses all of these gaps.

## Technical Challenges Identified

### 1. Sixel Support in vt100

**Challenge**: vt100 crate may not support Sixel parsing

**Mitigation Options**:
- Contribute Sixel support to vt100
- Fork and extend vt100
- Use custom parser for Sixel sequences
- Use termwiz which has broader protocol support

**Recommendation**: Evaluate vt100's Sixel support early in Phase 1. If insufficient, consider termwiz despite tighter WezTerm coupling.

### 2. Cross-Platform PTY Differences

**Challenge**: PTY behavior varies across platforms

**Mitigation**:
- Extensive cross-platform testing
- Use portable-pty's abstraction layer
- Document platform-specific quirks
- CI testing on all platforms

### 3. Timing and Race Conditions

**Challenge**: Integration tests may have timing issues

**Mitigation**:
- Robust timeout handling
- Condition-based waiting (not time-based)
- Async support for better control
- Comprehensive error messages for debugging

### 4. Test Determinism

**Challenge**: PTY-based tests may be less deterministic than unit tests

**Mitigation**:
- Control terminal size explicitly
- Use deterministic input sequences
- Wait for specific conditions, not arbitrary delays
- Snapshot testing to catch regressions

## Implementation Priority

### Phase 1 (Critical): Core PTY Harness
- Must have basic PTY functionality working
- Cross-platform testing essential
- Foundation for everything else

### Phase 2 (Critical): Event Simulation
- Essential for useful testing
- Smart waiting is key to reliability

### Phase 3 (Critical): Snapshot Integration
- Makes tests maintainable
- Provides familiar UX for users

### Phase 4 (High): Async Support
- Modern Ratatui apps are async
- Important for adoption

### Phase 5 (High): Sixel Support
- Original motivation
- Unique differentiator

### Phase 6 (Medium): Ratatui Helpers
- Nice to have, not essential
- Can be added by community

### Phase 7 (Critical): Polish
- Required for 1.0
- Documentation, error messages, performance

### Phase 8 (Medium): Community
- Ongoing after 1.0
- Essential for long-term success

## Potential Risks

### Technical Risks

1. **vt100 lacks Sixel support**
   - Impact: High
   - Probability: Medium
   - Mitigation: Have termwiz as backup plan

2. **PTY timing issues**
   - Impact: Medium
   - Probability: Medium
   - Mitigation: Robust waiting, timeouts, async support

3. **Memory leaks in PTY handling**
   - Impact: Medium
   - Probability: Low
   - Mitigation: Proper cleanup, leak detection tests

### Project Risks

1. **Scope creep**
   - Impact: Medium
   - Probability: Medium
   - Mitigation: Phased approach, MVP first

2. **Maintenance burden**
   - Impact: Medium
   - Probability: Medium
   - Mitigation: Good architecture, tests, docs

3. **Low adoption**
   - Impact: Low
   - Probability: Low
   - Mitigation: Solve real problem, quality first, community engagement

## Success Criteria

### Technical
- [ ] Test coverage > 80%
- [ ] Works on Linux, macOS, Windows
- [ ] Zero critical bugs at 1.0
- [ ] Performance overhead < 10% vs raw PTY
- [ ] Complete API documentation

### Adoption
- Downloads on crates.io
- GitHub stars
- Community contributions
- Projects using term-test
- Listed in Ratatui ecosystem

## Next Steps

### Immediate (Before Implementation)

1. **Validate vt100 Sixel support**
   - Check if vt100 can parse Sixel sequences
   - Test with sample Sixel output
   - Decide: vt100 vs termwiz

2. **Create minimal prototype**
   - Just PTY + vt100 integration
   - Prove the concept works
   - Validate cross-platform

3. **Community feedback**
   - Share design with Ratatui community
   - Get feedback on API design
   - Validate assumptions

### Phase 1 Tasks (See ROADMAP.md)

1. Project setup
2. PTY management layer
3. Terminal emulation layer
4. Basic test harness
5. Testing & documentation

## Open Questions

1. **License**: MIT or MIT/Apache-2.0 dual license?
   - Recommendation: MIT/Apache-2.0 dual (Rust standard)

2. **Repository owner**: Where should this live?
   - Options: Personal account, Ratatui org, new org
   - Recommendation: Start personal, transfer if Ratatui wants it

3. **Governance**: Who maintains this?
   - Start: Original author
   - Goal: Multiple maintainers, community-driven

4. **Funding**: Is this a funded project?
   - Consider: GitHub Sponsors, OpenCollective
   - Probably not needed initially

## Resources and References

### Crates
- [portable-pty](https://docs.rs/portable-pty) - PTY management
- [vt100](https://docs.rs/vt100) - Terminal emulation
- [vte](https://docs.rs/vte) - Alternative parser
- [insta](https://docs.rs/insta) - Snapshot testing
- [expect-test](https://docs.rs/expect-test) - Snapshot testing
- [ratatui](https://docs.rs/ratatui) - TUI framework

### Documentation
- [Ratatui Testing Guide](https://ratatui.rs/recipes/testing/snapshots/)
- [Testing TUI Apps](https://blog.waleedkhan.name/testing-tui-apps/)
- [Integration Testing TUI](https://quantonganh.com/2024/01/21/integration-testing-tui-app-in-rust.md)
- [Are We Sixel Yet](https://www.arewesixelyet.com/)
- [Jexer Sixel Tests](https://jexer.sourceforge.io/sixel.html)

### Repositories
- [WezTerm](https://github.com/wez/wezterm)
- [Alacritty VTE](https://github.com/alacritty/vte)
- [vt100-rust](https://github.com/doy/vt100-rust)
- [Ratatui](https://github.com/ratatui/ratatui)

## Session Notes

### Research Methodology

Used web search to gather information on:
- WezTerm architecture and components
- Rust TUI testing solutions
- VT100 and terminal emulation libraries
- PTY libraries and testing
- Sixel protocol and testing
- Existing Ratatui testing approaches

### Documentation Approach

Structured documentation to serve multiple audiences:
- **RESEARCH.md**: For understanding the problem space and existing solutions
- **ARCHITECTURE.md**: For implementers who will build the library
- **EXISTING_SOLUTIONS.md**: For users wondering why term-test is needed
- **TESTING_APPROACHES.md**: For users learning how to test TUIs
- **ROADMAP.md**: For contributors and stakeholders tracking progress
- **README.md**: For first-time visitors to understand the project

### Writing Style

- Technical but accessible
- Heavy use of examples
- Comparison tables for quick scanning
- Code samples for clarity
- Links to original sources

## Future Sessions

### Questions to Address

1. **vt100 Sixel support**: Does it work? Do we need termwiz instead?
2. **API ergonomics**: Should we have a builder pattern for harness configuration?
3. **Error types**: Should we use thiserror or anyhow or custom?
4. **Async runtime**: Should we support both Tokio and async-std equally?

### Implementation Priorities

1. Start with Phase 1 (Core PTY Harness)
2. Validate cross-platform early
3. Create working examples ASAP
4. Get community feedback before going too far

### Documentation Gaps

- CONTRIBUTING.md (Phase 1)
- CODE_OF_CONDUCT.md (Phase 1)
- Security policy (Phase 7)
- Detailed API docs (ongoing)

## Conclusion

This session completed the research and design phase for term-test. The library has a clear purpose (integration testing for Ratatui TUIs), a well-defined architecture (5 layers), comprehensive documentation (>20,000 words), and a concrete roadmap (8 phases to 1.0).

The next session should focus on:
1. Setting up the Rust project structure
2. Validating vt100 Sixel support
3. Creating a minimal working prototype
4. Beginning Phase 1 implementation

The foundation is solid. Time to build.

---

**Last Updated**: 2025-11-19
**Session Duration**: ~1 hour
**Lines of Documentation**: ~1,500
**Research Sources**: 20+ web searches, 10+ crates evaluated
**Status**: ✅ Research & Design Complete → Ready for Implementation
