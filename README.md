# You Know Nothing, AGENTS.md

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Docs](https://img.shields.io/badge/docs-online-blue.svg)](https://proteusiq.github.io/agentic)

Our 500-line AGENTS.md full of "best practices"? The LLM already knows. We're paying 20% more tokens to tell it how to write Python.

**The LLM is not Jon Snow.** We are.

---

## Why Markdown Hell Fails

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

### 3. It's Already Baked In

Modern LLMs know:
- Language conventions (Python, TypeScript, Rust, Go)
- Framework patterns (FastAPI, React, Django)
- Git workflows and commit conventions
- Testing best practices
- Error handling patterns

We don't need to teach this. We need to tell the LLM what's *specific to our project*.

### 4. Research Confirms It

Context files [reduce task success rates and increase cost by 20%](https://arxiv.org/abs/2602.11988). More context = worse results.

---

## The Fix

Three files. Minimal rules. Let the LLM research.

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

## Get Started

```bash
curl -O https://raw.githubusercontent.com/Proteusiq/agentic/main/AGENTS.md
touch learnings.md todo.md
```

> [!IMPORTANT]
> See [agents.md](https://agents.md/) for the spec.

**Docs:** [proteusiq.github.io/agentic](https://proteusiq.github.io/agentic)

---

## License

MIT
