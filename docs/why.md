# Why Minimal?

The case against bloated context files and external skills.

---

## Context Files Hurt Performance

Research suggests that pre-loaded context files may actually reduce task success rates while increasing costs.

**The pattern we see:**

| What happens | Why it hurts |
|--------------|--------------|
| Context duplicates what LLM knows | Wasted tokens, no benefit |
| Context gets stale | Conflicts with actual codebase |
| LLM follows our outdated rules | Extra work, worse outcomes |
| More tokens consumed | +20% inference cost |

**The alternative:** Let the LLM research in real-time. It reads current docs, explores the actual codebase, forms conclusions based on reality.

---

## External Skills Are Attack Vectors

Skills and plugins from external registries are a security risk.

**The attack pattern:**

1. Malicious skill gets uploaded to a registry
2. Skill contains "Prerequisites" or "Setup" instructions
3. Instructions tell LLM to run install commands
4. Commands exfiltrate secrets, install backdoors, or poison memory

**What gets stolen:**

- API keys and credentials
- SSH keys and tokens
- Browser passwords
- Agent memory files (enabling persistent behavioral changes)

**The fix:** Don't load external skills. No attack surface. If we need functionality, we build it ourselves or use vetted dependencies.

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

**todo.md** — Working memory. Current tasks. Ephemeral. Rewritten when complete.

This separates:
- What we prescribe (rules)
- What gets discovered (learnings)  
- What's in progress (todos)

---

## References

- [agents.md spec](https://agents.md/) — The AGENTS.md specification
- [arXiv:2602.11988](https://arxiv.org/abs/2602.11988) — Research on context file effectiveness
