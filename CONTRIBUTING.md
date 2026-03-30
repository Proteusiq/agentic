# Contributing

Thanks for your interest in improving Agentic.

## Quick Start

```bash
git clone https://github.com/Proteusiq/agentic.git
cd agentic
```

## What to Contribute

| Area | What We Need |
|------|--------------|
| **AGENTS.md** | Language-specific rules, tooling updates, better defaults |
| **SKILL.template.md** | Architecture patterns, domain rule examples |
| **scaffold/** | Bug fixes, new flags, cross-platform support |
| **docs/** | Clarity, examples, agent compatibility guides |

## Before You Start

1. **Check existing issues** — someone may already be working on it
2. **Open an issue first** for significant changes
3. **Read AGENTS.md** — follow the conventions you're contributing to

## Development

### Documentation

```bash
uv sync --group docs
uv run zensical serve
```

### Scaffold CLI (Rust)

```bash
cd scaffold
cargo build
cargo test
cargo clippy -- -D warnings
```

## Submitting Changes

### Commit Messages

```
type: short description
```

Types: `feat`, `fix`, `docs`, `refactor`, `test`, `chore`

### Pull Requests

1. Fork and branch from `main`
2. Follow the conventions in `AGENTS.md`
3. Run verification before pushing:
   - Docs: `uv run zensical build`
   - Rust: `cargo fmt && cargo clippy && cargo test`
4. Open a PR with a clear description

### PR Description Template

```markdown
## What

Brief description of the change.

## Why

The problem this solves or improvement it makes.

## Verification

Commands run locally to verify the change works.
```

## Code Style

See `AGENTS.md`. The short version:

- Simplicity over cleverness
- Types everywhere
- Test what matters
- No magic

## Questions?

Open an issue. We're friendly.
