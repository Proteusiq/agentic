# Agentic

**Give your AI coding agents the context they need to ship quality code.**

Most AI tools fail not because they can't code, but because they don't understand *your* codebase, *your* conventions, *your* domain. Agentic fixes this with a simple two-file system that turns any AI agent into a knowledgeable collaborator.

```
your-project/
├── AGENTS.md     # How you write code (universal conventions)
└── SKILL.md      # What this project is (domain knowledge)
```

---

## The Problem

You paste code into ChatGPT. It responds with `var` instead of `const`, uses `requests` instead of `httpx`, and names everything `data`. You've been here before.

AI agents are powerful, but they're flying blind. They don't know:
- Your team prefers functional programming over OOP
- You use `ruff` and `ty`, not `black` and `mypy`  
- Your API follows a specific layered architecture
- That one module has a quirk that breaks everything

**Agentic gives agents institutional knowledge.**

---

## The Solution

Two markdown files. That's it.

### `AGENTS.md` — Your Development Conventions

Language-agnostic principles and language-specific rules that apply everywhere:

```markdown
## Philosophy
- Simplicity is king
- Self-documenting code
- Functional over OOP

## Python
### Before Commit
uv run ruff format .
uv run ruff check --fix .
uv run ty check .
uv run pytest
```

### `SKILL.md` — Project-Specific Knowledge

Everything an agent needs to understand *this* repository:

```markdown
---
name: my-api
description: REST API for widget management
---

## Architecture
settings.py → services/ → db/ → api/

## Domain Rules
- Widgets must have unique SKUs
- Prices are stored in cents, displayed in dollars
- Never delete widgets, soft-delete only
```

---

## Quick Start

### Option 1: Scaffold (Recommended)

```bash
git clone https://github.com/Proteusiq/agentic.git
cd agentic

# Initialize in your project
./scaffold -d /path/to/your/project
```

### Option 2: Manual

```bash
# Copy the conventions file
curl -O https://raw.githubusercontent.com/Proteusiq/agentic/main/AGENTS.md

# Copy the skill template
curl -O https://raw.githubusercontent.com/Proteusiq/agentic/main/SKILL.template.md
mv SKILL.template.md SKILL.md

# Edit SKILL.md with your project details
```

---

## Using Skills with AI Agents

Skills follow the [Agent Skills](https://agentskills.io) open standard. Make them available globally:

```bash
# OpenCode
mkdir -p ~/.config/opencode/skills/my-project
ln -sfn ~/code/my-project/SKILL.md ~/.config/opencode/skills/my-project/SKILL.md

# Claude Code
mkdir -p ~/.claude/skills/my-project
ln -sfn ~/code/my-project/SKILL.md ~/.claude/skills/my-project/SKILL.md
```

Now your agent knows about `my-project` even when you're working elsewhere.

---

## What's Inside

### AGENTS.md Covers

| Section | What It Contains |
|---------|------------------|
| **Philosophy** | Core principles: simplicity, self-documenting code, functional patterns |
| **Cross-Language** | Error handling, testing (AAA pattern), documentation, git workflow |
| **Python** | `uv`, `ruff`, `ty`, `pytest` — modern tooling with examples |
| **Rust** | `cargo`, `bacon`, `clippy` — idiomatic patterns |
| **TypeScript** | Bun + Biome or Deno — both paths covered |
| **Bash** | `shellcheck`, `shfmt`, defensive scripting |
| **Git** | Conventional commits, PR guidelines |

### SKILL.md Template Covers

| Section | Purpose |
|---------|---------|
| **YAML Frontmatter** | Name and description for agent discovery |
| **Project Overview** | What the project does, production URLs |
| **Architecture** | Layer diagram showing dependency flow |
| **Source Layout** | Directory tree with explanations |
| **Key Files** | Important files and their purposes |
| **Domain Rules** | Business logic, validation, conventions |
| **Testing** | Commands and patterns |
| **Common Issues** | Known problems and solutions |

---

## Scaffold CLI

```bash
./scaffold [OPTIONS] [PROJECT_NAME]

Options:
  -d, --dir DIR      Target directory (default: current)
  -o, --org ORG      GitHub org (default: auto-detect)
  -s, --skill-only   Only create SKILL.md
  -a, --agents-only  Only create AGENTS.md  
  -l, --link         Symlink AGENTS.md instead of copying
  -h, --help         Show help
```

**Tip:** Use `--link` to maintain a single source of truth for your conventions across all projects.

---

## Why Two Files?

| Concern | File | Shared? |
|---------|------|---------|
| How you write code | `AGENTS.md` | Yes — symlink across projects |
| What this project is | `SKILL.md` | No — unique per repository |

This separation means:
- Update conventions once, propagate everywhere
- Keep project knowledge focused and scannable
- Agents load only what they need

---

## Design Principles

These aren't just suggestions—they're the philosophy baked into every template:

**Simplicity is king.** The simplest solution that works is the best solution.

**Self-documenting code.** If it needs comments, refactor it.

**Functional over OOP.** Pure functions, composition, immutability.

**Commit early, commit often.** Small, focused, verified commits.

---

## Contributing

```bash
git clone https://github.com/Proteusiq/agentic.git
cd agentic

# Make changes
# Follow the conventions in AGENTS.md (of course)

# Submit a PR
```

---

## License

MIT

---

*Built for developers who believe AI should adapt to their workflow, not the other way around.*
