# Philosophy

Why two files? Why not one? Why not twenty?

---

## The Insight

AI agents read everything you give them. But they can't remember anything between sessions.

Every time you start fresh, they're back to zero. No memory of your preferences. No understanding of your architecture. No knowledge of your domain.

So developers repeat themselves. Over and over.

**Agentic fixes this with two files:**

| File | Purpose | Scope |
|------|---------|-------|
| `AGENTS.md` | How you write code | Global — same across all projects |
| `SKILL.md` | What this project is | Local — unique per repository |

That's the split. Conventions vs. context.

---

## Why Two Files?

### One file is too rigid

If you put everything in one file, you either:

1. **Duplicate conventions** across every project (pain to maintain)
2. **Mix conventions with context** (too long, agents lose focus)

Two files solve both problems. Symlink your conventions. Keep context local.

### Three+ files is too scattered

Some teams try:

```
.cursorrules
AGENTS.md
SKILL.md
ARCHITECTURE.md
DOMAIN.md
CONVENTIONS.md
```

Now agents don't know what's important. Token budgets overflow. Critical info gets truncated.

**Two files is the sweet spot.** Small enough to fit in context. Comprehensive enough to be useful.

---

## The Conventions File

`AGENTS.md` captures **how you code** — independent of any specific project:

```markdown
## Do NOT

- Do NOT use `any` in TypeScript
- Do NOT use `requests`. Use `httpx`.
- Do NOT commit without running the verification loop.

## Python — Before Commit

uv run ruff format . && uv run ruff check --fix . && uv run pytest
```

This file:

- Lives in one place (your dotfiles, a central repo)
- Gets symlinked to every project
- Changes once, updates everywhere

---

## The Skills File

`SKILL.md` captures **what this project is** — the domain knowledge that makes it unique:

```markdown
## Architecture

settings.py → services/ → db/ → api/

## Domain Rules

- Prices stored in cents, displayed in dollars
- Never delete subscriptions, cancel them
- Webhook signatures must be verified
```

This file:

- Lives in each project root
- Is unique to that repository
- Contains knowledge that would take hours to rediscover

---

## Design Principles

### Under 200 Lines

Claude's team found that agents follow shorter instructions better. Past 200 lines, adherence drops.

Both files should be scannable. If you can't find what you need in 5 seconds, it's too long.

### Explicit Anti-Patterns

"Use X" is weaker than "Do NOT use Y."

Agents hallucinate toward popular patterns. If you want `httpx` over `requests`, say "Do NOT use requests" — not just "prefer httpx."

### Copy-Paste Ready

Every command should be runnable. No "configure appropriately" — give the exact command:

```bash
# Good
uv run ruff format . && uv run ruff check --fix .

# Bad
"Run the linter before committing"
```

### Domain Rules Over Code Comments

Don't explain code in SKILL.md. Explain the *business logic* that isn't obvious from reading code:

```markdown
# Bad — describes code
"The calculate_price function multiplies by 100"

# Good — explains domain
"Prices stored in cents, displayed in dollars"
```

---

## The Research

This approach is informed by:

| Source | Finding |
|--------|---------|
| [Claude Code](https://docs.anthropic.com/en/docs/claude-code) | Keep AGENTS.md under 200 lines |
| [Agent Skills spec](https://agentskills.io) | Frontmatter + sections = discoverability |
| [Cursor](https://cursor.com) | `.cursorrules` proved project context works |
| Real-world usage | Anti-patterns ("Do NOT") more effective than positive rules |

---

## The Loop

Everything comes back to one mantra:

```
Change → Format → Lint → Type Check → Test → Commit
```

This is the verification loop. It appears in AGENTS.md. It gets reinforced in every session.

Run it before every commit. No exceptions.

---

## Summary

```
┌─────────────────────────────────────────────────────┐
│  AGENTS.md                                          │
│  - Your conventions                                 │
│  - Symlinked everywhere                             │
│  - Under 200 lines                                  │
│  - "Do NOT" anti-patterns                           │
│  - Exact commands, no ambiguity                     │
├─────────────────────────────────────────────────────┤
│  SKILL.md                                           │
│  - Project-specific                                 │
│  - Domain knowledge                                 │
│  - Architecture + rules + quirks                    │
│  - What would take hours to rediscover              │
└─────────────────────────────────────────────────────┘
```

Two files. That's the system.
