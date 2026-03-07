<p align="center">
  <img src="https://readme-typing-svg.demolab.com?font=Fira+Code&weight=600&size=28&duration=3000&pause=1000&color=58A6FF&center=true&vCenter=true&random=false&width=500&lines=Stop+teaching+AI+your+codebase;Every.+Single.+Time." alt="Typing SVG" />
</p>

<h1 align="center">Agentic</h1>

<p align="center">
  <strong>Two files. Infinite context. Zero repetition.</strong>
</p>

<p align="center">
  <a href="#the-problem">Problem</a> •
  <a href="#the-fix">Fix</a> •
  <a href="#quick-start">Quick Start</a> •
  <a href="#how-it-works">How It Works</a>
</p>

---

## The Problem

You open your AI editor. You paste your code. You wait.

```
AI:  Here's the solution using `requests`...
You: We use httpx.
AI:  Here's the updated solution with classes...
You: We prefer functions.
AI:  Here's the refactored version...
You: We use ruff, not black.
AI:  *rewrites everything again*
```

**You've done this dance before.** Every session. Every project. Every time.

AI agents are powerful. But they're flying blind:

- They don't know you prefer `X | None` over `Optional[X]`
- They don't know your API has a layered architecture
- They don't know that `users.py` has a quirk that breaks everything
- They don't know *anything* until you tell them. Again.

---

## The Fix

```
your-project/
├── AGENTS.md     # How you write code
└── SKILL.md      # What this project is
```

That's it. Two markdown files.

**AGENTS.md** = Your development conventions. Symlink it across all your projects.

**SKILL.md** = This repo's domain knowledge. Lives in each project root.

Agents read these files. They *get it*. First try.

---

## Quick Start

### Install

```bash
# Clone and build
git clone https://github.com/Proteusiq/agentic.git
cd agentic/scaffold
cargo build --release

# Install to PATH
cp target/release/scaffold /usr/local/bin/
```

### Use

```bash
# Scaffold into your project
scaffold -d ~/code/my-project

# Use --link to symlink AGENTS.md (recommended)
scaffold -d ~/code/my-project --link

# Done. Your agent now knows your conventions.
```

### Options

```
scaffold [OPTIONS] [PROJECT_NAME]

  -d, --dir <DIR>    Target directory (default: current)
  -o, --org <ORG>    GitHub org (default: auto-detect from git)
  -l, --link         Symlink AGENTS.md instead of copying
  -s, --skill-only   Only create SKILL.md
  -a, --agents-only  Only create AGENTS.md
  -h, --help         Show help
```

Or grab files directly:

```bash
curl -O https://raw.githubusercontent.com/Proteusiq/agentic/main/AGENTS.md
curl -O https://raw.githubusercontent.com/Proteusiq/agentic/main/SKILL.template.md
mv SKILL.template.md SKILL.md
```

---

## How It Works

### AGENTS.md — Your Rules

Universal conventions that apply everywhere you code:

```markdown
## Do NOT

- Do NOT use `any` in TypeScript. Ever.
- Do NOT use `requests`. Use `httpx`.
- Do NOT use `black` or `mypy`. Use `ruff` and `ty`.
- Do NOT commit without running the verification loop.

## Python — Before Commit

uv run ruff format . && uv run ruff check --fix . && uv run ty check . && uv run pytest
```

One file. All projects. Symlink it:

```bash
ln -sfn ~/agentic/AGENTS.md ~/code/project-a/AGENTS.md
ln -sfn ~/agentic/AGENTS.md ~/code/project-b/AGENTS.md
ln -sfn ~/agentic/AGENTS.md ~/code/project-c/AGENTS.md
```

Change once. Update everywhere.

### SKILL.md — Project DNA

What makes *this* repository tick:

```yaml
---
name: payment-api
description: Stripe integration for subscription billing
---
```

```markdown
## Architecture

settings.py → services/ → db/ → api/

## Domain Rules

- Prices stored in cents, displayed in dollars
- Never delete subscriptions, cancel them
- Webhook signatures must be verified

## The Weird Parts

- `legacy_pricing.py` uses the old Stripe API. Don't touch it.
- Tests require `STRIPE_TEST_KEY` in env
```

Agents read this. They understand. They don't ask stupid questions.

---

## Works With

Skills follow the [Agent Skills](https://agentskills.io) open standard.

| Agent | How to Enable |
|-------|---------------|
| **Claude Code** | Reads `SKILL.md` automatically, or symlink to `~/.claude/skills/` |
| **OpenCode** | Symlink to `~/.config/opencode/skills/` |
| **Cursor** | Reads `.cursorrules` (rename or copy) |
| **Any LLM** | Paste the file contents, or use as system prompt |

Make skills global:

```bash
# Now your agent knows about my-api even when you're in another project
mkdir -p ~/.config/opencode/skills/my-api
ln -sfn ~/code/my-api/SKILL.md ~/.config/opencode/skills/my-api/SKILL.md
```

---

## What's In The Box

### AGENTS.md

| Section | What's There |
|---------|--------------|
| **Do NOT** | Explicit anti-patterns. No ambiguity. |
| **Before You Code** | Plan → Search → Visualize → Then code |
| **Python** | `uv` + `ruff` + `ty` + `pytest` |
| **Rust** | `cargo` + `bacon` + `clippy` |
| **TypeScript** | `bun`/`deno` + `biome` + `zod` |
| **Git** | Conventional commits. One change per PR. |

### SKILL.template.md

| Section | Purpose |
|---------|---------|
| **Frontmatter** | Name + description for agent discovery |
| **Architecture** | How the pieces connect |
| **Domain Rules** | Business logic that isn't obvious from code |
| **The Weird Parts** | Where the dragons live |

---

## The Philosophy

These aren't suggestions. They're convictions.

> **Simplicity is king.** The simplest solution that works is the best solution.

> **Self-documenting code.** If it needs comments, refactor it.

> **Functional over OOP.** Pure functions. Composition. Immutability.

> **Verify before commit.** If it's not tested, it's not done.

---

## Why Two Files?

| File | What | Shared? |
|------|------|---------|
| `AGENTS.md` | How you code | Yes — same everywhere |
| `SKILL.md` | What this is | No — unique per repo |

Update conventions once → every project gets them.
Keep domain knowledge focused → agents load fast.

---

## Contributing

```bash
git clone https://github.com/Proteusiq/agentic.git
cd agentic

# Follow AGENTS.md (obviously)
# Submit a PR
```

---

<p align="center">
  <sub>MIT License</sub>
</p>

<p align="center">
  <sub>Built for developers who believe AI should adapt to their workflow, not the other way around.</sub>
</p>
