---
name: {{PROJECT_NAME}}
description: {{SHORT_DESCRIPTION}}
---

# {{PROJECT_NAME}} Codebase Knowledge

## Project Overview

{{LONGER_DESCRIPTION}}

Repository: `{{GITHUB_ORG}}/{{REPO_NAME}}`
Production: {{PRODUCTION_URL_OR_DESCRIPTION}}

## Layer Architecture

```
settings.py / config.rs     (leaf: config, env vars, constants)
types.py / models.rs        (leaf: domain models, data types)
    |
services/                   (business logic, external API clients)
    |
db/ / storage/              (database, cache, persistence)
    |
api/ / mcp/                 (HTTP handlers, MCP tools)
    |
cli/                        (CLI entry points)
```

Dependencies flow downward. Lower layers never import higher layers.

## Source Layout

```
src/{{PROJECT_NAME}}/
  __init__.py           # Public API exports
  settings.py           # Configuration (pydantic-settings or similar)
  types.py              # Domain models
  services/
    __init__.py         # Service exports
    {{SERVICE_NAME}}.py # Core business logic
  db/
    engine.py           # Database connection
    loader.py           # ETL or data loading
  api/
    main.py             # FastAPI/Flask app
    routers/
      {{RESOURCE}}.py   # Resource endpoints
  cli/
    main.py             # Typer/Click CLI

tests/
  conftest.py           # Fixtures
  unit/                 # Unit tests (pure logic)
  integration/          # Integration tests (I/O)
```

## Key Files

| File | Purpose | Key Functions |
|------|---------|---------------|
| `services/{{SERVICE_NAME}}.py` | Core business logic | `{{MAIN_FUNCTION}}()` |
| `api/routers/{{RESOURCE}}.py` | REST endpoints | `get_{{RESOURCE}}`, `create_{{RESOURCE}}` |
| `settings.py` | Configuration | `Settings` class |
| `types.py` | Data models | `{{MODEL_NAME}}` |

## Domain Rules

### {{DOMAIN_AREA_1}}

{{DESCRIBE_BUSINESS_RULES}}

### {{DOMAIN_AREA_2}}

{{DESCRIBE_MORE_RULES}}

## API Endpoints

| Method | Path | Description |
|--------|------|-------------|
| GET | /{{RESOURCE}} | List {{RESOURCE}}s |
| GET | /{{RESOURCE}}/{id} | Get single {{RESOURCE}} |
| POST | /{{RESOURCE}} | Create {{RESOURCE}} |
| PUT | /{{RESOURCE}}/{id} | Update {{RESOURCE}} |
| DELETE | /{{RESOURCE}}/{id} | Delete {{RESOURCE}} |

## Environment Variables

| Variable | Description |
|----------|-------------|
| `{{PROJECT_NAME_UPPER}}_SECRET_KEY` | JWT/encryption key |
| `{{PROJECT_NAME_UPPER}}_DATABASE_URL` | Database connection string |
| `{{EXTERNAL_API}}_API_KEY` | External service API key |

## Testing

### Unit Tests (fast, no I/O)

```bash
uv run pytest tests/unit/
```

### Integration Tests (slow, requires services)

```bash
uv run pytest tests/integration/
```

### Verification Loop (before commit)

```bash
uvx ruff format .
uvx ruff check --fix .
uvx ty check .
uv run pytest
```

## Deployment

```bash
# Build
docker build -t {{PROJECT_NAME}} .

# Deploy (example)
gh workflow run "Deploy" --field environment=dev
```

Environments:
- `dev`: Development testing
- `tst`: Pre-production
- `prd`: Production

## Common Issues

### {{ISSUE_1_TITLE}}

{{DESCRIBE_ISSUE_AND_SOLUTION}}

### {{ISSUE_2_TITLE}}

{{DESCRIBE_ISSUE_AND_SOLUTION}}

## Related Issues

- Issue #{{N}}: {{DESCRIPTION}} (status)
