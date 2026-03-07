# Agentic Culture

A reference library of agent instructions, skills, and conventions for AI-assisted development.

## Overview

This repository provides standardized templates and tooling for creating agent documentation across projects. It implements a **two-file system**:

| File | Purpose | Scope |
|------|---------|-------|
| `AGENTS.md` | Development conventions, coding standards, language-specific rules | Universal (shared across projects) |
| `SKILL.md` | Repository-specific domain knowledge, architecture, API details | Project-specific |

## Quick Start

### Scaffold a New Project

```bash
# Clone this repo (or add to PATH)
git clone https://github.com/Norlys-A-S/dsai-agentic-culture.git
cd dsai-agentic-culture

# Scaffold docs in another project
./scaffold -d /path/to/your/project

# Or link AGENTS.md instead of copying (uses single source of truth)
./scaffold -d /path/to/your/project --link
```

### Manual Setup

1. Copy `AGENTS.md` to your project root
2. Copy `SKILL.template.md` to your project root as `SKILL.md`
3. Edit `SKILL.md` to add project-specific details

## Files

| File | Description |
|------|-------------|
| `AGENTS.md` | Master development conventions (Python, Rust, TypeScript, Bash, Git) |
| `SKILL.template.md` | Template for project-specific knowledge (replace placeholders) |
| `scaffold` | CLI tool to initialize agent docs in a project |
| `examples/` | Real-world SKILL.md examples from production projects |

## The Two-File System

### AGENTS.md (Universal Conventions)

Contains language-agnostic principles and language-specific rules that apply to all projects:

- **Philosophy**: Simplicity, self-documenting code, functional over OOP
- **Cross-Language Principles**: Error handling, testing, documentation
- **Language Sections**: Python, Rust, TypeScript, Bash with tools and workflows
- **Git Conventions**: Commit format, PR guidelines

This file can be:
- **Copied** to each project (for customization)
- **Symlinked** to a single source (for consistency)

### SKILL.md (Project-Specific Knowledge)

Contains everything an AI agent needs to understand a specific project:

- **Project Overview**: What the project does, production URLs
- **Architecture**: Layer diagram, dependency flow
- **Source Layout**: Directory structure with explanations
- **Key Files**: Table of important files and their purposes
- **Domain Rules**: Business logic, validation rules, conventions
- **API Endpoints**: Available routes and their descriptions
- **Testing**: Commands and patterns
- **Deployment**: How to deploy
- **Common Issues**: Known problems and solutions

## Using Skills with AI Agents

### OpenCode / Claude Code

Skills can be made available globally by symlinking:

```bash
# OpenCode
mkdir -p ~/.config/opencode/skills/my-project
ln -sfn ~/path/to/project/SKILL.md ~/.config/opencode/skills/my-project/SKILL.md

# Claude Code  
mkdir -p ~/.claude/skills/my-project
ln -sfn ~/path/to/project/SKILL.md ~/.claude/skills/my-project/SKILL.md
```

### SKILL.md Format

Skills follow the [Agent Skills](https://agentskills.io) open standard with YAML frontmatter:

```yaml
---
name: my-project
description: What this skill does and when to use it
---

# Project Knowledge

Content here...
```

## Scaffold CLI

```bash
# Usage
./scaffold [OPTIONS] [PROJECT_NAME]

# Options
-d, --dir DIR      Target directory (default: current directory)
-o, --org ORG      GitHub organization (default: auto-detect from git remote)
-s, --skill-only   Only create SKILL.md (use existing AGENTS.md)
-a, --agents-only  Only create AGENTS.md (skip SKILL.md)
-l, --link         Create symlink to global AGENTS.md instead of copying
-h, --help         Show this help message

# Examples
./scaffold                           # Scaffold in current directory
./scaffold -d ../my-project          # Scaffold in another directory
./scaffold --link                    # Use symlinked AGENTS.md
./scaffold --skill-only my-api       # Only create SKILL.md with name "my-api"
```

## Philosophy

### Why Two Files?

1. **Separation of concerns**: Universal conventions vs project-specific knowledge
2. **DRY principle**: AGENTS.md can be shared (symlinked) across projects
3. **Agent efficiency**: Smaller, focused files are easier for agents to process
4. **Maintainability**: Update conventions in one place, update skills per-project

### Design Principles

From the master `AGENTS.md`:

- **Simplicity is king** — the simplest solution that works is the best solution
- **Self-documenting code** — if it needs comments, refactor it
- **Functional over OOP** — pure functions, composition, immutability
- **Commit early, commit often** — small, focused, verified commits

## Examples

See the `examples/` directory for real-world SKILL.md files:

- `trustpilot-ai/` — MCP server for Trustpilot review responses
- `ufarm/` — Farm management platform with DuckDB
- `dotfiles/` — Personal dotfiles with GNU Stow

## Contributing

1. Fork the repository
2. Create a feature branch (`feat/my-feature`)
3. Follow the conventions in `AGENTS.md`
4. Submit a pull request

## License

MIT
