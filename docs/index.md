# Let's Stop the Markdown Hell

You don't need SKILL.md, ARCHITECTURE.md, CONVENTIONS.md, .cursorrules, or external skills.

**You need one file: AGENTS.md** (or CONTRIBUTING.md).

And it should be minimal.

---

## Issue 1: Security

Skills and context files are attack vectors.

In January 2026, security researchers found **341 malicious skills (12% of the registry)** on ClawHub:

> "Skills can exfiltrate data, poison memory, and persist backdoors... The payload delivery mechanism was deceptively simple: a 'Prerequisites' section instructing users to install additional components."
>
> — [Snyk Security Research](https://snyk.io/articles/skill-md-shell-access/)

What got stolen:

- API keys and wallet credentials
- SSH keys and browser passwords
- Agent memory files (enabling persistent behavioral changes)

**External skills = external attack surface.** Don't load them.

---

## Issue 2: Context Rot

Pre-loaded context becomes stale. Models get smarter.

A February 2026 study evaluated AGENTS.md files across multiple coding agents:

> "Context files are redundant documentation... LLM-generated context files have a marginal negative effect on task success rates."
>
> — [arXiv:2602.11988](https://arxiv.org/abs/2602.11988)

The research found:

| Finding | Evidence |
|---------|----------|
| Context files are redundant | 100% of generated files duplicate existing docs |
| Repository overviews don't help | No meaningful reduction in steps to find relevant files |
| Instructions get followed but hurt | More testing, more exploration, same or worse outcomes |

Your 300-line AGENTS.md contains opinions that:

1. The model already knows (Python style, TypeScript rules)
2. Conflict with what the codebase actually does
3. Become outdated as the project evolves

---

## Issue 3: Complexity for No Gain

More context = worse results + higher cost.

From the same study:

> "Context files tend to **reduce task success rates** compared to providing no repository context, while also **increasing inference cost by over 20%**."

The numbers:

| Metric | With Context Files | Without |
|--------|-------------------|---------|
| Task success | Lower | **Higher** |
| Inference cost | +20-23% | Baseline |
| Steps to complete | More | **Fewer** |
| Reasoning tokens | +14-22% more | Baseline |

The agent spends tokens reading your rules, following your instructions, running extra tests you demanded — and solves fewer problems.

---

## The Fix: RTFM

Read The Ducking Manual. In real-time.

```markdown
# Development Conventions

You are an expert software engineer. You write clean, maintainable code. You think before you code.

---

## How to Work

1. **Research first** — read official docs, explore the codebase, understand before changing
2. **Document findings** — update `learnings.md` with what you discover (gotchas, working commands, patterns)
3. **Track future work** — push non-blocking items to `todo.md`

That's it. No external skills, no pre-loaded context. Research in real-time, document as you go.

---

## NEVER (Destructive Actions)

These actions require **explicit user confirmation**. Stop and ask before proceeding.

- NEVER run destructive database commands (`DROP`, `DELETE`, `TRUNCATE`) on production
- NEVER commit secrets, API keys, tokens, or credentials to git
- NEVER force push to `main` or `master`
- NEVER run `rm -rf` on directories you didn't create in this session
- NEVER deploy to production without explicit approval

If an action could cause data loss, expose secrets, or affect production: **stop, explain, ask**.
```

**27 lines.** Workflow + guardrails. Nothing else.

---

## The Three Files

| File | Purpose | Who writes it |
|------|---------|---------------|
| `AGENTS.md` | Rules and workflow | Human |
| `learnings.md` | Discovered knowledge, gotchas, working commands | LLM (during work) |
| `todo.md` | Current tasks in progress | LLM (working memory) |

**AGENTS.md** is prescriptive — what to do and what not to do.

**learnings.md** is descriptive — what the LLM discovered while working. Patterns, gotchas, commands that work. This grows over time.

**todo.md** is ephemeral — tracks current work. Rewritten/summarized when tasks complete to stay minimal.

---

## Why This Works

| This Approach | Markdown Hell |
|---------------|---------------|
| Agent reads current docs | Agent reads your stale summary |
| Agent explores the codebase | Agent follows your outdated overview |
| Agent forms conclusions | Agent inherits your assumptions |
| Agent adapts to changes | Agent conflicts with reality |
| Agent documents findings | Knowledge lost each session |
| No attack surface | External skills = vulnerabilities |
| Lower cost | +20% inference overhead |

The agent is capable. Let it research. Let it learn. Let it document what it discovers.

---

## Get Started

1. Copy [AGENTS.md](https://github.com/Proteusiq/agentic/blob/main/AGENTS.md) to your project root
2. Create empty `learnings.md` and `todo.md`
3. Done

See [agents.md](https://agents.md/) for the spec.
