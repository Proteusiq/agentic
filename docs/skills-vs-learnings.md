# Skills vs Learnings

How Anthropic does it vs how we do it.

---

## What Anthropic Built

Based on [leaked source code](https://github.com/x1xhlol/system-prompts-and-models-of-ai-tools), Claude Code uses an elaborate `SKILL.md` system:

```
plugins/
└── plugin-name/
    └── skills/
        └── skill-name/
            ├── SKILL.md          # Domain instructions
            ├── references/       # Detailed docs
            ├── examples/         # Working code
            └── scripts/          # Utilities
```

Each `SKILL.md` contains:

- **Frontmatter** with name, description, trigger phrases
- **Domain-specific workflows** — multi-step procedures
- **Tool integrations** — file formats, APIs
- **Bundled resources** — scripts, references, assets

They use "progressive disclosure":

1. **Metadata** (~100 words) — always in context
2. **SKILL.md body** (<5k words) — when skill triggers
3. **Bundled resources** — as needed

---

## What We Do

One file: `learnings.md`

```
learnings.md    # Everything the LLM discovers
```

The LLM writes it. Documents what it finds. Organizes by topic. Grows over time.

---

## The Comparison

| Aspect | Anthropic's SKILL.md | Our learnings.md |
|--------|---------------------|------------------|
| Structure | Nested directories per domain | Single flat file |
| Who writes | Humans (pre-authored) | LLM (discovered) |
| When loaded | Trigger-based | Always available |
| Content | Static instructions | Dynamic findings |
| Maintenance | Manual updates | LLM maintains |
| Overhead | Plugin ecosystem | None |

---

## Same Goal, Different Path

Both approaches recognize the same truth: **domain-specific knowledge helps**.

Anthropic's solution:
- Pre-author detailed skills
- Bundle with references, examples, scripts
- Load based on trigger phrases
- Complex directory structure

Our solution:
- Let the LLM research in real-time
- Document findings in `learnings.md`
- Always available, no triggers needed
- Simple flat file

---

## Why We Think Simpler Works

### 1. Knowledge Gets Stale

Pre-authored skills become outdated. The Python ecosystem moved from `black` to `ruff`, from `mypy` to `ty`. Static skills don't update themselves.

`learnings.md` is discovered knowledge — researched when needed, updated when wrong.

### 2. Context Rot

A comprehensive skill loads thousands of words of instructions. Most of it irrelevant to the current task.

`learnings.md` contains only what the LLM actually discovered for this project. No bloat.

### 3. Maintenance Burden

Anthropic's approach requires:
- Writing detailed SKILL.md files
- Organizing references and examples
- Maintaining scripts
- Updating trigger phrases

Our approach requires:
- Nothing. The LLM does it.

### 4. One File vs Many

Anthropic's Claude Code has 10+ `SKILL.md` files across nested directories. Each with references, examples, scripts.

We have one `learnings.md` that grows organically.

---

## When Skills Make Sense

To be fair, Anthropic's approach has advantages:

- **Consistency** — same instructions every time
- **Quality control** — human-authored, reviewed
- **Complex workflows** — bundled scripts and tools
- **Team knowledge** — shared across users

If you need deterministic, repeatable workflows with bundled tooling, skills work.

---

## When Learnings Make Sense

Our approach works when:

- **Projects vary** — different needs, different knowledge
- **Tooling evolves** — need current information
- **Simplicity matters** — no plugin overhead
- **LLM capability** — can research and document

For most projects, letting the LLM discover and document beats pre-loading static instructions.

---

## The Hybrid

Nothing stops you from doing both:

1. **AGENTS.md** — minimal rules and workflow
2. **learnings.md** — LLM-discovered knowledge (like SKILL.md content)
3. **todo.md** — working memory

The LLM builds `learnings.md` over time. It becomes your project's skill file — but discovered, not pre-authored.

---

## Summary

Anthropic built an elaborate system to give Claude domain knowledge. We think the same goal is achieved simpler:

> Let the LLM research. Let it document. Keep what it learns.

That's `learnings.md`.
