# Development Conventions

---

## How to Work

1. **Research first** — read official docs, explore the codebase, understand before changing
2. **Document findings** — update `learnings.md` with what you discover (gotchas, working commands, patterns)
3. **Track future work** — push non-blocking items to `todo.md`

That's it. No external skills, no pre-loaded context. Research in real-time, document as you go.

---

## Philosophy

- **Simplicity is king** — the simplest solution that works is the best solution
- **Self-documenting code** — if it needs comments, refactor it
- **Functional over OOP** — pure functions, composition, immutability
- **Verify before commit** — if it's not tested, it's not done

---

## Do NOT

- Do NOT jump straight into code. Plan first.
- Do NOT use `any` in TypeScript. Ever.
- Do NOT use `Optional[X]` in Python. Use `X | None`.
- Do NOT use `requests`. Use `httpx`.
- Do NOT use `black` or `mypy`. Use `ruff` and `ty`.
- Do NOT use `eslint` + `prettier`. Use `biome`.
- Do NOT swallow errors silently. Propagate with context.
- Do NOT write inline comments. If code needs explaining, refactor.
- Do NOT use mutable default arguments in Python.
- Do NOT use `.unwrap()` in production Rust. Handle the error.
- Do NOT commit without running the verification loop.

---

## NEVER (Destructive Actions)

These actions require **explicit user confirmation**. Stop and ask before proceeding.

- NEVER run destructive database commands (`DROP`, `DELETE`, `TRUNCATE`) on production
- NEVER commit secrets, API keys, tokens, or credentials to git
- NEVER force push to `main` or `master`
- NEVER run `rm -rf` on directories you didn't create in this session
- NEVER deploy to production without explicit approval

If an action could cause data loss, expose secrets, or affect production: **stop, explain, ask**.

---

## Python Structure

Reference: [packaging.python.org](https://packaging.python.org/en/latest/tutorials/packaging-projects/)

Praysonic Style: [Proteusiq/beacon](https://github.com/Proteusiq/beacon)

---

## Logging

- Use `logging.getLogger(__name__)` — one logger per module
- Levels: `DEBUG` (diagnostics), `INFO` (working), `WARNING` (unexpected), `ERROR` (failure)
- Log liberally in long functions, try/except blocks, and external calls
- Never log secrets, tokens, or PII

---

## Quick Reference

| Lang | Format | Lint | Type Check | Test |
|------|--------|------|------------|------|
| Python | `ruff format .` | `ruff check --fix .` | `ty check .` | `pytest` |
| Rust | `cargo fmt` | `cargo clippy` | (built-in) | `cargo test` |
| TS (Bun) | `biome format --write .` | `biome check --fix .` | `tsc --noEmit` | `bun test` |
| TS (Deno) | `deno fmt` | `deno lint` | `deno check .` | `deno test` |

---

## Git

**Commit format:** `type: short description`

Types: `feat`, `fix`, `refactor`, `docs`, `test`, `chore`

**Branch naming:** `feat/…`, `fix/…`, `refactor/…`

---

## The Loop

```
Change → AGENTS.md rules followed → Format → Lint → Type Check → Test → Commit
```

Run the full verification before every commit. No exceptions.
