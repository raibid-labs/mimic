# Parallel Orchestration Plan v2 - Enhancement Issues

## Issue Dependency Graph

```
                    ┌─────────────────────────────────────────────┐
                    │          WAVE 1 (Core Enhancements)         │
                    │   No dependencies - Execute in parallel     │
                    └─────────────────────────────────────────────┘
                              │         │         │
                    ┌─────────┼─────────┼─────────┼─────────┐
                    ▼         ▼         ▼
              ┌─────────┐ ┌─────────┐ ┌─────────┐
              │  #17    │ │  #18    │ │  #20    │
              │ Mouse   │ │ Helpers │ │ Visual  │
              │ Events  │ │type_text│ │ Regress │
              └────┬────┘ └────┬────┘ └────┬────┘
                   │           │           │
                   └───────────┴───────────┘
                               │
                               ▼
         ┌─────────────────────────────────────────────┐
         │           WAVE 2 (Advanced Features)        │
         │       Can start after Wave 1 begins         │
         └─────────────────────────────────────────────┘
                    │         │         │
              ┌─────┴───┐ ┌───┴───┐ ┌───┴───┐
              ▼         ▼ ▼       ▼ ▼       ▼
        ┌─────────┐ ┌─────────┐ ┌─────────┐
        │  #19    │ │  #21    │ │  #24    │
        │ Timing  │ │ Debug   │ │ Memory  │
        │ Control │ │ Record  │ │ Profile │
        └────┬────┘ └────┬────┘ └────┬────┘
             │           │           │
             └───────────┴───────────┘
                         │
                         ▼
         ┌─────────────────────────────────────────────┐
         │         WAVE 3 (Infrastructure)             │
         │     Complex, architectural changes          │
         └─────────────────────────────────────────────┘
                    │                   │
              ┌─────┴─────┐       ┌─────┴─────┐
              ▼           ▼       ▼           ▼
        ┌─────────┐ ┌─────────┐
        │  #22    │ │  #23    │
        │ Multi-  │ │Parallel │
        │Terminal │ │ Tests   │
        └─────────┘ └─────────┘
```

## Wave 1: Core Enhancements (3 agents)

| Agent | Issue | Title | Priority |
|-------|-------|-------|----------|
| A1 | #17 | Mouse event simulation | Medium |
| A2 | #18 | Convenience helpers (type_text, etc.) | Medium |
| A3 | #20 | Visual regression testing | Medium |

## Wave 2: Advanced Features (3 agents)

| Agent | Issue | Title | Priority |
|-------|-------|-------|----------|
| B1 | #19 | Event timing control | Low |
| B2 | #21 | Debug/recording support | Medium |
| B3 | #24 | Memory profiling | Low |

## Wave 3: Infrastructure (2 agents)

| Agent | Issue | Title | Priority |
|-------|-------|-------|----------|
| C1 | #22 | Multi-terminal compatibility | Low |
| C2 | #23 | Parallel test execution | Medium |

## Total: 8 issues across 3 waves
