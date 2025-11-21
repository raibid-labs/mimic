# ROADMAP.md Phase 3 Update

## Changes to Apply

### Update Phase 2 Status (Line 136)

**Current**:
```markdown
**Status**: 🚀 **READY TO START** - Phase 1 Complete
```

**Update to**:
```markdown
**Status**: 🔄 **IN PROGRESS (85%)** - Event Simulation Complete, Async Support Pending
```

### Update Phase 2 Overview (After line 145)

**Add after "High-Level Tasks:"**:
```markdown
**Current Progress**: Phase 2 is 85% complete
- ✅ Event simulation foundation (100%)
- ✅ KeyCode enum and Modifiers bitflags (100%)
- ✅ Escape sequence generation (100%)
- ✅ Harness event methods (send_key, send_keys) (100%)
- ✅ Enhanced wait conditions (100%)
- 🔶 Async support (pending - see PHASE2_CHECKLIST.md)
```

### Replace Phase 3 Section (Lines 236-292)

**Replace entire Phase 3 section with**:

```markdown
### Phase 3: Sixel Graphics Support with Position Tracking

**Goal**: Enable Sixel testing with position verification and bounds checking

**Priority**: P0 (Critical - MVP Blocker, Original Motivation)

**Status**: 🎯 **READY TO START** - Phase 1 Complete, Phase 2 Mostly Complete

**Dependencies**: Phase 1 ✅ (vtparse integrated), Phase 2 🔶 (85% complete)

**Architecture Documents**:
- **[PHASE3_CHECKLIST.md](../PHASE3_CHECKLIST.md)** - Comprehensive implementation checklist
- **[SIXEL_PARSING_STRATEGY.md](./SIXEL_PARSING_STRATEGY.md)** - Parsing algorithm design
- **[PHASE3_VALIDATION_API.md](./PHASE3_VALIDATION_API.md)** - API specification
- **[PHASE3_TEST_STRATEGY.md](./PHASE3_TEST_STRATEGY.md)** - Testing plan

**Detailed Planning**: See PHASE3_CHECKLIST.md for complete task breakdown (7 sections, 50+ tests)

**High-Level Tasks**:

1. **Sixel Raster Attribute Parsing** (Days 1-2)
   - Enhance existing parse_raster_attributes() in src/screen.rs
   - Handle missing/malformed sequences with fallbacks
   - Add dimension validation and clamping
   - Implement pixel-to-cell conversion
   - Unit tests for all edge cases

2. **Position Tracking Enhancement** (Days 3-4)
   - Validate cursor position capture (already implemented)
   - Add cell dimension calculation
   - Update SixelRegion with width_cells, height_cells fields
   - Implement bounds checking methods (is_within_cells, overlaps_cells)
   - Integration tests with real Sixel sequences

3. **Validation API Implementation** (Days 5-7)
   - Add harness methods:
     - assert_sixel_within_bounds()
     - get_sixel_at()
     - sixel_count()
     - verify_sixel_cleared()
   - Enhance SixelCapture with query methods:
     - sequences_overlapping()
     - sequences_at_row()
     - has_sequences_in()
     - total_coverage()
     - bounding_box()
   - Comprehensive integration tests

4. **Test Fixtures & Data** (Days 8-9)
   - Create tests/fixtures/sixel/ directory
   - Generate or obtain real Sixel test sequences:
     - minimal_10x10.sixel
     - red_100x50.sixel
     - blue_200x100.sixel
     - gradient_150x150.sixel
     - large_500x500.sixel
   - Implement fixture loading helpers
   - Document fixture usage

5. **Integration Testing** (Days 10-11)
   - Expand tests/integration/sixel.rs
   - Test Sixel detection and parsing
   - Test position tracking accuracy
   - Test bounds validation
   - Test clearing detection
   - Test with all fixtures

6. **dgx-pixels Scenario Validation** (Days 12-13)
   - Create tests/integration/dgx_pixels_scenarios.rs
   - Test Gallery preview area validation
   - Test screen transitions
   - Test multiple thumbnail images
   - Test boundary conditions
   - Document dgx-pixels patterns

7. **Documentation & Polish** (Days 14-15)
   - Complete API rustdoc (100% coverage)
   - Create docs/SIXEL_TESTING.md user guide
   - Update examples/sixel_test.rs
   - Create examples/dgx_pixels_preview.rs
   - CI/CD integration
   - Performance benchmarks

**Success Criteria**:
- [x] ✅ vtparse DCS callbacks working (Phase 1)
- [x] ✅ Cursor position tracking (Phase 1)
- [x] 🔶 SixelRegion struct with basic fields (Phase 1)
- [ ] Raster attributes parsed correctly (>95% accuracy)
- [ ] Pixel-to-cell conversion accurate
- [ ] Position tracking validated
- [ ] All validation APIs implemented
- [ ] Can verify Sixel within bounds (preview area)
- [ ] Can detect Sixel outside bounds
- [ ] Can detect Sixel clearing on screen change
- [ ] All unit tests pass (>30 tests)
- [ ] All integration tests pass (>15 tests)
- [ ] All E2E tests pass (>5 dgx-pixels scenarios)
- [ ] Code coverage >70% for Phase 3 code
- [ ] **Can prevent dgx-pixels Sixel bugs**

**Implementation Status**:

**Infrastructure Ready** ✅:
- vtparse integration with DCS callbacks (dcs_hook, dcs_put, dcs_unhook)
- VTActor implementation in TerminalState
- Cursor position tracking via VTActor
- SixelRegion struct: start_row, start_col, width, height, data
- sixel_regions() and has_sixel_at() accessors
- SixelSequence and SixelCapture types
- Basic integration tests

**Needs Implementation** 🔶:
- Enhance parse_raster_attributes() (stub exists)
- Add pixel-to-cell conversion
- Implement validation APIs in harness
- Create test fixtures
- Expand integration tests
- dgx-pixels scenario tests
- Documentation

**Estimated Effort**: 2-3 weeks (15 days)

**Timeline Breakdown**:
- Week 1: Parsing, conversion, bounds checking
- Week 2: Validation APIs, fixtures, integration tests
- Week 3: dgx-pixels validation, documentation, polish

**Risk Assessment**: ✅ **LOW**
- vtparse DCS support validated in Phase 1
- Cursor tracking working
- Architecture fully designed
- Clear implementation path

**Key Deliverables**:
1. Enhanced Sixel parsing (src/screen.rs)
2. Validation APIs (src/harness.rs, src/sixel.rs)
3. Test fixtures (tests/fixtures/sixel/)
4. Integration tests (50+ tests)
5. dgx-pixels scenarios (5 E2E tests)
6. User guide (docs/SIXEL_TESTING.md)
7. API documentation (100% rustdoc)
```

### Update Timeline (Line 630)

**Current**:
```markdown
- **Phase 3**: ⏳ Pending (2-3 weeks)
```

**Update to**:
```markdown
- **Phase 3**: 🎯 Ready to Start (2-3 weeks) - Architecture Complete
```

### Update Current Position (Line 638)

**Current**:
```markdown
**Current Position**: Week 4 of 16 (25% complete)
```

**Update to**:
```markdown
**Current Position**: Week 5-6 of 16 (30-35% complete)
  - Phase 1: ✅ Complete (100%)
  - Phase 2: 🔄 In Progress (85%)
  - Phase 3: 🎯 Ready to Start (architecture complete)
```

---

## Summary of Changes

### Status Updates
- ✅ Phase 1: Complete (100%)
- 🔄 Phase 2: In Progress (85% - event simulation done, async pending)
- 🎯 Phase 3: Ready to Start (architecture documents complete)

### New Documentation References
- PHASE3_CHECKLIST.md - 7 sections, comprehensive implementation guide
- SIXEL_PARSING_STRATEGY.md - Algorithm design and edge cases
- PHASE3_VALIDATION_API.md - Complete API specification
- PHASE3_TEST_STRATEGY.md - 50+ tests planned

### Timeline
- Phase 3 can begin immediately (Phase 2 async is independent)
- Estimated: 2-3 weeks (15 days)
- Risk: Low (infrastructure ready, design complete)

---

**Instructions for applying these changes**:
1. Update Phase 2 status line to reflect 85% progress
2. Replace Phase 3 section with new detailed version
3. Update timeline estimates
4. Add cross-references to new Phase 3 docs
