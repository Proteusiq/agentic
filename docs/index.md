# Let's Stop the Markdown Hell

We don't need SKILL.md, ARCHITECTURE.md, CONVENTIONS.md, .cursorrules, or dozens of AI instruction files.

**We need one file: [AGENTS.md](https://agents.md/)** — and it should be minimal.

---

## The Problem

Every tool, every directory wants its own file:

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

They get bloated. They get stale. **That's markdown hell.**

---

## Why It Fails

### 1. LLM Capabilities Keep Increasing

What we wrote 6 months ago? The model already knows it better now. Our "best practices" doc is teaching the teacher.

### 2. Tooling Evolves Faster Than Our Docs

Remember when we wrote "use `black` and `mypy`"? The Python community moved to `uv`, `ruff`, and `ty`. Our AGENTS.md is now actively wrong.

### 3. Agentic Workflows Can Discover

The LLM doesn't need to know everything upfront. It can:
- Read official documentation (current, not our stale copy)
- Explore the actual codebase
- Query databases and APIs
- Document what it finds

### 4. Context Rot

Pre-loaded context is more than we bargained for. A Polars SKILL.md with everything about the library when we only need 3% of it. Multiple files contributing less value than the tokens they cost.

---

## The Fix: RTFM

Read The Ducking Manual. In real-time.

Three files. That's it.

| File | Purpose | Who writes it |
|------|---------|---------------|
| `AGENTS.md` | Rules and workflow | Us |
| `learnings.md` | Discovered knowledge | LLM (during work) |
| `todo.md` | Current tasks in progress | LLM (working memory) |

**AGENTS.md** is prescriptive — workflow + NEVER rules. Nothing the LLM already knows.

**learnings.md** is descriptive — what the LLM discovered while working. Patterns, gotchas, commands that work. This grows over time.

**todo.md** is ephemeral — tracks current work. Summarized when tasks complete.

---

## Why This Works

| This Approach | Markdown Hell |
|---------------|---------------|
| Agent reads current docs | Agent reads our stale summary |
| Agent explores the codebase | Agent follows our outdated overview |
| Agent forms conclusions | Agent inherits our assumptions |
| Agent adapts to changes | Agent conflicts with reality |
| Agent documents findings | Knowledge lost each session |

The agent is capable. Let it research. Let it learn. Let it document what it discovers.

---

## Get Started

```bash
curl -O https://raw.githubusercontent.com/Proteusiq/agentic/main/AGENTS.md
touch learnings.md todo.md
```

See [agents.md](https://agents.md/) for the spec.
