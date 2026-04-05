# You Know Nothing, AGENTS.md

Our 500-line AGENTS.md full of "best practices"? The LLM already knows. We're paying 20% more tokens to tell it how to write Python.

**The LLM is not Jon Snow.** We are.

---

## The Problem

We pre-load context that:

1. **The LLM already knows** — Python conventions, TypeScript patterns, git workflows
2. **Gets stale** — our "current" architecture doc is 6 months old
3. **Conflicts with reality** — the codebase evolved, our rules didn't
4. **Costs more** — [+20% inference overhead](https://arxiv.org/abs/2602.11988) for worse results

Stop telling the LLM what it knows. Start telling it what it doesn't.

---

## The Fix

Three files. That's it.

| File | Purpose | Who writes |
|------|---------|------------|
| `AGENTS.md` | Rules + guardrails | Us |
| `learnings.md` | Discovered knowledge | The LLM |
| `todo.md` | Working memory | The LLM |

**AGENTS.md** — Minimal. Workflow + NEVER rules. Nothing the LLM already knows.

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
    └──▶ Read official docs, not our stale summary
```

The LLM researches in real-time. Documents what it finds. We keep the knowledge.

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
