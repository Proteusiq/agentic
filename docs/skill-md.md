# Writing SKILL.md

Your project's DNA. Everything an agent needs to understand *this* codebase.

---

## The Goal

Capture knowledge that would take hours to rediscover:

- Architecture decisions
- Domain rules that aren't obvious from code
- The weird parts where dragons live
- Environment and deployment quirks

An agent reading your SKILL.md should understand your project in 60 seconds.

---

## Structure

A good SKILL.md has these sections:

| Section | Purpose |
|---------|---------|
| **Frontmatter** | Name + description for agent discovery |
| **Project Overview** | One paragraph on what this is |
| **Architecture** | How the pieces connect |
| **Key Files** | Where to look for what |
| **Domain Rules** | Business logic not obvious from code |
| **The Weird Parts** | Known quirks, legacy code, footguns |
| **Environment** | Config, secrets, deployment |

---

## Frontmatter

Start with YAML frontmatter. This helps agents discover and identify skills:

```yaml
---
name: payment-api
description: Stripe integration for subscription billing
---
```

Some tools (like OpenCode) use this to list available skills. Keep it short.

---

## Project Overview

One paragraph. What is this thing?

```markdown
## Project Overview

Payment API handles subscription billing via Stripe. It processes webhooks,
manages customer lifecycle, and syncs subscription state to our database.

Repository: `acme/payment-api`
Production: `payments.acme.com`
```

Don't oversell. Just state the facts.

---

## Architecture

Show how the pieces connect. ASCII diagrams work great:

```markdown
## Architecture

```
settings.py          (config, env vars)
    │
services/
├── stripe.py        (Stripe API wrapper)
├── billing.py       (business logic)
└── sync.py          (DB ↔ Stripe sync)
    │
db/
├── models.py        (SQLAlchemy models)
└── queries.py       (database access)
    │
api/
└── routes.py        (FastAPI endpoints)
```

Or use a simple flow:

```markdown
## Data Flow

Webhook → validate_signature → parse_event → update_database → notify_services
```

Keep it high-level. Details go in code.

---

## Key Files

A table of "where to find what":

```markdown
## Key Files

| File | Purpose | Key Functions |
|------|---------|---------------|
| `services/billing.py` | Subscription logic | `create_subscription()`, `cancel()` |
| `services/stripe.py` | Stripe API wrapper | `StripeClient` class |
| `api/routes.py` | HTTP endpoints | `POST /webhook`, `GET /subscription` |
| `db/models.py` | Database schema | `Customer`, `Subscription` |
```

This saves agents from grepping blindly.

---

## Domain Rules

The business logic that *isn't* obvious from reading code:

```markdown
## Domain Rules

### Pricing
- Prices stored in **cents**, displayed in **dollars**
- Never round during calculation, only on display

### Subscriptions
- Never delete subscriptions — mark as `cancelled`
- Grace period is 7 days after payment failure
- Downgrades take effect at period end

### Webhooks
- Signature verification is **mandatory**
- Process webhooks idempotently (same event may arrive twice)
```

These are the rules that would take a new developer hours to learn from code.

---

## The Weird Parts

Every codebase has dragons. Document them:

```markdown
## The Weird Parts

### `legacy_pricing.py`
Uses the old Stripe API (pre-2020). Don't touch it. It handles grandfathered
customers on the legacy plan. Will be removed when last legacy customer churns.

### `sync.py` timing
Runs on a 5-minute cron, but Stripe webhooks arrive in ~1 second. There's a
race condition if you rely on sync state immediately after a webhook.

### Test mode
Tests hit Stripe test mode. You need `STRIPE_TEST_KEY` in `.env`. Requests
are rate-limited to 25/sec even in test mode.
```

This section saves hours of debugging.

---

## Environment

What's needed to run this thing:

```markdown
## Environment Variables

| Variable | Description |
|----------|-------------|
| `STRIPE_SECRET_KEY` | Stripe API key |
| `STRIPE_WEBHOOK_SECRET` | Webhook signature secret |
| `DATABASE_URL` | PostgreSQL connection string |
| `REDIS_URL` | Cache connection string |

## Local Development

```bash
cp .env.example .env
# Add your Stripe test keys
docker-compose up -d
uv run python -m api
```

## Deployment

Push to `main` → deploys to staging
Tag `v*` → deploys to production
```

---

## Length Guidelines

SKILL.md can be longer than AGENTS.md — it contains dense project knowledge. But still:

| Length | Status |
|--------|--------|
| < 100 lines | Probably missing important context |
| 100-300 lines | Sweet spot |
| > 300 lines | Consider splitting or trimming |

If it's getting long, ask: "Would an agent need this, or is it just documentation?"

---

## Full Example

```yaml
---
name: payment-api
description: Stripe integration for subscription billing
---
```

```markdown
# payment-api

Stripe-based subscription billing for Acme Corp.

Repository: `acme/payment-api`
Production: `payments.acme.com`

## Architecture

```
settings.py → services/ → db/ → api/
```

## Key Files

| File | Purpose |
|------|---------|
| `services/billing.py` | Subscription logic |
| `api/routes.py` | HTTP endpoints |

## Domain Rules

- Prices in cents, displayed in dollars
- Never delete subscriptions, cancel them
- Webhooks must be signature-verified

## The Weird Parts

- `legacy_pricing.py` — old Stripe API, don't touch
- Tests require `STRIPE_TEST_KEY` in env

## Environment

| Variable | Description |
|----------|-------------|
| `STRIPE_SECRET_KEY` | API key |
| `DATABASE_URL` | PostgreSQL |
```

50 lines. Everything an agent needs.

---

## Next Steps

- Start with the [SKILL.template.md](https://github.com/Proteusiq/agentic/blob/main/SKILL.template.md)
- Fill in your project's specifics
- Add domain rules as you discover them
- Keep it updated — this is living documentation
