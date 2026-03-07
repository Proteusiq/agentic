# Writing AGENTS.md

Your conventions file. The one that follows you everywhere.

---

## The Goal

Write once. Symlink everywhere. Every AI agent knows your preferences from line one.

```bash
ln -sfn ~/agentic/AGENTS.md ~/code/project-a/AGENTS.md
ln -sfn ~/agentic/AGENTS.md ~/code/project-b/AGENTS.md
ln -sfn ~/agentic/AGENTS.md ~/code/project-c/AGENTS.md
```

Change the source. Every project updates.

---

## Structure

A good AGENTS.md has these sections:

| Section | Purpose |
|---------|---------|
| **Persona** | Set the mental model ("You are an expert...") |
| **Philosophy** | Core principles (3-4 bullets max) |
| **Do NOT** | Explicit anti-patterns |
| **NEVER** | Destructive actions requiring confirmation |
| **Quick Reference** | Scannable command table |
| **Language Sections** | Per-language tools and style |
| **Git** | Commit format, PR rules |
| **The Loop** | Verification mantra |

---

## Start With a Persona

The opening line sets the tone:

```markdown
You are an expert software engineer. You write clean, maintainable code.
You think before you code.
```

This isn't fluff. It primes the agent's behavior. Without it, you get generic output.

---

## Philosophy First

State your convictions. Keep it to 3-4 bullets:

```markdown
## Philosophy

- **Simplicity is king** — the simplest solution that works is the best solution
- **Self-documenting code** — if it needs comments, refactor it
- **Functional over OOP** — pure functions, composition, immutability
- **Verify before commit** — if it's not tested, it's not done
```

These aren't suggestions. They're the foundation everything else builds on.

---

## The Do NOT Section

This is the most powerful part of your file.

AI agents default to popular patterns. If you want something different, you have to explicitly reject the common path.

```markdown
## Do NOT

- Do NOT jump straight into code. Plan first.
- Do NOT use `any` in TypeScript. Ever.
- Do NOT use `Optional[X]` in Python. Use `X | None`.
- Do NOT use `requests`. Use `httpx`.
- Do NOT use `black` or `mypy`. Use `ruff` and `ty`.
- Do NOT swallow errors silently. Propagate with context.
- Do NOT commit without running the verification loop.
```

**Why this works:**

| Approach | What Happens |
|----------|--------------|
| "Prefer httpx" | Agent might still use requests |
| "Do NOT use requests" | Agent avoids requests completely |

Negative instructions are stronger than positive suggestions.

---

## The NEVER Section (Security Guardrails)

Beyond coding style, you need guardrails against **destructive actions**. This is critical for production safety.

```markdown
## NEVER (Destructive Actions)

These actions require **explicit user confirmation**. Stop and ask before proceeding.

### Data & Infrastructure
- NEVER run `DROP`, `DELETE`, `TRUNCATE` on production databases
- NEVER run migrations on production without explicit approval
- NEVER delete cloud resources (VMs, databases, storage buckets)
- NEVER run `terraform destroy`, `pulumi destroy`, or equivalent

### Secrets & Security
- NEVER commit secrets, API keys, tokens, or credentials to git
- NEVER log, print, or expose secrets — even in debug mode
- NEVER disable SSL/TLS verification (`verify=False`, `--insecure`)

### Git & Deployment
- NEVER force push to `main` or `master`
- NEVER deploy to production without explicit approval
- NEVER skip CI checks (`--no-verify`)

### If in Doubt
1. **Stop** — do not execute
2. **Explain** — describe what you were about to do
3. **Ask** — get explicit user confirmation
```

This section is **more important than coding style**. Using `requests` instead of `httpx` is annoying. Dropping your production database is catastrophic.

**Key patterns:**

| Category | Examples |
|----------|----------|
| Data destruction | `DROP TABLE`, `DELETE FROM`, `rm -rf` |
| Secret exposure | Committing `.env`, logging API keys |
| Production changes | Deploys, migrations, infrastructure |
| Irreversible git | Force push, history rewrite |

The "Stop → Explain → Ask" pattern gives you a chance to intervene.

---

## Quick Reference Table

Put this near the top. Developers (and agents) scan for commands:

```markdown
## Quick Reference

| Lang | Format | Lint | Type Check | Test |
|------|--------|------|------------|------|
| Python | `ruff format .` | `ruff check --fix .` | `ty check .` | `pytest` |
| Rust | `cargo fmt` | `cargo clippy` | (built-in) | `cargo test` |
| TS (Bun) | `biome format --write .` | `biome check --fix .` | `tsc --noEmit` | `bun test` |
```

No prose. Just commands.

---

## Language Sections

For each language you use, include:

1. **Tools** — What you use for package management, formatting, linting, testing
2. **Before commit** — The exact command to run
3. **Style** — A short code example showing your preferred patterns
4. **Rules** — 3-5 bullet points of language-specific conventions

```markdown
## Python

**Tools:** `uv` (packages), `ruff` (format + lint), `ty` (types), `pytest` (test)

**Before commit:**
```bash
uv run ruff format . && uv run ruff check --fix . && uv run ty check . && uv run pytest
```

**Style:**
```python
async def fetch_users(user_ids: list[int]) -> list[User]:
    async with httpx.AsyncClient() as client:
        tasks = [client.get(f"/users/{id}") for id in user_ids]
        responses = await asyncio.gather(*tasks)
        return [User(**r.json()) for r in responses]
```

**Rules:**
- Type annotations on all functions
- `httpx` for HTTP, `pydantic` for validation
- Async for I/O. No blocking calls in async code.
```

---

## Git Conventions

Commit format and PR rules:

```markdown
## Git

**Commit format:**
```
type: short description
```

| Type | Use |
|------|-----|
| `feat:` | New feature |
| `fix:` | Bug fix |
| `refactor:` | Restructure (no behavior change) |
| `docs:` | Documentation |
| `test:` | Tests |
| `chore:` | Maintenance |

**PR rule:** One logical change per PR. Explain the *why*.
```

---

## The Loop

End with the verification mantra:

```markdown
## The Loop

```
Change → Format → Lint → Type Check → Test → Commit
```

Run the full verification before every commit. No exceptions.
```

This is the anchor. Everything else supports this rhythm.

---

## Length Matters

!!! warning "Keep it under 200 lines"
    Claude's team found that agent adherence drops past 200 lines. Longer files get skimmed. Critical rules get missed.

If your AGENTS.md is growing past 200 lines:

1. Move project-specific info to SKILL.md
2. Cut redundant explanations
3. Rely on examples over prose

---

## Full Example

Here's a condensed AGENTS.md:

```markdown
# Development Conventions

You are an expert software engineer. You write clean, maintainable code.

> For project-specific knowledge, see [SKILL.md](./SKILL.md).

## Philosophy

- **Simplicity is king**
- **Self-documenting code**
- **Functional over OOP**
- **Verify before commit**

## Do NOT

- Do NOT use `any` in TypeScript
- Do NOT use `requests`. Use `httpx`.
- Do NOT use `black`. Use `ruff`.
- Do NOT commit without running the verification loop.

## Quick Reference

| Lang | Format | Lint | Test |
|------|--------|------|------|
| Python | `ruff format .` | `ruff check --fix .` | `pytest` |
| Rust | `cargo fmt` | `cargo clippy` | `cargo test` |

## Python

**Before commit:**
uv run ruff format . && uv run ruff check --fix . && uv run pytest

## The Loop

Change → Format → Lint → Type Check → Test → Commit
```

That's 30 lines. Effective. Scannable. Complete.

---

## Next Steps

- Copy the [full AGENTS.md](https://github.com/Proteusiq/agentic/blob/main/AGENTS.md) from the repo
- Customize for your stack
- Symlink to your projects
- Never repeat yourself again
