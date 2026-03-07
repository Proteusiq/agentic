## Two files. Infinite context. Zero repetition.

---

## The Problem

You open your AI editor. You paste your code. You wait.

```
AI:  Here's the solution using `requests`...
You: We use httpx.
AI:  Here's the updated solution with classes...
You: We prefer functions.
AI:  Here's the refactored version...
You: We use ruff, not black.
AI:  *rewrites everything again*
```

**You've done this dance before.** Every session. Every project. Every time.

AI agents are powerful. But they're flying blind:

- They don't know you prefer `X | None` over `Optional[X]`
- They don't know your API has a layered architecture
- They don't know that `users.py` has a quirk that breaks everything
- They don't know *anything* until you tell them. Again.

---

## The Fix

```
your-project/
├── AGENTS.md     # How you write code
└── SKILL.md      # What this project is
```

That's it. Two markdown files.

| File | What It Does | Shared? |
|------|--------------|---------|
| **AGENTS.md** | Your development conventions | Yes — symlink across all projects |
| **SKILL.md** | This repo's domain knowledge | No — unique per project |

Agents read these files. They *get it*. First try.

---

## Quick Start

=== "Scaffold CLI"

    ```bash
    # Clone and build
    git clone https://github.com/Proteusiq/agentic.git
    cd agentic/scaffold && cargo build --release
    
    # Install
    cp target/release/scaffold /usr/local/bin/
    
    # Use
    scaffold -d ~/code/my-project --link
    ```

=== "Manual"

    ```bash
    curl -O https://raw.githubusercontent.com/Proteusiq/agentic/main/AGENTS.md
    curl -O https://raw.githubusercontent.com/Proteusiq/agentic/main/SKILL.template.md
    mv SKILL.template.md SKILL.md
    ```

=== "Just Copy"

    Grab [AGENTS.md](https://github.com/Proteusiq/agentic/blob/main/AGENTS.md) and [SKILL.template.md](https://github.com/Proteusiq/agentic/blob/main/SKILL.template.md) from the repo.

---

## How It Works

```
┌─────────────────────────────────────────────────────────────┐
│                        AGENTS.md                            │
│                                                             │
│  "Use httpx, not requests"                                  │
│  "Run ruff before commit"                                   │
│  "Functional over OOP"                                      │
│                                                             │
│  ↓ symlinked to every project                               │
├─────────────────────────────────────────────────────────────┤
│                        SKILL.md                             │
│                                                             │
│  "This is a payment API"                                    │
│  "Prices in cents, displayed in dollars"                    │
│  "Never delete subscriptions, cancel them"                  │
│                                                             │
│  ↓ unique per project                                       │
└─────────────────────────────────────────────────────────────┘
                              ↓
                    AI reads both files
                              ↓
                 Writes code YOUR way, first try
```

Update conventions once → every project gets them.  
Keep domain knowledge focused → agents load fast.

---

## Works With

Skills follow the [Agent Skills](https://agentskills.io) open standard.

| Agent | How to Enable |
|-------|---------------|
| **Claude Code** | Reads `AGENTS.md` and `SKILL.md` automatically |
| **OpenCode** | Symlink to `~/.config/opencode/skills/` |
| **Cursor** | Reads `.cursorrules` — rename or copy |
| **Any LLM** | Paste the file contents or use as system prompt |

---

## Dive In

| Page | What You'll Learn |
|------|-------------------|
| [Philosophy](philosophy.md) | Why two files? Why these conventions? The thinking behind Agentic. |
| [Writing AGENTS.md](agents-md.md) | How to write conventions that AI agents actually follow. |
| [Writing SKILL.md](skill-md.md) | Capture your project's DNA so agents understand it instantly. |
| [Scaffold CLI](scaffold.md) | The Rust CLI that sets up both files in seconds. |
| [Examples](examples.md) | Real-world SKILL.md files from production projects. |
