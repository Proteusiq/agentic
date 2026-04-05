# Workflow

How agentic development actually works.

---

## The Loop

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                              RESEARCH PHASE                                 │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│   ┌─────────────┐    ┌─────────────┐    ┌─────────────┐                     │
│   │  Language   │    │  Project    │    │  Library/   │                     │
│   │  Best       │───▶│  Structure  │───▶│  Tool Docs  │                     │
│   │  Practices  │    │  Patterns   │    │             │                     │
│   └─────────────┘    └─────────────┘    └─────────────┘                     │
│                                                                             │
│   Examples:                                                                 │
│   • Python → packaging.python.org, PEPs                                     │
│   • FastAPI → fastapi.tiangolo.com                                          │
│   • FastMCP → github.com/jlowin/fastmcp                                     │
│                                                                             │
└──────────────────────────────────┬──────────────────────────────────────────┘
                                   │
                                   ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│                              DISCOVERY PHASE                                │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│   ┌─────────────┐    ┌─────────────┐    ┌─────────────┐                     │
│   │  Explore    │    │  Query      │    │  Test       │                     │
│   │  Codebase   │───▶│  DB/APIs    │───▶│  Endpoints  │                     │
│   │             │    │             │    │             │                     │
│   └─────────────┘    └─────────────┘    └─────────────┘                     │
│                                                                             │
│   What does THIS project need?                                              │
│   • CRUD operations                                                         │
│   • Services / business logic                                               │
│   • External clients                                                        │
│   • Database schemas                                                        │
│                                                                             │
└──────────────────────────────────┬──────────────────────────────────────────┘
                                   │
                                   ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│                              UPDATE LEARNINGS                               │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│   learnings.md ← Document what was discovered                               │
│                                                                             │
│   • Working commands                                                        │
│   • Gotchas and edge cases                                                  │
│   • Project-specific patterns                                               │
│   • API quirks                                                              │
│                                                                             │
└──────────────────────────────────┬──────────────────────────────────────────┘
                                   │
                                   ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│                              PLAN WORK                                      │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│   todo.md ← Break down into tasks                                           │
│                                                                             │
│   • Specific, actionable items                                              │
│   • One task in progress at a time                                          │
│   • Mark complete immediately                                               │
│   • Summarize/rewrite when done                                             │
│                                                                             │
└──────────────────────────────────┬──────────────────────────────────────────┘
                                   │
                                   ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│                              BUILD                                          │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│   Write code → Update todo.md → Repeat                                      │
│                                                                             │
└──────────────────────────────────┬──────────────────────────────────────────┘
                                   │
                                   ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│                              VERIFICATION LOOP                              │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│   ┌──────────┐   ┌──────────┐   ┌──────────┐   ┌──────────┐                 │
│   │  Format  │──▶│   Lint   │──▶│  Type    │──▶│   Test   │                 │
│   │          │   │          │   │  Check   │   │          │                 │
│   └──────────┘   └──────────┘   └──────────┘   └──────────┘                 │
│                                                                             │
│   Before release:                                                           │
│   ┌──────────────────────────────────────────────────────────┐              │
│   │  SECURITY SCAN                                           │              │
│   │  • Find vulnerabilities (red team mindset)               │              │
│   │  • Patch what you find                                   │              │
│   │  • Repeat until secure                                   │              │
│   │  • Check: injection, auth, secrets, deps, inputs         │              │
│   └──────────────────────────────────────────────────────────┘              │
│                                                                             │
└──────────────────────────────────┬──────────────────────────────────────────┘
                                   │
                                   ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│                              COMMIT                                         │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│   Only when verification passes.                                            │
│                                                                             │
│   learnings.md persists for next session.                                   │
│   todo.md gets summarized/cleared.                                          │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## Example: FastAPI Project

**Research Phase:**
```
1. Read fastapi.tiangolo.com (routing, dependencies, Pydantic)
2. Read packaging.python.org (pyproject.toml, src layout)
3. Check project's existing patterns
```

**Discovery Phase:**
```
1. Explore existing endpoints in src/app/
2. Query database to understand schema
3. Test existing endpoints to see response shapes
```

**Update learnings.md:**
```markdown
## FastAPI Patterns

- Use `Annotated[Dep, Depends(...)]` for dependencies
- Pydantic models in `src/app/schemas/`
- SQLAlchemy models in `src/app/models/`
- Alembic for migrations: `alembic revision --autogenerate -m "msg"`
```

**Plan in todo.md:**
```markdown
- [ ] Create Pydantic schema for User
- [ ] Add SQLAlchemy model
- [ ] Generate migration
- [ ] Implement CRUD endpoints
- [ ] Write tests
```

**Verification:**
```bash
ruff format . && ruff check --fix . && ty check . && pytest
```

---

## Example: FastMCP Server

**Research Phase:**
```
1. Read github.com/jlowin/fastmcp
2. Understand tool/resource/prompt patterns
3. Check MCP spec for protocol details
```

**Discovery Phase:**
```
1. What capabilities does this server need?
2. What external APIs will it call?
3. What data does it need to expose?
```

**Update learnings.md:**
```markdown
## FastMCP Patterns

- Tools: `@mcp.tool()` decorator
- Resources: `@mcp.resource("uri://pattern")` 
- Context: `ctx: Context` for logging, progress
- Run with: `mcp run server.py` or `mcp dev server.py`
```

---

## Example: GitHub Actions

**Research Phase:**
```
1. Read docs.github.com/en/actions
2. Check existing workflows in .github/workflows/
3. Understand runner environments
```

**Discovery Phase:**
```
1. What triggers needed? (push, PR, schedule)
2. What secrets are available?
3. What artifacts need to be produced?
```

**Update learnings.md:**
```markdown
## GitHub Actions

- Use `actions/checkout@v4` for repo access
- Cache dependencies: `actions/cache@v4`
- Matrix builds: `strategy.matrix`
- Secrets: `${{ secrets.NAME }}`
```

---

## Knowledge Reuse

`learnings.md` persists across sessions. Reuse depends on knowledge freshness:

| Knowledge Type | Shelf Life | Example |
|----------------|------------|---------|
| Language fundamentals | Years | Python packaging, type hints |
| Framework patterns | Months | FastAPI routing, SQLAlchemy ORM |
| Library APIs | Weeks/Months | Specific library versions |
| Project-specific | Until changed | Your database schema, your patterns |

When in doubt, re-research. Stale knowledge hurts more than no knowledge.
