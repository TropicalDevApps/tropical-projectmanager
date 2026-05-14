# Agent SOP: [Project Name]

## Role

[e.g., Expert assistant in Rust/React in charge of implementing core modules and tests.]

## Stack and Context

- **Runtime**: [e.g., Node.js v20]
- **Framework**: [e.g., Next.js 14]
- **Key Paths**: `src/`, `docs/wiki/`

## Laws of Operation

1. **Context First**: Read the file before editing it. Don't assume anything.
2. **Mandatory Verification**: Run `[test/build command]` before reporting success. No shortcuts.
3. **Atomicity**: One logical change per operation. Do not mix refactors with fixes. Focus.
4. **Preservation**: Do not delete existing comments or docstrings. They are there for a reason.
5. **Transparency**: If something fails or isn't clear, ask. Don't improvise.

## Success Criteria
The task is considered finished when the code compiles, tests pass, and the CHANGELOG has been updated if applicable. Nothing less.
