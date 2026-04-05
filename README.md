# You Know Nothing, AGENTS.md

Your 500-line AGENTS.md full of "best practices"? The model already knows. You're paying 20% more tokens to tell Claude how to write Python.

**The model is not Jon Snow.** You are.

---

## The Problem

You pre-load context that:

1. **The model already knows** — Python conventions, TypeScript patterns, git workflows
2. **Gets stale** — your "current" architecture doc is 6 months old
3. **Conflicts with reality** — the codebase evolved, your rules didn't
4. **Costs more** — [+20% inference overhead](https://arxiv.org/abs/2602.11988) for worse results

Stop telling the model what it knows. Start telling it what it doesn't.

---

## The Fix

Three files. That's it.

| File | Purpose | Who writes |
|------|---------|------------|
| `AGENTS.md` | Rules + guardrails | You |
| `learnings.md` | Discovered knowledge | The model |
| `todo.md` | Working memory | The model |

**AGENTS.md** — Minimal. Workflow + NEVER rules. Nothing the model already knows.

**learnings.md** — Project-specific gotchas, working commands, patterns. Grows over time. This is the actual value.

**todo.md** — Current tasks. Ephemeral. Rewritten when complete.

---

## The Workflow

```
Research → Discover → Document → Build → Verify → Commit
    │          │          │
    │          │          └──▶ learnings.md (persists)
    │          │
    │          └──▶ Query DB, test APIs, explore codebase
    │
    └──▶ Read official docs, not your stale summary
```

The model researches in real-time. Documents what it finds. You keep the knowledge.

**Docs:** [proteusiq.github.io/agentic](https://proteusiq.github.io/agentic)

---

## Get Started

```bash
curl -O https://raw.githubusercontent.com/Proteusiq/agentic/main/AGENTS.md
touch learnings.md todo.md
```

See [agents.md](https://agents.md/) for the spec.

---

## License

MIT
