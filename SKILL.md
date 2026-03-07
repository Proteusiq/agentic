---
name: agentic
description: Two-file system for AI agent instructions and project knowledge
---

# Agentic

A reference library of AI agent instructions. Two files that teach AI agents your conventions and project context.

Repository: `Proteusiq/agentic`
Docs: `proteusiq.github.io/agentic`

## Architecture

```
AGENTS.md               ← universal dev conventions (symlink everywhere)
SKILL.template.md       ← template for project-specific knowledge
scaffold/               ← Rust CLI to set up both files
docs/                   ← Zensical documentation site
```

## Key Files

| File | Purpose |
|------|---------|
| `AGENTS.md` | Development conventions — persona, philosophy, Do NOT section, language-specific rules |
| `SKILL.template.md` | Template with placeholders for project-specific SKILL.md |
| `scaffold/src/main.rs` | Rust CLI — embeds templates, handles symlinks, validates input |
| `mkdocs.yml` | Zensical/MkDocs Material config for docs site |

## The Two-File System

| File | Scope | Sharing |
|------|-------|---------|
| `AGENTS.md` | How you code (universal) | Symlinked across all projects |
| `SKILL.md` | What this project is | Unique per repository |

## Scaffold CLI

Written in Rust. Embeds templates at compile time:

```rust
const AGENTS_TEMPLATE: &str = include_str!("../../AGENTS.md");
const SKILL_TEMPLATE: &str = include_str!("../../SKILL.template.md");
```

Key features:
- Auto-detects project name and org from git remote
- `--link` flag symlinks AGENTS.md to `~/.cache/agentic/AGENTS.md`
- Input validation against path traversal and shell injection
- Prints instructions for global skill setup (OpenCode, Claude Code)

## Domain Rules

### AGENTS.md Design
- Keep under 200 lines (agent adherence drops past this)
- Start with persona ("You are an expert...")
- "Do NOT" section is more effective than positive rules
- Exact commands, no ambiguity ("run X" not "format code")

### SKILL.md Design
- 100-300 lines sweet spot
- Architecture as ASCII diagrams
- Domain rules that aren't obvious from code
- "The Weird Parts" section for dragons/legacy

### Documentation
- Punchy, personality-driven (not corporate)
- Uses Zensical (mkdocs-material successor)
- Dark mode by default

## Development

```bash
# Docs
uv sync --group docs
uv run zensical serve

# Scaffold CLI
cd scaffold
cargo build --release
cargo test
```

## Deployment

- Docs: GitHub Actions deploys to GitHub Pages on push to `docs/` or `mkdocs.yml`
- Scaffold: Manual `cargo build --release`, copy binary to PATH

## The Weird Parts

### Scaffold symlink mode
Creates `~/.cache/agentic/AGENTS.md` and symlinks to it. Checks directory ownership to prevent symlink attacks in shared `/tmp`.

### Zensical vs MkDocs
Zensical reads `mkdocs.yml` natively but is a separate tool. Install with `pip install zensical`, run with `zensical serve` (not `mkdocs serve`).
