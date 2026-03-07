---
name: trustpilot-ai
description: Architecture, prompts, domain rules, and conventions for the Norlys Trustpilot AI review response system
---

# Trustpilot AI Codebase Knowledge

## Project Overview

Trustpilot AI is an MCP server that helps Norlys respond to Trustpilot reviews using AI-generated replies. It analyzes reviews, drafts personalized responses following Danish customer service conventions, and can post replies via the Trustpilot API.

Repository: `Norlys-A-S/dsai-tai`
Production: Deployed as Azure Container App

## About Norlys (Context for AI)

Norlys is Denmark's largest integrated energy and telecom company:
- **Ownership**: Cooperative (andelsselskab) owned by 800,000+ Danes
- **Products**: Electricity, gas, internet (Denmark's largest fiber network), mobile, TV, EV charging
- **Acquisition**: Acquired Telia Denmark in 2024 (1.9M mobile customers, 800 employees)
- **Scale**: 4,500 employees, 4,300+ mobile towers, 30+ physical stores
- **Mission**: "Skaber positiv forandring gennem fællesskabet" (Creating positive change through community)

**What customers love**: Friendly staff, helpful service, quick solutions, easy processes, personal in-store service
**What frustrates customers**: Internet issues, wait times, hard to reach support

## Layer Architecture

```
settings.py              (leaf: config, env vars, secrets)
types.py                 (leaf: domain models, response types)
    |
client.py                (Trustpilot REST API client)
    |
services/
  analysis.py            (Azure OpenAI: analyze, draft_reply, classify)
  reviewer.py            (LangGraph agent for review workflow)
    |
mcp/
  server.py              (MCP tool definitions, FastMCP server)
    |
cli/
  agent.py               (Interactive CLI tools)
```

Dependencies flow downward. Lower layers never import higher layers.

## Source Layout

```
src/trustpilot/
  __init__.py            # Public API exports
  client.py              # Trustpilot API client (OAuth, reviews, replies)
  settings.py            # pydantic-settings configuration
  types.py               # Domain models (Review, Business, CompanyReply)
  services/
    __init__.py          # Service exports
    analysis.py          # LLM prompts + Azure OpenAI calls
    reviewer.py          # LangGraph agent workflow
  mcp/
    server.py            # FastMCP server with tools
  cli/
    agent.py             # Interactive agent CLI

tests/
  conftest.py            # Fixtures
  test_api.py            # API integration tests
  test_client.py         # Trustpilot client tests
  test_credentials.py    # Env var loading tests
  test_mcp.py            # MCP server tests
  test_validation.py     # Reply validation tests
  test_greeting.py       # LLM greeting extraction tests (integration, slow)
```

## Key Files

| File | Purpose | Key Functions |
|------|---------|---------------|
| `services/analysis.py` | LLM prompts and calls | `analyze_review()`, `draft_reply()`, `classify_review()` |
| `services/reviewer.py` | Agent workflow | `ReviewAgent`, `create_agent()` |
| `mcp/server.py` | MCP tools | `get_review`, `generate_reply`, `post_reply` |
| `client.py` | Trustpilot API | `get_reviews()`, `post_reply()`, `get_review_by_id()` |
| `types.py` | Data models | `Review`, `Business`, `CompanyReply` |

## REPLY_PROMPT Structure (Critical)

The main prompt in `services/analysis.py` has these sections:

| Section | Purpose |
|---------|---------|
| ROLE | Define the AI persona |
| ABOUT NORLYS | Company context for tone |
| CONTEXT | Platform, brand voice, goals |
| LANGUAGE | Danish/English/other handling |
| TONE | Informal "du", company voice "vi/os" |
| GREETING | First name extraction rules |
| BANNED WORDS | Words to never use |
| STRUCTURE (positive) | Template for 4-5 star reviews |
| STRUCTURE (negative) | Template for 1-2 star reviews |
| SIGN-OFF | Required format and names |
| CONTACT POLICY | When to include contact info |
| LENGTH | Match reply depth to review depth |
| CONSTRAINTS | Formatting rules |
| BAD VS GOOD EXAMPLES | Concrete examples of what to avoid |
| EVIDENCE | The actual review data |
| ACCEPTANCE CRITERIA | Checklist for validation |

## Danish Language Rules

### Tone
- **Always informal**: Use "du", NEVER formal "De"
- **Company voice**: Use "vi"/"os", NEVER "jeg"/"mig" alone
- **Neighborly**: Like talking to a neighbor, not a corporation

### Banned Words (NEVER use)
- "De" (formal you)
- "jeres" (use "din"/"dit" instead)
- "tilbagemelding" (use "anmeldelse" instead)
- "driller"
- "glat"
- "pensionist"
- "super"
- "nogensinde"
- "fedt" (too casual/slang)

### Greeting Extraction

The LLM extracts first names from `consumer_name` field:

| Display Name | Greeting | Rule |
|--------------|----------|------|
| Peter Jensen | Hej Peter | Simple first name |
| Sofie Marie Ploug | Hej Sofie Marie | Danish double first name |
| Anne-Marie Larsen | Hej Anne-Marie | Hyphenated stays together |
| SØREN HANSEN | Hej Søren | Fix capitalization |
| peter123 | Hej | Username/handle → no name |
| MIKKELSEN | Hej | Just a surname → no name |
| K. Nielsen | Hej | Initial only → no name |

### Language Fallback
- Danish review → reply in Danish
- English review → reply in English
- Any other language → reply in English

## Reply Quality Rules

### Length Matching
Match reply depth to review depth:
- "Fin service" → Short reply: "Hej\n\nTak for din anmeldelse!\n\nDe bedste hilsner\nMai, Norlys ❤️"
- Detailed story → Warm, thoughtful response acknowledging specifics
- Frustrated rant → Address core concern, offer help, stay concise

**Golden rule**: Reply should never feel heavier than the review itself

### Anti-Parroting Rules

1. **Don't mirror exact words**:
   - ❌ "...du oplevede vores medarbejdere som både hyggelige og imødekommende"
   - ✅ "...du følte dig godt modtaget"

2. **Don't repeat negative framing**:
   - ❌ "...få løst det problem, du stod med"
   - ✅ "...fik den løsning, du havde behov for"

3. **Pick one point when review has multiple**:
   - ❌ Mirror both "konstruktivt" AND "brugte rigelig tid"
   - ✅ Focus on most meaningful: "god tid til at gennemgå tingene"

4. **Focus on outcome, not journey**:
   - ❌ "efter flere forsøg fik den hjælp..."
   - ✅ "fandt en god løsning"

### Sign-Off Format

Always use exactly:
```
De bedste hilsner
[Name], Norlys ❤️
```

Allowed names: Mai, Nora, Sofie, Oscar, Carl, William, Emma, Ella, Alma

NEVER use: "Customer Service", "Kundeservice", etc.

### Contact Policy

- Do NOT offer contact info unless the issue REQUIRES it
- Phone: 70 11 40 40 (digital) or 80 40 40 40 (mobil)
- Email: kontakt@norlys.dk or kontakt.digital@norlys.dk
- Most positive reviews need NO contact info

## Validation Rules

Reply validation in `mcp/server.py` checks:

| Rule | Description |
|------|-------------|
| `has_valid_signoff_name` | Ends with allowed name + "Norlys" |
| `has_greeting_before_signoff` | Has "De bedste hilsner" or similar |
| `has_bullet_points` | No bullets (-, *, •) |
| `has_em_dashes` | No em/en dashes (–, —) |
| `max_one_emoji` | Only ❤️ in sign-off |

## Testing

### Regular Tests (fast, no LLM)
```bash
uv run pytest                    # Runs all except integration
uv run pytest -v                 # Verbose
```

### Integration Tests (slow, requires LLM)
```bash
uv run pytest -m integration     # Run LLM tests
uv run pytest -m "not integration"  # Skip LLM tests (default)
```

Integration tests are marked with `@pytest.mark.integration` and `@pytest.mark.slow`.

### Verification Loop (before commit)
```bash
uvx ruff format .
uvx ruff check --fix .
uvx ty check .
uv run pytest
```

## MCP Tools

### Safe Tools (read-only, can test freely)

| Tool | Description |
|------|-------------|
| `status` | Check API connectivity |
| `get_business_stats` | Get trust score, star distribution |
| `find_business` | Find business by domain |
| `search_business` | Search businesses by name |
| `get_latest_reviews` | Fetch recent reviews |
| `get_unreplied_reviews` | Fetch reviews without replies |
| `get_negative_unreplied_reviews` | Fetch 1-2 star unreplied |
| `get_positive_unreplied_reviews` | Fetch 4-5 star unreplied |
| `get_reviews_since` | Reviews from last N hours/days |
| `get_reviews_between` | Reviews in date range |
| `get_review` | Get single review by ID |
| `get_review_tags` | Get tags for a review |
| `get_review_likes` | Get likes for a review |
| `analyze_review_by_id` | LLM analysis (sentiment, urgency) |
| `classify_review_by_id` | LLM classification (business unit) |
| `generate_reply` | Generate draft reply (does NOT post) |
| `start_analysis` | Async analysis task |
| `start_classification` | Async classification task |
| `start_generate_reply` | Async reply generation task |
| `get_task_result` | Check async task status |

### Dangerous Tools (NEVER test - modifies production data)

| Tool | Why Dangerous |
|------|---------------|
| `send_reply` | Posts reply to Trustpilot |
| `confirm_send_reply` | Confirms and posts reply |
| `delete_reply` | Deletes existing reply |
| `confirm_delete_reply` | Confirms and deletes reply |
| `add_review_tags` | Adds tags to review |
| `set_review_tags` | Replaces all tags on review |
| `remove_review_tag` | Removes tag from review |
| `create_find_reviewer_request` | Sends email to customer |

### MCP Prompts

| Prompt | Description |
|--------|-------------|
| `review_workflow` | Step-by-step workflow guide |
| `danish_reply_guidelines` | Danish language rules |

## Environment Variables

| Variable | Description |
|----------|-------------|
| `TRUSTPILOT_API_KEY` | Trustpilot API key |
| `TRUSTPILOT_SECRET_KEY` | Trustpilot API secret |
| `TRUSTPILOT_BUSINESS_ID` | Default business unit ID |
| `TRUSTPILOT_BUSINESS_USER_ID` | User ID for posting replies |
| `AZURE_OPENAI_KEY` | Azure OpenAI API key |
| `AZURE_OPENAI_ENDPOINT` | Azure OpenAI endpoint URL |
| `AZURE_OPENAI_MODEL` | Model deployment name |

## Deployment

Deployment via GitHub Actions workflow dispatch:

```bash
# Deploy to dev
gh workflow run "Main Continous Deployment API Endpoint" --field environment=dev

# Deploy to tst (release triggers tst, then requires approval for prd)
gh workflow run "Main Continous Deployment API Endpoint" --field environment=release
```

Environments:
- `dev`: Development testing
- `tst`: Pre-production testing
- `prd`: Production (requires approval after tst succeeds)

## Git Workflow

```bash
# Feature branch
git checkout -b feat/my-feature

# Commit format
git commit -m "feat: description"
git commit -m "fix: description"
git commit -m "refactor: description"

# Tag releases
git tag -a v1.2.0 -m "v1.2.0: Description"
git push origin main --tags
```

## Common Issues

### "De" instead of "du"
The LLM sometimes uses formal Danish. Add to BANNED WORDS and add examples.

### Too long replies for short reviews
Add LENGTH section with proportionality rules and concrete examples.

### Parroting customer's words
Add BAD VS GOOD EXAMPLES section showing anti-mirroring patterns.

### Username instead of first name
Add GREETING rules for detecting handles (numbers, underscores, @).

### Danish double first names
"Sofie Marie" should be used together, not just "Sofie". Add examples in GREETING.

## Trustpilot Review Stats (from analysis)

From 1,000 recent reviews:
- 5★: 49.7%
- 4★: 14.7%
- 3★: 5.1%
- 2★: 4.3%
- 1★: 26.2%

**Negative review themes**: Internet/fiber issues, contact difficulties, customer service, mobile/billing, wait times

**Positive review themes**: Good service, helpful staff, easy processes, friendly, fast resolution

## Local Development & Testing

### Running MCP Server Locally

```bash
# Start MCP server on port 8765
cd /Users/pradan/Codes/dsai/dsai-tai
nohup uv run python -c "
from trustpilot.mcp.server import mcp
mcp.run(transport='http', host='127.0.0.1', port=8765)
" > /tmp/mcp-server.log 2>&1 &

# Check logs
cat /tmp/mcp-server.log

# Stop server
pkill -f "trustpilot.mcp.server"
```

### Running MCP Inspector (MCPJam)

```bash
# Start MCPJam inspector
npx @mcpjam/inspector@latest

# Opens at http://127.0.0.1:6274
# Connect to: http://127.0.0.1:8765/mcp
```

### Testing with curl

```bash
# List tools
curl -s -X POST http://127.0.0.1:8765/mcp \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","id":1,"method":"tools/list"}' | jq '.result.tools[].name'

# Get business stats
curl -s -X POST http://127.0.0.1:8765/mcp \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":"get_business_stats","arguments":{}}}' | jq

# List prompts
curl -s -X POST http://127.0.0.1:8765/mcp \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","id":1,"method":"prompts/list"}' | jq
```

### CRITICAL: Testing Rules

**NEVER test these operations** (they modify production data):
- `post_reply` / `send_reply` / `confirm_send_reply` - Posts reply to Trustpilot
- `delete_reply` / `confirm_delete_reply` - Deletes existing reply
- `add_review_tags` / `set_review_tags` / `remove_review_tag` - Modifies review tags
- `create_find_reviewer_request` - Sends email to reviewer

**Safe to test** (read-only operations):
- `status` - Check API health
- `get_business_stats` - Get statistics
- `get_latest_reviews` / `get_unreplied_reviews` - Fetch reviews
- `get_reviews_since` / `get_reviews_between` - Date-filtered reviews
- `get_review` - Get single review
- `get_review_tags` - Read tags
- `get_review_likes` - Read likes
- `analyze_review` - LLM analysis (read-only)
- `classify_review` - LLM classification (read-only)
- `generate_reply` - Generate draft (does NOT post)
- `validate_reply` - Check reply rules

## Related Issues

- Issue #3: Improve REPLY_PROMPT for natural Danish responses (closed)
- Issue #2: Archive eTray case when replying (open, blocked)
