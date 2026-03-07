# Examples

Real-world SKILL.md files from production projects.

---

## API Backend

A FastAPI service with PostgreSQL and background workers.

### Frontmatter

```yaml
---
name: order-api
description: Order management API for e-commerce platform
---
```

### Overview

REST API for order lifecycle management. Handles cart → checkout → payment → fulfillment.

- Repository: `acme/order-api`
- Production: `api.acme.com/orders`

### Architecture

```
config.py               ← settings, env vars
services/
├── orders.py           ← order lifecycle
├── payments.py         ← Stripe integration
├── inventory.py        ← stock checks
└── notifications.py    ← email/SMS
db/
├── models.py           ← SQLAlchemy
└── queries.py          ← raw SQL for reports
api/
├── app.py              ← FastAPI
└── routes/
    ├── orders.py
    ├── cart.py
    └── webhooks.py
workers/
└── fulfillment.py      ← Celery tasks
```

### Key Files

| File | Purpose |
|------|---------|
| `services/orders.py` | `create_order()`, `cancel_order()`, `refund()` |
| `services/payments.py` | Stripe charge, refund, webhook handling |
| `workers/fulfillment.py` | Async order processing |

### Domain Rules

**Pricing:**

- All amounts in **cents** (integer)
- Tax calculated at checkout, not stored on line items
- Discounts apply before tax

**Order States:**

```
cart → pending → paid → processing → shipped → delivered
                    ↓
               cancelled → refunded
```

- Orders can only be cancelled before `processing`
- Refunds create negative transactions, never delete records

**Inventory:**

- Stock reserved at checkout, committed at payment
- Reserved stock expires after 15 minutes
- Never go negative — reject order if insufficient

### The Weird Parts

**`legacy_orders.py`** — Pre-2022 orders use a different schema. Read-only. Don't modify.

**Webhook idempotency** — Stripe sends webhooks multiple times. We dedupe by `event_id` in Redis with 24h TTL.

**Test mode** — Use `STRIPE_TEST_KEY`. Test card: `4242424242424242`.

### Environment

| Variable | Description |
|----------|-------------|
| `DATABASE_URL` | PostgreSQL connection |
| `REDIS_URL` | Cache and queue broker |
| `STRIPE_SECRET_KEY` | Stripe API key |
| `STRIPE_WEBHOOK_SECRET` | Webhook signature verification |

---

## CLI Tool

A Rust command-line tool for data processing.

### Frontmatter

```yaml
---
name: datapipe
description: ETL pipeline CLI for transforming CSV/JSON data
---
```

### Overview

CLI for data transformation pipelines. Reads CSV/JSON, applies transforms, outputs to various formats.

- Repository: `tools/datapipe`

### Architecture

```
src/
├── main.rs             ← CLI entry, clap
├── config.rs           ← pipeline configuration
├── readers/
│   ├── csv.rs
│   └── json.rs
├── transforms/
│   ├── filter.rs
│   ├── map.rs
│   └── aggregate.rs
├── writers/
│   ├── csv.rs
│   ├── json.rs
│   └── parquet.rs
└── error.rs            ← thiserror types
```

### Usage

```bash
datapipe input.csv \
  --filter "age > 18" \
  --map "name = upper(name)" \
  --output result.parquet
```

### Key Types

| Type | Location | Purpose |
|------|----------|---------|
| `Pipeline` | `config.rs` | Full transformation spec |
| `Record` | `readers/mod.rs` | Generic row representation |
| `Transform` | `transforms/mod.rs` | Trait for all transforms |

### Domain Rules

**Type Coercion:**

- Strings to numbers: fail loudly, no silent `0`
- Dates: ISO 8601 only (`YYYY-MM-DD`)
- Nulls: explicit `null` or empty string, configurable

**Memory:**

- Streaming by default (record-at-a-time)
- Aggregations buffer to disk if > 1GB
- `--memory-limit` flag for constrained environments

**Output Formats:**

- CSV: RFC 4180 compliant, quoted if needed
- JSON: newline-delimited (NDJSON), not array
- Parquet: Snappy compression, row group size 64MB

### The Weird Parts

**Filter parser** — Hand-rolled recursive descent in `transforms/filter.rs:parse_expression()`. Don't add new operators without tests.

**Windows line endings** — We normalize to `\n` on read. Output always `\n` unless `--crlf`.

---

## ML Service

A Python ML inference service with model versioning.

### Frontmatter

```yaml
---
name: predictor
description: Real-time ML inference API with model versioning
---
```

### Overview

Serves ML models via REST API. Handles model loading, versioning, and A/B testing.

- Repository: `ml/predictor`
- Production: `ml.acme.com/predict`

### Architecture

```
config.py               ← settings
models/
├── registry.py         ← model version management
├── loader.py           ← S3 → memory
└── cache.py            ← LRU model cache
services/
├── predict.py          ← inference orchestration
├── preprocess.py       ← feature engineering
└── postprocess.py      ← output formatting
api/
└── app.py              ← FastAPI
```

### Key Files

| File | Purpose |
|------|---------|
| `models/registry.py` | `get_model(name, version)`, `list_versions()` |
| `services/predict.py` | `predict(model, features)` — main entry |
| `config.py` | Model paths, cache size, timeouts |

### Domain Rules

**Model Versioning:**

- Models stored in S3: `s3://models/{name}/{version}/model.pkl`
- Version format: `v1`, `v2`, ... (not semver)
- `latest` alias points to highest version number
- Never delete old versions (rollback safety)

**A/B Testing:**

- Traffic split configured in `config.py`
- Split by user ID hash (consistent assignment)
- Metrics logged to DataDog with `model_version` tag

**Inference:**

- Timeout: 100ms for prediction, 5s for model load
- Batch size: max 100 items per request
- Features must match training schema exactly (no partial)

### The Weird Parts

**Model pickle security** — We only load pickles from our S3 bucket. Never accept user-uploaded models. The loader validates S3 path prefix before loading.

**GPU memory** — Models loaded to GPU stay resident. LRU eviction when > 80% VRAM. If you see OOM, check `models/cache.py:evict()`.

**Cold start** — First request loads model (1-3s). Use `/warmup` endpoint in k8s readiness probe.

### Environment

| Variable | Description |
|----------|-------------|
| `MODEL_BUCKET` | S3 bucket for models |
| `MODEL_CACHE_SIZE` | Max models in memory (default: 5) |
| `AWS_REGION` | S3 region |
| `CUDA_VISIBLE_DEVICES` | GPU selection |

---

## Writing Your Own

Start with the [SKILL.template.md](https://github.com/Proteusiq/agentic/blob/main/SKILL.template.md) and fill in:

1. **Frontmatter** — name and one-line description
2. **Overview** — what this project does (one paragraph)
3. **Architecture** — how the pieces connect (ASCII diagram)
4. **Key Files** — where to find what
5. **Domain Rules** — business logic not obvious from code
6. **The Weird Parts** — dragons, legacy, quirks
7. **Environment** — what's needed to run it

Keep it under 300 lines. Update it as you learn.
