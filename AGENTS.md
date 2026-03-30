# Development Conventions

You are an expert software engineer. You write clean, maintainable code. You think before you code.

> For project-specific knowledge, see [SKILL.md](./SKILL.md).

---

## Architecture Overview

> **Fill this in per-project.** Describe how the system works in 2-3 sentences.
>
> Example: "Requests hit the API layer, which validates input and dispatches to service modules. Services contain business logic and call repositories for persistence. Background jobs run via the task queue."

---

## Project Structure

> **Fill this in per-project.** Map directories to their responsibilities.
>
> ```
> src/<package>/
> ├── cli/        # CLI commands, argument parsing
> ├── crud/       # Data access, database operations
> ├── <domain>/   # Core domain logic (name it contextually)
> ├── __init__.py # Public API exports
> ├── client.py   # External service clients
> ├── settings.py # Configuration, environment variables
> └── types.py    # Domain types, Pydantic models
> tests/          # Mirrors src/<package>/ structure
> scripts/        # CI, deployment, one-off automation
> docs/           # mkdocs documentation
> ```

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

### Data & Infrastructure
- NEVER run `DROP`, `DELETE`, `TRUNCATE` on production databases
- NEVER run migrations on production without explicit approval
- NEVER delete cloud resources (VMs, databases, storage buckets)
- NEVER modify infrastructure-as-code for production environments
- NEVER run `terraform destroy`, `pulumi destroy`, or equivalent

### Secrets & Security
- NEVER commit secrets, API keys, tokens, or credentials to git
- NEVER log, print, or expose secrets — even in debug mode
- NEVER disable SSL/TLS verification (`verify=False`, `--insecure`)
- NEVER store secrets in plain text files
- NEVER add secrets to environment variables in CI logs

### Git & Deployment
- NEVER force push to `main` or `master` (`git push --force`)
- NEVER delete remote branches without asking
- NEVER deploy to production without explicit approval
- NEVER skip CI checks or pre-commit hooks (`--no-verify`)
- NEVER rewrite public git history (`rebase`, `reset --hard` on shared branches)

### File System
- NEVER run `rm -rf` on directories you didn't create in this session
- NEVER modify system files (`/etc`, `/usr`, `~/.ssh`, `~/.gnupg`)
- NEVER overwrite files without reading them first

### If in Doubt
If an action could cause data loss, expose secrets, or affect production:

1. **Stop** — do not execute
2. **Explain** — describe what you were about to do
3. **Ask** — get explicit user confirmation

---

## Before You Code

1. **Understand the task** — restate it in your own words
2. **Search for existing solutions** — check docs, issues, original code
3. **Plan the approach** — what changes, what could break?
4. **Visualize changes** — ASCII diagrams for behavior/architecture changes
5. **Then code** — small, verified steps

---

## Code Design

```
┌─────────────────────────────────┐
│         Interfaces              │  ← CLI, API, UI (thin)
├─────────────────────────────────┤
│         Side Effects            │  ← I/O, database, network
├─────────────────────────────────┤
│         Core Logic              │  ← Pure functions (testable)
└─────────────────────────────────┘
```

- Pure functions where feasible. Isolate side effects.
- Types at module boundaries. Validate external input.
- Errors as data, not control flow. Add context when propagating.
- No magic numbers. Extract literals into named constants.

---

## Logging

- Use `logging.getLogger(__name__)` — one logger per module.
- Log at appropriate levels:
  - `DEBUG` — detailed diagnostic info (disabled in production)
  - `INFO` — confirmation that things are working as expected
  - `WARNING` — something unexpected, but the system continues
  - `ERROR` — a failure that prevented an operation from completing
- Log liberally in:
  - Long functions with multiple steps
  - `try`/`except` blocks that catch broad exceptions
  - External service calls (before and after)
- Never log secrets, tokens, passwords, or PII.

---

## Quick Reference

| Lang | Format | Lint | Type Check | Test |
|------|--------|------|------------|------|
| Python | `ruff format .` | `ruff check --fix .` | `ty check .` | `pytest` |
| Rust | `cargo fmt` | `cargo clippy` | (built-in) | `cargo test` |
| TS (Bun) | `biome format --write .` | `biome check --fix .` | `tsc --noEmit` | `bun test` |
| TS (Deno) | `deno fmt` | `deno lint` | `deno check .` | `deno test` |
| Bash | `shfmt -w` | `shellcheck` | `bash -n` | — |

---

## Python

**Tools:** `uv` (packages), `ruff` (format + lint), `ty` (types), `pytest` (test)

**Before commit:**
```bash
uv run ruff format . && uv run ruff check --fix . && uv run ty check . && uv run pytest
```

**Style:**
```python
async def fetch_users(user_ids: list[int]) -> list[User]:
    """Fetch users by their IDs."""
    async with httpx.AsyncClient() as client:
        tasks = [client.get(f"/users/{id}") for id in user_ids]
        responses = await asyncio.gather(*tasks)
        return [User(**r.json()) for r in responses]
```

**Rules:**
- Type annotations on all functions (Python 3.12+: `list[T]`, `X | None`)
- `httpx` for HTTP, `pydantic` for validation, `dataclass` for data
- Async for I/O. No blocking calls in async code.
- Absolute imports. No circular dependencies.

---

## Rust

**Tools:** `cargo` (build/test), `bacon` (watch), `clippy` (lint)

**Before commit:**
```bash
cargo fmt && cargo clippy -- -D warnings && cargo test
```

**Style:**
```rust
pub async fn fetch_user(user_id: u64) -> Result<User> {
    let url = format!("https://api.example.com/users/{user_id}");
    let user = reqwest::get(&url).await?.json().await?;
    Ok(user)
}
```

**Rules:**
- `Result<T, E>` with `?` for error handling
- `anyhow` for applications, `thiserror` for libraries
- No `.unwrap()` in production. Use `.expect("reason")` or handle properly.

---

## TypeScript

**Tools:** `bun` or `deno` (runtime), `biome` (format + lint), `zod` (validation)

**Before commit (Bun):**
```bash
bun biome format --write . && bun biome check --fix . && bun tsc --noEmit && bun test
```

**Before commit (Deno):**
```bash
deno fmt && deno lint && deno check . && deno test
```

**Style:**
```typescript
const UserSchema = z.object({ id: z.number(), name: z.string() });
type User = z.infer<typeof UserSchema>;

export async function fetchUser(userId: number): Promise<User> {
  const response = await fetch(`/users/${userId}`);
  return UserSchema.parse(await response.json());
}
```

**Rules:**
- Always TypeScript. Never `any`.
- `zod` for runtime validation
- `async`/`await` over raw promises

---

## Bash

**Before commit:**
```bash
shellcheck script.sh && bash -n script.sh
```

**Style:**
```bash
#!/bin/bash
set -euo pipefail

main() {
    local name="${1:-World}"
    echo "Hello, $name!"
}

main "$@"
```

**Rules:**
- Always `set -euo pipefail`
- Quote all variables: `"$var"`
- Use `[[ ]]` not `[ ]`

---

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

**Branch naming:** `feat/…`, `fix/…`, `refactor/…`

**PR rule:** One logical change per PR. Explain the *why*.

**PR descriptions should include:**
- Summary of intent (what and why)
- Verification commands run locally
- Dependency changes (if any)
- Breaking changes or migration notes

---

## Testing

- **Unit tests** for pure logic (no mocks needed)
- **Integration tests** for I/O boundaries
- **AAA pattern**: Arrange, Act, Assert
- Assert behavior, not implementation

**Organization:**
- Mirror `src/` structure under `tests/` — match filenames for traceability
- Use pytest markers for optional test suites (e.g., `@pytest.mark.slow`, `@pytest.mark.integration`)
- Skip expensive tests locally with `-m "not slow"`, run full suite in CI
- Parametrize test cases where inputs vary but logic is the same
- Favor real implementations over mocks; mock only at true boundaries

---

## Common Issues

> **Fill this in per-project.** Document known gotchas and their fixes.
>
> Example:
> - **`uv run` permission denied under `~/.cache`** — Override cache locations: `UV_CACHE="$(pwd)/.cache_uv" XDG_CACHE_HOME="$(pwd)/.cache_xdg" uv run <command>`
> - **Import errors after pulling** — Run `uv sync` to install new dependencies
> - **Type errors in CI but not locally** — Ensure you're running the same Python version as CI

---

## The Loop

```
Change → Format → Lint → Type Check → Test → Commit
```

Run the full verification before every commit. No exceptions.
