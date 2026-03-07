---
name: ufarm-codebase
description: Architecture map, data flow, table definitions, unit conversion state, and domain rules for the UFarm codebase
---

# UFarm Codebase Knowledge

## Project Overview

UFarm is a unified farm management platform for Agroindustrial San Antonio (Honduras). It manages crops, cattle, land, and security across four farms: San Antonio, San Sebastián, El Achiote, and Valle de Santos.

The system ingests Excel data from Google Drive, loads it into DuckDB, and serves it via a FastAPI API with an admin panel. Luis (farm admin) updates spreadsheets in Google Drive; data is pulled into UFarm on request.

Production URL: https://ufarm.atlan-innovation.com (Hetzner VPS, Docker, Angie reverse proxy).

## User Story Acceptance Criteria (SACRED - DO NOT MODIFY)

**Test Location:** `tests/integration/test_user_story_acceptance.py`

These acceptance criteria are the **source of truth** for the Active Crops View feature. The values come directly from the farm admin and represent real production data that must be matched exactly. 

**NEVER modify these tests without explicit user permission.** If tests fail, fix the functionality, not the tests. The code must conform to these acceptance criteria, not the other way around.

### Example 1: Chile Lamuyo 2024/C1 - El Achiote

| Field | Expected Value |
|-------|----------------|
| Planting Period | June 6, 2024 – June 7, 2024 |
| Harvest Start Date | August 6, 2024 |
| Harvest End Date | November 5, 2024 |
| Harvest Duration | 91 days |
| Crop Duration | 153 days |
| Area | 1.04 hectares |
| Production | 647.05 sacks |
| Yield | 622.16 sacks/hectare |

### Example 2: Jalapeño Mixteco 2024/C1 - San Antonio

| Field | Expected Value |
|-------|----------------|
| Planting Period | July 18, 2024 – July 31, 2024 |
| Harvest Start Date | September 12, 2024 |
| Harvest End Date | March 19, 2025 |
| Harvest Duration | 188 days |
| Crop Duration | 245 days |
| Area | 2.02 hectares |
| Production | 578,608 pounds |
| Yield | 260,634.23 lbs/hectare |
| Unit Toggle | Pounds ↔ Quintals (1 quintal = 100 lbs) |

### Example 3: Chile Lamuyo 2025/C1 - El Achiote

| Field | Expected Value |
|-------|----------------|
| Planting Date | June 25, 2025 (single day) |
| Harvest Start Date | August 23, 2025 |
| Harvest End Date | November 8, 2025 |
| Harvest Duration | 77 days |
| Crop Duration | 137 days |
| Area | 1.04 hectares |
| Production | 585.72 sacks + 2,000 units |
| Yield | 563.19 sacks/hectare |

### Example 4: Pepino Braga 2025/C1 - San Sebastián

| Field | Expected Value |
|-------|----------------|
| Planting Date | December 20, 2024 (single day) |
| Harvest Start Date | January 28, 2025 |
| Harvest End Date | March 3, 2025 |
| Harvest Duration | 34 days |
| Crop Duration | 74 days |
| Area | 0.31 hectares |
| Production | 209 sacks |
| Yield | 674.19 sacks/hectare |

### Example 5: Pepino Braga 2025/C1 - El Achiote

| Field | Expected Value |
|-------|----------------|
| Planting Date | March 18, 2025 (single day) |
| Harvest Start Date | May 7, 2025 |
| Harvest End Date | June 14, 2025 |
| Harvest Duration | 38 days |
| Crop Duration | 89 days |
| Area | 0.68 hectares |
| Production | 842 sacks |
| Yield | 1,238.24 sacks/hectare |

### Calculation Rules (derived from acceptance criteria)

- **Harvest Duration** = `harvest_end - harvest_start` (in days, not inclusive)
- **Crop Duration** = `harvest_end - planting_start + 1` (in days, inclusive of both dates)
- **Yield** = Primary production unit quantity / area_ha
- **Primary unit for yield**: First unit in `CROP_UNITS` list (Sacks for Chile Lamuyo, Pounds for Jalapeño)

## Layer Architecture

```
settings.py          (leaf: config, paths, constants)
db/schema.py         (leaf: column mappings, table definitions)
    |
services/            (Google Drive, Sheets, Auth, Crops logic)
    |
db/loader.py         (ETL: Excel -> DuckDB)
    |
api/                 (FastAPI routers, schemas, deps)
    |
cli/                 (Typer CLI entry points)
```

Dependencies flow downward. Lower layers never import higher layers.

## Source Layout

```
src/ufarm/
  __init__.py
  settings.py              # DATA_PATH, SECRET_KEY, FILES registry, FARMS, MAIN_CROPS
  cli/
    main.py                # CLI entry point (Typer)
    data.py                # Data extraction commands
    db.py                  # Database commands
  db/
    schema.py              # COLUMN_MAPPINGS, SHEET_TO_TABLE, ColumnMapping TypedDict
    loader.py              # ETL: read Excel, rename columns, transform, load to DuckDB
    engine.py              # SQLAlchemy engine (DuckDB), get_session() dependency
  services/
    extractor.py           # Google Drive file downloads
    google_auth.py         # OAuth credential management
    google_drive.py        # Drive API client
    google_sheets.py       # Sheets API client
    crops.py               # Crops business logic (711 lines, query + CRUD + cycle duration)
  api/
    main.py                # FastAPI app, middleware, lifespan, admin panel
    deps.py                # get_current_user, require_role dependencies
    routers/
      auth.py              # /auth/login, /auth/me
      users.py             # /users/ CRUD (admin only), hard delete
      crops.py             # /crops/* endpoints (stats, performance, summary, cycles, durations)
      health.py            # /health
    schemas/
      crops.py             # Pydantic models for all crop responses
  auth/
    models.py              # SQLAlchemy Base, User model (auth_users table)
    security.py            # Argon2 hashing (pwdlib), JWT (pyjwt)

tests/
  conftest.py              # Fixtures: test_db, test_client, user fixtures (pytest-env for SECRET_KEY)
  unit/
    auth/
      test_auth_models.py    # User model tests
      test_security.py       # Password hashing, JWT tests
    db/
      test_loader.py         # ETL loader tests
      test_schema.py         # Schema mapping tests
      test_unit_conversions.py # Presentation enrichment, normalization, filtering, yield tests
    services/
      test_crops_service.py  # Crops CRUD validation tests
      test_cycle_duration.py # compute_cycle_duration pure function tests
    test_settings.py         # Settings tests
  integration/
    api/
      test_auth_api.py       # Auth endpoint integration tests
      test_users_api.py      # Users endpoint integration tests
```

## Data Flow: Excel to API

```
Google Drive (BDProduccion.xlsx)
        |
        v
  services/extractor.py      downloads Excel files to data/
        |
        v
  db/loader.py                ETL pipeline:
    1. pl.read_excel()           read sheets (Polars)
    2. rename columns            Spanish -> English via schema.py
    3. transform                 table-specific transforms
    4. _enforce_schema_types()   cast to DuckDB types
    5. DROP + CREATE TABLE       write to DuckDB
        |
        v
  data/ufarm.duckdb           11 tables (see below)
        |
        v
  services/crops.py           SQL queries + Polars post-processing
        |
        v
  api/routers/crops.py        FastAPI endpoints -> JSON responses
```

## DuckDB Tables

### Crops Domain

| Table | Source Sheet | Key Columns |
|-------|-------------|-------------|
| production | Produccion | detail_id, date, client_id, farm_id, product_id, quantity |
| products | Productos | product_id, farm, name, category, presentation, crop_category_id |
| crop_categories | CategoriaCultivo | category_id, name |
| crop_cycles | CicloTemporada | cycle_id, farm_id, category_id, season, cycle, harvest_start_date, planting_date, area_ha |
| farms | Fincas | farm_id, name, location |
| clients | Clientes | client_id, name, tax_id, phone |
| users | Usuarios | user_id, farm_id, name, level, email |
| categories | Categoria | category_id, name |
| presentations | Presentaciones | presentation_id, presentation, total_pounds, unit_type, name_en |

### Other Domains

| Table | Source |
|-------|--------|
| plots | Three farm Excel files (land) |
| cattle | Two lot Excel files |
| transplanting | Trasplante_Lotes.xlsx |

### ETL Transforms

- **crop_cycles**: area converted from Manzanas to Hectares (area * 0.7), column renamed to area_ha; filtered to monitored categories only
- **crop_categories**: filtered to monitored categories only
- **presentations**: enriched with unit_type (weight/volume/count/batch) and name_en (English translation)
- **products**: presentation names normalized for case matching; filtered to category=1 (Crops) only

### Monitored Crop Categories

Only these 10 categories (by `category_id`) are loaded into the database. All others are excluded at ETL time.

| category_id | Name               |
|-------------|--------------------|
| 3           | Café               |
| 4           | Chile Lamuyo       |
| 5           | Elote              |
| 7           | Jalapeño Amuleto   |
| 8           | Jalapeño Mixteco   |
| 11          | Maíz Amarillo      |
| 12          | Maíz DK-390        |
| 14          | Pepino Braga       |
| 16          | Tomate Pera        |
| 18          | Jalapeño Poderoso  |

Excluded: Banano (1), Botes Plásticos (2), Guate de Maíz (6), Jalapeño Procesado (9), Leña (10), Pepino (13), Plátano (15), Tusa de Elote (17).

Defined as `MONITORED_CROP_CATEGORIES` in `loader.py`. Uses integer IDs, not names.

### Units by Crop

Each crop has specific units it can be displayed in. Defined in `CROP_UNITS` in `settings.py`.

| Crop | Units | Notes |
|------|-------|-------|
| Café | Pounds | Weight only |
| Chile Lamuyo | Sacks, Units | Volume-based (sacks not weighed) |
| Elote | Manos | 1 Mano = 5 units |
| Jalapeño Amuleto | Pounds, Quintals | 1 quintal = 100 lbs |
| Jalapeño Mixteco | Pounds, Quintals | 1 quintal = 100 lbs |
| Jalapeño Poderoso | Pounds, Quintals | 1 quintal = 100 lbs |
| Maíz Amarillo | Pounds, Quintals | 1 quintal = 100 lbs |
| Maíz DK-390 | Pounds, Quintals | 1 quintal = 100 lbs |
| Pepino Braga | Sacks, Units | Volume-based |
| Tomate Pera | Pounds | Internally converted from boxes |

## Presentations and Unit Conversions

The presentations table maps product packaging to weight conversion factors.

### PRESENTATION_ENRICHMENTS (16 entries)

| Presentation (Spanish) | unit_type | name_en | total_pounds |
|------------------------|-----------|---------|-------------|
| Libra | weight | Pound | 1 |
| Quintal | weight | Quintal | 100 |
| Saco (100 Lbs.) | weight | 100 lb Sack | 100 |
| Caja de Carton 26 Lbs. | weight | Cardboard Box 26 lbs | 26 |
| 33 Libras | weight | 33 Pounds | 33 |
| Caja Plastica 35 Lbs. | weight | Plastic Crate 35 lbs | 26 (internal conversion) |
| Saco | volume | Sack | NULL |
| Caja | volume | Box | NULL |
| Paila | volume | Bucket | NULL |
| 20 Litros | volume | 20 Liters | NULL |
| Manos | count | Hands | 5 (ears per hand) |
| Racimo | count | Bunch | NULL |
| Unidad | count | Unit | NULL |
| Ensilado | batch | Silage | NULL |
| Viaje | batch | Trip | NULL |
| 16 onzas / 32 Oz Vidrio | weight | 16 oz / 32 oz Glass | keeps original |

### Conversion at Query Time

Unit conversion happens in SQL within `services/crops.py`, not during ETL:

```sql
SUM(p.quantity * COALESCE(pres.total_pounds, 1)) AS total_quantity
```

- Weight items: quantity * total_pounds = pounds
- Volume/count/batch items: total_pounds is NULL, COALESCE gives 1, so raw count preserved

Join path: `production.product_id` -> `products.presentation` -> `presentations.presentation` -> `presentations.total_pounds`

## Crops API Endpoints

All endpoints require authentication. CRUD operations require editor role.

| Method | Path | Description | Unit Conversion |
|--------|------|-------------|----------------|
| GET | /crops/stats | Year-over-year KPIs | Yes |
| GET | /crops/farms | List farms | No |
| GET | /crops/categories | List crop categories | No |
| GET | /crops/cycles | List crop cycles | No |
| GET | /crops/durations | Average cycle duration by crop | No |
| GET | /crops/performance | Production totals by farm/crop/season | Yes |
| GET | /crops/summary | Cycles + production + yield_per_ha | Yes |
| GET | /crops/cycles/{id} | Single cycle | No |
| POST | /crops/cycles | Create cycle (editor) | No |
| PUT | /crops/cycles/{id} | Update cycle (editor) | No |
| DELETE | /crops/cycles/{id} | Delete cycle (editor) | No |

API response conventions: all weights in pounds (Lb), all areas in hectares (Ha), yield in Lb/Ha.

## Cycle Cards (Active Crops View)

`get_cycle_cards()` in `services/crops.py` returns full cycle data for the frontend Active Crops View.

Each card includes:
- **Farm, crop, season, cycle**: Basic identification
- **Planting period**: `transplant_start_date` and `transplant_end_date` from transplanting table
- **Harvest dates**: `harvest_start_date` (first production) and `harvest_end_date` (last production)
- **Durations**: `crop_duration_days` (planting start to harvest end), `harvest_duration_days` (harvest start to end)
- **Area**: `area_ha` in hectares
- **Production**: List of `{unit, quantity}` for each unit type (Pounds, Sacks, Manos, Units)
- **Yield**: `yield_per_ha` in primary unit per hectare
- **Available units**: List of unit names from `CROP_UNITS` for unit toggle
- **Status**: `is_active` (True if no successor cycle exists)

Transplanting data comes from `Trasplante_Lotes.xlsx` with per-plot dates aggregated by MIN/MAX.

## Cycle Duration Logic

`compute_cycle_duration()` in `services/crops.py` is a pure function:

- Cycles ordered by (farm_id, category_id, harvest_start_date)
- First cycle in a farm+crop group: cycle_start_date = harvest_start_date
- Subsequent cycles: cycle_start_date = previous cycle's cycle_end_date
- cycle_end_date = last production date before next cycle's harvest_start_date
- Active (last) cycle has NULL cycle_end_date
- Cross-season successors close earlier cycles

## Auth System

- SQLAlchemy User model in auth_users table (DuckDB)
- Password hashing: Argon2 via pwdlib
- JWT tokens: pyjwt with HS256
- Roles: admin, editor, viewer
- Admin panel: sqladmin at /admin/
- DELETE /users/{id} performs hard delete (not soft delete)

## Domain Rules (from Luis, farm admin)

### Product Categories (in Productos sheet)

- Category 1 = Crops (actively tracked for production)
- Category 2 = Processed products (sauces, vinegars — separate system)
- Category 3 = Other products (recyclables, secondary items)

### Units by Crop

**Jalapeño Mixteco**: pounds and quintals (1 quintal = 100 lbs, sold in 100 lb sacks). Users must toggle between pounds and quintals since clients vary.

**Chile Lamuyo (Bell Pepper)**: sacks (volume-based, not weighed, price = market rate) and units (one client buys by individual pepper count).

**Tomato** (most complex):
- Caja de Cartón 26 lbs: quantity * 26 = total pounds
- 33 Libras: quantity * 33 = total pounds
- Caja Plástica 35 lbs: client-provided 35 lb crate, internally converted to 26 lb box equivalent

**Cucumber**: sacks only (volume-based, not weighed).

**Other crops**:
- Banana: bunch (counted, not weighed)
- Plantain: unit (individual count)
- Corn (Elote): mano (1 mano = 5 ears)
- Silage Corn: batch (fixed total price, rare, Finca El Achiote only)

### Sales Data and Mount Dora Farms Rule

Three source tables for production/rejections:
- **Ventas MD-PBI**: official Mount Dora Farms report with real delivery dates and final accepted weights
- **Ventas Plaza-PBI**: plaza sales — Mount Dora Farms appears here but MUST BE EXCLUDED (its dates are payment-received dates, not harvest dates; including it duplicates production)
- **VDS-Entrega Cocina**: jalapeño volume delivered to industrial kitchen for sauce/pickled processing

### Cycle Logic (agreed with Luis)

- Start date of new cycle = end date of previous cycle
- Last production date of previous cycle = start date of new cycle
- Cycle duration is a tracked parameter

### CRUD (agreed with Luis)

- UPDATE: crop size (resize), harvest date, name (preserving related data), planting date, locations
- DELETE: delete a cycle (with option to recreate)
- CREATE: size, harvest date, planting date, location

### Data Updates

Luis updates spreadsheets in Google Drive. Changes are pulled to UFarm on request (not automatic sync).

## Active Crops (10 monitored)

Only these crops are actively tracked. All others should be filtered out of the dashboard.

| Crop | Category ID | Presentation | Unit Type | Weight |
|------|-------------|--------------|-----------|--------|
| Café | 3 | Libra | weight | 1 lb |
| Chile Lamuyo | 4 | Saco, Caja, Unidad | volume/count | - |
| Guate de Maíz | 6 | Ensilado | batch | - |
| Jalapeño Amuleto | 7 | Libra | weight | 1 lb |
| Jalapeño Mixteco | 8 | Libra, Saco (100 Lbs.) | weight | 1 lb, 100 lbs |
| Maíz Amarillo | 11 | Saco (100 Lbs.) | weight | 100 lbs |
| Maíz DK-390 | 12 | Saco (100 Lbs.) | weight | 100 lbs |
| Pepino Braga | 14 | Saco | volume | - |
| Tomate Pera | 16 | Caja de Cartón 26 Lbs., 33 Libras, Caja Plástica 35 Lbs. | weight | 26, 33, 26 lbs |
| Jalapeño Poderoso | 18 | Libra | weight | 1 lb |

### Saco Clarification

"Saco" has two meanings:
- **Saco (100 Lbs.)**: 100-pound sack = 1 quintal. Used for Jalapeño Mixteco (QQ), Maíz Amarillo, Maíz DK-390.
- **Saco** (no weight): Volume-based container with no defined weight. Used for Chile Lamuyo, Pepino Braga.

### Crops with lb ↔ qq Toggle

Only crops with `total_pounds > 1` should show the toggle:
- Jalapeño Mixteco (QQ) - 100 lbs
- Maíz Amarillo - 100 lbs
- Maíz DK-390 - 100 lbs
- Tomate Pera - 26 or 33 lbs per box

Crops already in pounds (Café, Jalapeño Amuleto, Jalapeño Poderoso) don't need toggle.

## Issue #34 Status: Unit Conversions

### Completed

- #39 presentations table enriched with unit_type and name_en (ETL + tests)
- #40 conversion logic in crops service via SQL COALESCE pattern (service + tests)
- #52 API + frontend unit display (UnitType enum, unit fields in CropSummary, frontend toggle)

### Open Issues

- #55 Move lb/qq toggle from filter bar to individual crop cards
- #56 Filter dashboard to show only 10 active crops

### Pending Clarification (Luis)

- SAC4 (Jalapeño Mixteco QQ) and SAC7 (Jalapeño Mixteco Descarte QQ) are the only products explicitly in Saco (100 Lbs.). Should lb ↔ qq toggle apply only to these, or also to other weight-based crops like Tomate Pera?

### Open Questions

- Rejections tracking (#34 mentions total rejected pounds per crop type) — source tables identified but not yet implemented
- Volume-based items (sacks): currently COALESCE to 1 (raw count), unclear if yield calculations should handle differently

## Known Data Anomalies (Issue #57)

### Date Anomalies

Three crop cycles have harvest dates before transplant dates:

| Farm | Crop | Cycle | Harvest | Transplant | Gap |
|------|------|-------|---------|------------|-----|
| El Achiote | Tomate Pera | 2024/C1 | 2024-01-06 | 2024-04-13 | -98 days |
| San Sebastián | Pepino Braga | 2025/C1 | 2025-01-02 | 2025-02-20 | -49 days |
| El Achiote | Chile Lamuyo | 2024/C1 | 2024-01-09 | 2024-01-11 | -2 days |

These need clarification from Luis before correcting.

### Production Data Discrepancy

**Jalapeño Mixteco 2024/C1 - San Antonio** has a production discrepancy:

| Metric | Acceptance Criteria | Database Value | Difference |
|--------|---------------------|----------------|------------|
| Production | 578,608 lbs | 540,781 lbs | -37,827 lbs (6.5% short) |
| Yield | 260,634.23 lbs/ha | 267,713.37 lbs/ha | +2.7% (based on actual data) |

The acceptance criteria tests currently fail for this cycle. Possible causes:
1. Database not fully synced with latest spreadsheet data
2. Acceptance criteria based on projected/estimated values
3. Some production recorded under different category

**Action Required:** Verify with Luis that BDProduccion.xlsx is fully synced and investigate missing 37,827 lbs.

### Crop Name Aliases (Fixed)

The transplanting table uses "Pepino Braga F1" but crop_categories uses "Pepino Braga". This is handled by `CROP_NAME_ALIASES` in `services/crops.py` which normalizes crop names before joining.

## Technical Debt

### Manual Cycle Adjustment Sentinel Values

`db/production.py` uses sentinel values to create synthetic records when users manually override cycle totals:

| Constant | Value | Purpose |
|----------|-------|---------|
| `MANUALLY_ADJUSTED_PRODUCT_ID` | `"MANUAL-ADJ"` | Fake product_id (real ones are EAC#, SAC#, SSC#) |
| `MANUALLY_ADJUSTED_CLIENT_ID` | `0` | Fake client_id (real ones are positive integers) |

**Problem:** This breaks FK integrity with the `products` table. The JOINs in `get_plaza_cycle_total` naturally exclude these records since no product matches, but it's a brittle design.

**Cleaner alternative:** Create a separate `cycle_adjustments` table with columns `(farm_id, category_id, harvest_end, adjustment_lb)` instead of polluting the production table with fake records.

## Pre-Commit Philosophy Checklist

**Run this checklist before every commit.** See `AGENTS.md` for full conventions.

### Verification Loop (must all pass)

```bash
uvx ruff format .
uvx ruff check --fix .
uvx ty check .
uv run pytest
```

### Philosophy Review

| Principle | Check | Pass Criteria |
|-----------|-------|---------------|
| **Simplicity** | Is this the simplest solution that works? | Functions are focused, minimal, do one thing |
| **Self-documenting** | Do comments explain *why*, not *what*? | No obvious comments; rename/refactor instead of commenting |
| **Functional** | Pure functions, composition, no OOP? | No classes unless necessary; isolate side effects |
| **Error handling** | Specific exceptions only? | No bare `except:`; catch `duckdb.CatalogException` not `Exception` |
| **Resources** | Context managers for cleanup? | `with get_connection()` for DB; never manual `.close()` |
| **Testing** | AAA pattern, behavior assertions? | Tests assert expected values, not implementation details |
| **Layer architecture** | Dependencies flow downward? | `settings` → `db/loader` → `services` → `api/cli` |
| **No suppression** | No `noqa`, `type: ignore`? | Fix the code, don't suppress warnings |

### Allowed `ty: ignore` Exceptions

Only two `ty: ignore` uses are permitted in this codebase:

1. **`app.add_middleware()` in `src/ufarm/api/main.py`** — Starlette's `add_middleware` uses a `ParamSpec`-based signature that ty cannot resolve. This is a known upstream limitation ([starlette#2464](https://github.com/encode/starlette/issues/2464)), not a code issue. Each `add_middleware` call has `# ty: ignore[invalid-argument-type]` inline.

2. **`[[tool.ty.overrides]]` for `tests/**`** — Tests intentionally pass wrong types (strings, floats, None) to verify runtime validation guards. The `invalid-argument-type` rule is set to `ignore` for test files only via `pyproject.toml` override.

Everything else must be fixed, not suppressed. No `noqa`, no `type: ignore`, no adding rules to global ignore lists.

### Quick Checks

```bash
# No error suppressions in changed files
git diff --name-only | xargs grep -l "noqa\|type: ignore" || echo "Clean"

# Type check specific files
uvx ty check src/ufarm/services/crops.py src/ufarm/db/loader.py

# Comments audit (should explain why, not what)
grep -n "# " src/ufarm/services/crops.py | head -20
```

### Snapshot Testing with inline-snapshot

Use `inline-snapshot` for freezing database state and API responses. See `tests/integration/test_cycle_cards.py` for examples.

**Key insight:** Snapshots store expected values in git, making it easy to update tests when data changes. However, tests that call database functions still need the database to run - snapshots only validate the *result*, not replace the database call.

**Local workflow (with database):**
```bash
# 1. Write test with empty snapshot()
assert result == snapshot()

# 2. Load database and generate snapshots
uv run ufarm db load
uv run pytest --inline-snapshot=create

# 3. Update when data changes intentionally
uv run pytest --inline-snapshot=fix

# 4. Commit the filled snapshots to git
git add tests/integration/test_cycle_cards.py
```

**CI workflow (no database):**
- Tests that query the database are skipped via `pytestmark = pytest.mark.skipif(...)`
- Snapshots help with local development: auto-update when data changes
- Pure unit tests (no DB) run in CI normally

**Pattern:**
```python
from inline_snapshot import snapshot
from dirty_equals import IsInt, IsDatetime

def test_cycle_card():
    result = get_cycle_card(cycle_id=42)
    
    # Start empty locally, run --inline-snapshot=create to fill
    # Once filled, commit to git so CI has expected values
    assert result == snapshot({
        "id": 42,
        "name": "Chile Lamuyo",
        "area_ha": 1.04,
    })
    
    # For dynamic values, use dirty-equals (preserved on --fix)
    assert result == snapshot({
        "id": IsInt(),
        "created_at": IsDatetime(),
    })
```

**When to use:**
- Acceptance criteria tests (freeze expected values from user stories)
- API response validation
- Database query results

**When to skip (in CI):**
- ETL tests that load from Excel files
- Tests that require `uv run ufarm db load`
- API response validation
- Database query results with complex structures

**Rules:**
- Use `dirty-equals` matchers for dynamic values (IDs, timestamps)
- Run `--inline-snapshot=fix` only when data changes are intentional
- Review diffs before committing updated snapshots

### Security Scan (if auth/API changed)

See `AGENTS.md` for full security checklist. Key checks:

- [ ] Protected endpoints return 401 without token
- [ ] Invalid tokens rejected (garbage, expired, wrong secret)
- [ ] CORS blocks unauthorized origins
- [ ] No hardcoded secrets in `src/`

## Transplanting Data

The `transplanting` table has 381 records but is not linked to `crop_cycles` due to farm name mismatch:
- Transplanting uses "San Antonio"
- Crop cycles use "Finca San Antonio"

Some crops in transplanting don't exist in crop_categories: Ensilaje, Frijol Amadeus, Pepino Braga F1.

## Processed Products

48 processed jalapeño products exist in the database (jars, salsas with presentations like "16 oz Plástico", "32 oz Vidrio"). These are from the industrial processing facility and are NOT displayed on the farm dashboard. They intentionally lack unit_type enrichment.

## Frontend Testing with Playwright

### Architecture: Unified Server

UFarm serves both API and frontend from a **single server on port 8000**:

| Path | Purpose |
|------|---------|
| `/health`, `/auth/*`, `/crops/*`, `/users/*` | API endpoints |
| `/admin/*` | Admin panel |
| `/login.html`, `/pages/*`, `/css/*`, `/js/*` | Frontend static files |

The frontend is mounted at `/` with `html=True`, so it serves `index.html` for directory requests.

### Starting the Server

```bash
export UFARM_SECRET_KEY="test-key-for-scanning-only-32b!"
uv run uvicorn ufarm.api.main:app --port 8000
```

Or in background:
```bash
export UFARM_SECRET_KEY="test-key-for-scanning-only-32b!"
uv run uvicorn ufarm.api.main:app --port 8000 > /dev/null 2>&1 &
```

### Running Playwright Tests

```bash
# Install Chromium (first time only)
uvx playwright install chromium

# Run a test script
uv run --with playwright python test_script.py
```

### Playwright Test Template

```python
"""Template for Playwright frontend tests."""
from playwright.sync_api import sync_playwright
import time

def test_feature():
    with sync_playwright() as p:
        browser = p.chromium.launch(headless=False, slow_mo=500)
        page = browser.new_page()
        
        # Everything runs on port 8000 (API + frontend)
        page.goto('http://localhost:8000/login.html', wait_until='networkidle')
        
        # Login form uses id="email" and id="password"
        page.wait_for_selector('#email', state='visible', timeout=10000)
        page.fill('#email', 'prayson@ufarm.com')
        page.fill('#password', 'prayson')
        page.click('#loginBtn')
        
        # Wait for redirect to crops page
        page.wait_for_url('**/pages/crops.html', timeout=10000)
        print("✓ Logged in successfully")
        
        # Wait for dynamic content to load
        page.wait_for_selector('.crop-card', timeout=10000)
        
        # Your test logic here...
        
        # Keep browser open for visual inspection
        time.sleep(5)
        browser.close()

if __name__ == "__main__":
    test_feature()
```

### Common Selectors

| Element | Selector |
|---------|----------|
| Email input | `#email` |
| Password input | `#password` |
| Login button | `#loginBtn` |
| Crop card | `.crop-card` |
| Unit toggle button | `.btn-unit-toggle-card` |
| Production value | `[data-metric="production-value"]` |
| Production label | `[data-metric="production-label"]` |
| Yield value | `[data-metric="yield-value"]` |
| Yield label | `[data-metric="yield-label"]` |
| Edit button | `.btn-edit` |
| Delete button | `.btn-delete` |

### Test User Credentials

| Email | Password | Role |
|-------|----------|------|
| `prayson@ufarm.com` | `prayson` | editor |

### Cleanup After Tests

```bash
# Stop server
pkill -f "uvicorn ufarm" 2>/dev/null || true

# Remove test files
rm -f test_*.py
```

### Debugging Tips

1. **Use `headless=False`** to see the browser
2. **Use `slow_mo=500`** to slow down actions for visibility
3. **Add `page.screenshot(path="debug.png")`** to capture state
4. **Check `page.url`** to verify navigation worked
5. **Use `wait_until='networkidle'`** for pages that fetch data

### Example: Test Hover Preview on Unit Toggle

```python
"""Test hover functionality - button shows TARGET unit."""
from playwright.sync_api import sync_playwright
import time

def test_hover():
    with sync_playwright() as p:
        browser = p.chromium.launch(headless=False, slow_mo=300)
        page = browser.new_page()
        
        page.goto('http://localhost:8000/login.html', wait_until='networkidle')
        page.fill('#email', 'prayson@ufarm.com')
        page.fill('#password', 'prayson')
        page.click('#loginBtn')
        page.wait_for_url('**/pages/crops.html', timeout=10000)
        page.wait_for_selector('.crop-card', timeout=10000)
        
        toggle_btn = page.locator('.btn-unit-toggle-card').first
        card = toggle_btn.locator('xpath=ancestor::div[contains(@class, "crop-card")]')
        prod_value = card.locator('[data-metric="production-value"]')
        
        # Initial state
        btn_label = toggle_btn.locator('span').text_content()
        value = prod_value.text_content()
        print(f"Initial: button='{btn_label}', value='{value}'")
        
        # Hover (preview)
        toggle_btn.hover()
        time.sleep(0.5)
        btn_hover = toggle_btn.locator('span').text_content()
        value_hover = prod_value.text_content()
        print(f"Hover: button='{btn_hover}', value='{value_hover}'")
        
        # Leave hover (should restore)
        page.locator('body').hover(position={"x": 10, "y": 10})
        time.sleep(0.5)
        btn_after = toggle_btn.locator('span').text_content()
        value_after = prod_value.text_content()
        print(f"After hover: button='{btn_after}', value='{value_after}'")
        
        # Click (convert)
        toggle_btn.click()
        time.sleep(0.5)
        btn_clicked = toggle_btn.locator('span').text_content()
        value_clicked = prod_value.text_content()
        print(f"After click: button='{btn_clicked}', value='{value_clicked}'")
        
        # Verify
        assert value == value_after, "Value should restore after hover"
        assert value_hover == value_clicked, "Click value should match hover preview"
        assert btn_label != btn_clicked, "Button label should change after click"
        print("✓ All assertions passed")
        
        time.sleep(2)
        browser.close()

if __name__ == "__main__":
    test_hover()
```

**Expected behavior:**
- **Button shows TARGET** (what clicking converts TO), not current unit
- **On hover**: values preview conversion, button shows "go back" option
- **On leave**: everything restores to original
- **On click**: conversion happens, button shows new target

## Google Drive Authentication (Local Development)

### Standard Flow (Production)

Use the OOB (Out-of-Band) flow with `credentials/client_secrets.json`:

```bash
uv run ufarm data login
```

This prints a URL, you open it in any browser, sign in, and paste the auth code.

### Emergency Fallback (Local Dev Only)

If the OAuth app is unverified and blocking login, or if API access is restricted:

1. **Manual download**: Go to the Google Drive folder, select all files, right-click → Download
2. Extract the zip to `data/` folder
3. Run `uv run ufarm db load --all` to reload the database

**Important:**
- Token in `credentials/user_token.json` must be gitignored
- For production, always use the proper OOB flow with `client_secrets.json`
