# Examples

What `learnings.md` looks like in practice.

---

## Python Project

A real `learnings.md` after working on a Python project:

```markdown
# Learnings

LLM knowledge base. Document findings here: gotchas, working commands, patterns, API quirks.
Persists across sessions. Summarize when knowledge becomes outdated or redundant.

---

## Task Approach

- **Enter plan mode for non-trivial tasks** — don't jump straight into code
- **No shortcuts. No laziness. No surface solutions.**
- Always ask: *how might this change affect the whole?*
- **Don't reinvent the wheel** — search for existing solutions first
- Reread documentation, check GitHub issues (open *and* closed), study original code

---

## Code Design

- Prefer **pure functions** where feasible; isolate side effects
- Organize code so changes are easy and predictable
- Avoid hidden state and mutable globals
- **No magic numbers** — extract literals into named constants

---

## Types & Data

- Declare types explicitly at *module boundaries*
- Use language-specific type features to model domain constraints

---

## Error Handling

- Treat errors as structured data, not control flow
- Add contextual information when propagating errors
- Never swallow errors silently

---

## Testing

- **Unit tests** for pure logic, **integration tests** for I/O boundaries
- Assert behavior, not implementation details
- Aim for reproducibility and determinism
- Follow **AAA pattern**: Arrange, Act, Assert

---

## Git & Collaboration

- Feature-branch workflow: `feat/…`, `fix/…`, `refactor/…`
- Rebase or squash commits to maintain clean history
- PRs with reviews, tests, and clear descriptions

---

## Project Structure

Functional Core, Imperative Shell:

```
┌─────────────────────────────────┐
│         Interfaces              │  ← CLI, API, UI (thin)
├─────────────────────────────────┤
│         Side Effects            │  ← I/O, database, network
├─────────────────────────────────┤
│         Core Logic              │  ← Pure functions (testable)
└─────────────────────────────────┘
```

Layout:
```
src/
  core/       # Pure business logic (no I/O)
  services/   # Side effects (DB, HTTP, filesystem)
  api/        # HTTP handlers
  cli/        # CLI commands
tests/
  unit/       # Tests for core/
  integration/ # Tests for services/
```

Why this works:
- Core logic is **pure** → easy to test, no mocks needed
- Side effects are **isolated** → easy to swap implementations
- Interfaces are **thin** → easy to add new entry points

---

## Python Tools

| Tool | Purpose | Install |
|------|---------|---------|
| `uv` | Package/project manager | `brew install uv` |
| `ruff` | Linter & formatter | `uv tool install ruff` |
| `ty` | Type checker (10-100x faster than mypy) | `uv tool install ty` |
| `pytest` | Testing | `uv add --dev pytest` |

---

## Python Workflow

```bash
uv init myproject && cd myproject
uv add httpx
uv add --dev pytest

uv run python script.py
uv run pytest

# One-off (no install)
uvx ruff check .
uvx ty check .
```

Before commit:
```bash
uv run ruff format .
uv run ruff check --fix .
uv run ty check .
uv run pytest
```

---

## Python Style

```python
async def fetch_users(user_ids: list[int]) -> list[User]:
    """Fetch users by their IDs."""
    async with httpx.AsyncClient() as client:
        tasks = [client.get(f"/users/{id}") for id in user_ids]
        responses = await asyncio.gather(*tasks)
        return [User(**r.json()) for r in responses]
```

- Type annotations: always, Python 3.12+ (`list[T]`, `X | None`)
- Docstrings: brief, public APIs only
- Async for I/O

---

## Python Checklist

- [ ] **Tools**: Using `uv`, `ruff`, `ty`, `pytest`
- [ ] **Types**: All functions have type annotations
- [ ] **Async**: Using `asyncio` for I/O (not blocking `requests`)
- [ ] **Data**: Using `dataclass` or `pydantic` for structured data
- [ ] **HTTP**: Using `httpx` (async) instead of `requests` (sync)
- [ ] **Validation**: Using `pydantic` for external input
- [ ] **Errors**: Returning structured errors, not exceptions for control flow
- [ ] **Imports**: Absolute imports, no circular dependencies
- [ ] **Config**: Environment variables, not hardcoded values
```

---

## Key Takeaways

This `learnings.md` contains:

1. **Principles** — Task approach, code design, error handling
2. **Project-specific patterns** — Structure, layout, architecture
3. **Tool knowledge** — Commands that work, workflows
4. **Checklists** — Quick verification before commit

The LLM builds this over time. Each session adds discoveries. Knowledge persists.
