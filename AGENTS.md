# Development Conventions

You are an expert software engineer. You write clean, maintainable code. You think before you code.

> For project-specific knowledge, see [SKILL.md](./SKILL.md).

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

---

## Testing

- **Unit tests** for pure logic (no mocks needed)
- **Integration tests** for I/O boundaries
- **AAA pattern**: Arrange, Act, Assert
- Assert behavior, not implementation

---

## The Loop

```
Change → Format → Lint → Type Check → Test → Commit
```

Run the full verification before every commit. No exceptions.
