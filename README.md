<p align="center">
  <strong>MINIMAL RULES</strong> · <strong>LLM DISCOVERS</strong> · <strong>KNOWLEDGE PERSISTS</strong>
  <br><br>
  <em>Stop teaching the teacher. Let it research.</em>
</p>

# You Know Nothing, AGENTS.md

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Docs](https://img.shields.io/badge/docs-online-blue.svg)](https://proteusiq.github.io/agentic)

> *"You know nothing, Jon Snow."*
> — Ygritte, Game of Thrones

Our bloated AGENTS.md full of generic "best practices"? The LLM already knows. We're paying extra tokens to tell it how to write Python.

**The LLM is not Jon Snow.** We are.

---

## Issues with Markdown Hell

You know the pattern:

```
AGENTS.md
CLAUDE.md
CURSOR.md
CONVENTIONS.md
ARCHITECTURE.md
CONTRIBUTING.md
SKILL.md
.cursorrules
.github/copilot-instructions.md
code-reviewer.md
testing.md
api-conventions.md
pokemon.md
...
```

Every tool, every directory wants its own file. We copy "best practices" into each one. They get bloated. They get stale. **That's markdown hell.**

### 1. LLM Capabilities Keep Increasing

What we wrote 6 months ago? The model already knows it better now. Our "best practices" doc is teaching the teacher.

### 2. Tooling Evolves Faster Than Our Docs

Remember when we wrote "use `black` and `mypy`"? The Python community moved to `uv`, `ruff`, and `ty`. Our AGENTS.md is now actively wrong.

> [!WARNING]
> Examples of outdated advice in the wild:
> - "Use `pip install`" → `uv` is faster and better
> - "Format with `black`" → `ruff format` does it faster
> - "Type check with `mypy`" → `ty` is 10-100x faster
> - "Use `requests`" → `httpx` supports async

### 3. Agentic Workflows Can Discover

The LLM doesn't need to know everything upfront. With agentic workflows, it can:
- Read official documentation (current, not our stale copy)
- Explore the actual codebase
- Query databases and APIs
- Test endpoints and verify behavior
- Document what it finds in `learnings.md`

We don't need to pre-load generic knowledge. We need to elicit context *specific to our project* and let the LLM research the rest.

### 4. Context Rot

Pre-loaded context is more than we bargained for. A Polars SKILL.md with everything about the library when we only need 3% of it. Multiple files contributing less value than the tokens they cost.

Irrelevant context:
- Wastes tokens
- Distracts from the actual task
- Conflicts with project-specific needs
- Gets outdated while sitting in our repo

Tailor it to what we actually need. Let the LLM research the rest.

---

## The Fix: RTFM

Instead of markdown hell, go old school. Read The Ducking Manual. Update our learnings. Document our todos.

| File | Purpose | Who writes |
|------|---------|------------|
| `AGENTS.md` | Rules + guardrails | Us |
| `learnings.md` | Discovered knowledge | The LLM |
| `todo.md` | Working memory | The LLM |

> [!NOTE]
> **AGENTS.md** — Workflow + NEVER rules. Nothing the LLM already knows.
>
> **learnings.md** — Project-specific gotchas, working commands, patterns. The LLM documents what it discovers. This persists. This is the actual value.
>
> **todo.md** — Current tasks. Ephemeral. Summarized when complete.

> [!TIP]
> Still love your markdowns? See [examples](https://proteusiq.github.io/agentic/examples/) for what `learnings.md` looks like in practice.

---

## The Workflow

```
Research → Discover → Document → Build → Verify → Commit
    │          │          │
    │          │          └──▶ learnings.md (persists)
    │          │
    │          └──▶ Query DB, test APIs, explore codebase
    │
    └──▶ Read official docs (current), not our summary (stale)
```

The LLM researches in real-time. Reads current documentation. Documents what it finds. We keep the knowledge.

---

## Example: Python Project from Scratch

Starting a new Python project? Point your agentic tool to these files:

```
Use https://github.com/Proteusiq/agentic/blob/main/AGENTS.md as rules.
Use https://github.com/Proteusiq/agentic/blob/main/docs/examples/python.md as initial learnings.
Create todo.md for tracking tasks.
```

The LLM reads the rules, starts with Python-specific knowledge, and builds from there.

> [!IMPORTANT]
> See [agents.md](https://agents.md/) for the spec.

---

## Docs

**[proteusiq.github.io/agentic](https://proteusiq.github.io/agentic)**

| Page | Description |
|------|-------------|
| [Why](https://proteusiq.github.io/agentic/why/) | Why minimal is better |
| [Workflow](https://proteusiq.github.io/agentic/workflow/) | The research → discover → document loop |
| [Research](https://proteusiq.github.io/agentic/research/) | What the arXiv papers say |
| [Skills vs Learnings](https://proteusiq.github.io/agentic/skills-vs-learnings/) | Anthropic's approach vs ours |
| [Examples](https://proteusiq.github.io/agentic/examples/) | Ready-to-use `learnings.md` templates |

---

## In Good Company?

Even the best fall into AI markdown hell:

| Project | Developer | AI Files |
|---------|-----------|----------|
| [Ghostty](https://github.com/ghostty-org/ghostty) | Mitchell Hashimoto | `AGENTS.md`, `AI_POLICY.md`, `.agents/commands/`, `.agents/skills/` |
| [Claude Code](https://github.com/anthropics/claude-code) | Anthropic | [Source code leaked](https://github.com/x1xhlol/system-prompts-and-models-of-ai-tools) — plugins, skills, extensive internal markdown |

Everyone's in markdown hell. The question is how deep.

---

## License

MIT — Prayson Wilfred Daniel
