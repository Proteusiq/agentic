<p align="center">
  <img src="https://readme-typing-svg.demolab.com?font=Fira+Code&weight=600&size=28&duration=3000&pause=1000&color=58A6FF&center=true&vCenter=true&random=false&width=500&lines=Stop+teaching+AI+your+codebase;Every.+Single.+Time." alt="Typing SVG" />
</p>

<h1 align="center">Agentic</h1>

<p align="center">
  <strong>Two files. Infinite context. Zero repetition.</strong>
</p>

<p align="center">
  <a href="https://github.com/Proteusiq/agentic/blob/main/LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue.svg" alt="License: MIT"></a>
  <a href="https://github.com/Proteusiq/agentic/actions/workflows/ci.yml"><img src="https://github.com/Proteusiq/agentic/actions/workflows/ci.yml/badge.svg" alt="CI"></a>
  <a href="https://proteusiq.github.io/agentic"><img src="https://img.shields.io/badge/docs-online-brightgreen.svg" alt="Docs"></a>
</p>

<p align="center">
  <a href="#-the-problem">Problem</a> •
  <a href="#-the-fix">Fix</a> •
  <a href="#-quick-start">Quick Start</a> •
  <a href="#-how-it-works">How It Works</a> •
  <a href="CONTRIBUTING.md">Contributing</a>
</p>

---

## The Problem

We've all been here. You open your AI editor, paste some code, and wait...

```
AI:  Here's the solution using requests...
You: We use httpx.
AI:  Here's the updated solution with classes...
You: We prefer functions.
AI:  Here's the refactored version...
You: We use ruff, not black.
AI:  *rewrites everything again*
```

Sound familiar? Every session. Every project. Every time.

AI agents are incredibly powerful, but they start each conversation knowing nothing about:

- Your preference for `X | None` over `Optional[X]`
- Your API's layered architecture
- That quirky `users.py` file that breaks everything if you touch it wrong
- *Anything* about how you actually work

Until you tell them. Again. And again.

---

## The Fix

```
your-project/
├── AGENTS.md     # How you write code
└── SKILL.md      # What this project is
```

That's it. Two markdown files. No fancy tooling required.

| File | Purpose | Sharing |
|------|---------|---------|
| **AGENTS.md** | Your development conventions | Symlink across all projects |
| **SKILL.md** | This repo's domain knowledge | Unique per project |

Drop them in your project root. Agents read them. They *get it*. First try.

---

## Quick Start

### Option 1: Just grab the files

```bash
curl -O https://raw.githubusercontent.com/Proteusiq/agentic/main/AGENTS.md
curl -O https://raw.githubusercontent.com/Proteusiq/agentic/main/SKILL.template.md
mv SKILL.template.md SKILL.md
# Edit SKILL.md with your project details
```

### Option 2: Use the scaffold CLI

```bash
# Install
git clone https://github.com/Proteusiq/agentic.git
cd agentic/scaffold && cargo build --release
cp target/release/scaffold /usr/local/bin/

# Use
scaffold -d ~/code/my-project --link
```

The `--link` flag symlinks `AGENTS.md` so all your projects share the same conventions. Change once, update everywhere.

<details>
<summary>CLI Options</summary>

```
scaffold [OPTIONS] [PROJECT_NAME]

  -d, --dir <DIR>    Target directory (default: current)
  -o, --org <ORG>    GitHub org (default: auto-detect from git)
  -l, --link         Symlink AGENTS.md instead of copying
  -s, --skill-only   Only create SKILL.md
  -a, --agents-only  Only create AGENTS.md
  -h, --help         Show help
```
</details>

---

## How It Works

### AGENTS.md — Your coding style, everywhere

This is *you* as a developer. Your preferences, your tools, your non-negotiables:

```markdown
## Do NOT

- Do NOT use `any` in TypeScript. Ever.
- Do NOT use `requests`. Use `httpx`.
- Do NOT use `black` or `mypy`. Use `ruff` and `ty`.

## Python — Before Commit

uv run ruff format . && uv run ruff check --fix . && uv run ty check .
```

Symlink it across projects:

```bash
ln -sfn ~/agentic/AGENTS.md ~/code/project-a/AGENTS.md
ln -sfn ~/agentic/AGENTS.md ~/code/project-b/AGENTS.md
```

One source of truth. All your projects. Always in sync.

### SKILL.md — This project's DNA

What makes *this* repository tick. The stuff that isn't obvious from reading the code:

```markdown
---
name: payment-api
description: Stripe integration for subscription billing
---

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

Agents read this and understand context immediately. No more explaining the same things over and over.

---

## Works With

| Agent | Setup |
|-------|-------|
| **Claude Code** | Reads `AGENTS.md` and `SKILL.md` from project root automatically |
| **OpenCode** | Symlink to `~/.config/opencode/skills/` |
| **Cursor** | Rename to `.cursorrules` or reference in settings |
| **Any LLM** | Paste contents or use as system prompt |

Want global skills? Make them available everywhere:

```bash
mkdir -p ~/.config/opencode/skills/my-api
ln -sfn ~/code/my-api/SKILL.md ~/.config/opencode/skills/my-api/SKILL.md
```

---

## What's Inside

### AGENTS.md includes

| Section | What you get |
|---------|--------------|
| **Do NOT** | Explicit anti-patterns — no ambiguity |
| **NEVER** | Destructive action guardrails |
| **Before You Code** | Plan → Search → Visualize → Code |
| **Language guides** | Python, Rust, TypeScript, Bash |
| **Git conventions** | Commits, branches, PRs |

### SKILL.template.md includes

| Section | Purpose |
|---------|---------|
| **Frontmatter** | Name + description for discovery |
| **Architecture** | How pieces connect |
| **Domain Rules** | Business logic that isn't in the code |
| **The Weird Parts** | Here be dragons |

---

## Philosophy

> **Simplicity is king.** The simplest solution that works is the best solution.

> **Self-documenting code.** If it needs comments, refactor it.

> **Functional over OOP.** Pure functions. Composition. Immutability.

> **Verify before commit.** If it's not tested, it's not done.

> **Plan mode over borrowed skills.** Research it yourself. Borrowed skills pollute context with stale assumptions.

---

## Contributing

We'd love your help making this better. See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

```bash
git clone https://github.com/Proteusiq/agentic.git
cd agentic
# Read AGENTS.md (obviously), then submit a PR
```

---

<p align="center">
  <sub>MIT License — Built for developers who believe AI should adapt to them, not the other way around.</sub>
</p>
