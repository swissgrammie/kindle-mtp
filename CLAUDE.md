# Project: kindle-mtp

## Quick Start
Read `docs/spec.md` first. Always.

## Development Philosophy
This project uses **spec-driven development**:
1. The spec is the source of truth
2. Design before implementing
3. Test before marking complete
4. Document deviations from spec

## Workflow

### Phase 1: Understand
- Read `docs/spec.md` completely
- Ask clarifying questions before coding
- Identify unknowns and risks

### Phase 2: Design  
- For non-trivial features, create `docs/architecture.md`
- Document key decisions in `docs/decisions/` as ADRs
- Define interfaces before implementations

### Phase 3: Implement
- Work in small, testable increments
- Commit frequently with clear messages
- Keep functions focused and testable

### Phase 4: Verify
- Write tests alongside implementation
- Manual testing for edge cases
- Update docs if implementation differs from spec

## Code Standards
- Clarity over cleverness
- Explicit over implicit  
- Document "why", not "what"
- Handle errors explicitly

## Project Structure
```
.
├── CLAUDE.md          # You are here
├── docs/
│   ├── spec.md        # Requirements (source of truth)
│   ├── architecture.md # Design decisions
│   └── decisions/     # ADRs for significant choices
├── src/               # Implementation
├── tests/             # Test files
└── scripts/           # Build/utility scripts
```

## Agent Usage
Custom agents are in `.claude/agents/`. Use them for:
- `architect` - Design decisions, API contracts
- `implementer` - Focused coding tasks
- `tester` - Test creation and coverage
- `reviewer` - Code review, security checks

Invoke with: "Use the architect agent to..."

## When Stuck
1. Re-read the relevant spec section
2. Check if there's an existing decision in `docs/decisions/`
3. If making a new significant decision, document it as an ADR
4. Ask for clarification rather than assuming
