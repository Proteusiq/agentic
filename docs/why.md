# Why Minimal?

The case against bloated context files.

---

## LLM Capabilities Keep Increasing

What we wrote 6 months ago? The model already knows it better now.

Our "best practices" doc is teaching the teacher.

---

## Tooling Evolves Faster Than Our Docs

Remember when we wrote "use `black` and `mypy`"? The Python community moved to `uv`, `ruff`, and `ty`.

Examples of outdated advice in the wild:
- "Use `pip install`" → `uv` is faster and better
- "Format with `black`" → `ruff format` does it faster
- "Type check with `mypy`" → `ty` is 10-100x faster
- "Use `requests`" → `httpx` supports async

Our AGENTS.md is now actively wrong.

---

## Agentic Workflows Can Discover

The LLM doesn't need to know everything upfront. With agentic workflows, it can:
- Read official documentation (current, not our stale copy)
- Explore the actual codebase
- Query databases and APIs
- Test endpoints and verify behavior
- Document what it finds in `learnings.md`

We don't need to pre-load generic knowledge. We need to elicit context *specific to our project* and let the LLM research the rest.

---

## Context Rot

Pre-loaded context is more than we bargained for.

A Polars SKILL.md with everything about the library when we only need 3% of it. Multiple files contributing less value than the tokens they cost.

Irrelevant context:
- Wastes tokens
- Distracts from the actual task
- Conflicts with project-specific needs
- Gets outdated while sitting in our repo

Tailor it to what we actually need. Let the LLM research the rest.

---

## What Should Go in AGENTS.md?

Only what the LLM doesn't know:

| Include | Exclude |
|---------|---------|
| Project-specific workflow | Language conventions |
| NEVER rules (destructive actions) | How to write good code |
| Where to document findings | Testing best practices |
| How to verify before commit | Git basics |

The LLM already knows Python style, TypeScript patterns, and git workflows. We don't need to teach it.

---

## The Three-File System

Instead of one bloated file, use three focused ones:

**AGENTS.md** — Rules we write. Minimal. Workflow + guardrails.

**learnings.md** — Knowledge the LLM discovers. Grows over time. Project-specific gotchas, working commands, patterns.

**todo.md** — Working memory. Current tasks. Ephemeral. Summarized when complete.

This separates:
- What we prescribe (rules)
- What gets discovered (learnings)
- What's in progress (todos)

---

## References

- [agents.md spec](https://agents.md/) — The AGENTS.md specification
