# Scaffold CLI

A small Rust CLI that sets up `AGENTS.md` and `SKILL.md` in your project.

---

## Why a CLI?

You could copy the files manually. But scaffold does a few things better:

1. **Auto-detects** project name and org from git remote
2. **Fills in placeholders** in SKILL.md template
3. **Symlinks** AGENTS.md so updates propagate automatically
4. **Prints instructions** for global skill setup

It's 670KB compiled. No dependencies at runtime.

---

## Installation

=== "Build from source"

    ```bash
    git clone https://github.com/Proteusiq/agentic.git
    cd agentic/scaffold
    cargo build --release
    cp target/release/scaffold /usr/local/bin/
    ```

=== "With Cargo"

    ```bash
    # Coming soon
    cargo install scaffold
    ```

---

## Usage

### Basic

```bash
cd ~/code/my-project
scaffold
```

Creates:
- `AGENTS.md` — copied from template
- `SKILL.md` — with project name and org auto-filled

### With Symlink (Recommended)

```bash
scaffold --link
```

Creates:
- `AGENTS.md` → symlinked to `~/.cache/agentic/AGENTS.md`
- `SKILL.md` — copied with placeholders filled

When you update the cached AGENTS.md, all symlinked projects get the update.

### Target Another Directory

```bash
scaffold -d ~/code/other-project
```

### Override Project Name or Org

```bash
scaffold --org mycompany my-custom-name
```

---

## Options

```
scaffold [OPTIONS] [PROJECT_NAME]

Arguments:
  [PROJECT_NAME]    Project name (default: auto-detect from directory or git)

Options:
  -d, --dir <DIR>   Target directory (default: current)
  -o, --org <ORG>   GitHub organization (default: auto-detect from git)
  -l, --link        Symlink AGENTS.md instead of copying
  -s, --skill-only  Only create SKILL.md
  -a, --agents-only Only create AGENTS.md
  -h, --help        Print help
  -V, --version     Print version
```

---

## Examples

### New project, full setup

```bash
mkdir my-api && cd my-api
git init
git remote add origin git@github.com:myorg/my-api.git
scaffold --link
```

Output:
```
Scaffolding agent docs for: myorg/my-api (project: my-api)

Linked AGENTS.md -> "/Users/you/.cache/agentic/AGENTS.md"
Created SKILL.md (edit to add project-specific details)

To make this skill available globally, run:

  # OpenCode
  mkdir -p /Users/you/.config/opencode/skills/my-api
  ln -sfn /Users/you/code/my-api/SKILL.md /Users/you/.config/opencode/skills/my-api/SKILL.md

  # Claude Code
  mkdir -p /Users/you/.claude/skills/my-api
  ln -sfn /Users/you/code/my-api/SKILL.md /Users/you/.claude/skills/my-api/SKILL.md

Done! Edit the files to add project-specific details.
```

### Add SKILL.md to existing project

Your project already has an AGENTS.md:

```bash
scaffold --skill-only
```

### Set up conventions only (no domain knowledge yet)

```bash
scaffold --agents-only --link
```

---

## What Gets Created

### AGENTS.md

If `--link` is used:
```
~/.cache/agentic/AGENTS.md     ← actual file
your-project/AGENTS.md         ← symlink
```

If not:
```
your-project/AGENTS.md         ← copy of template
```

### SKILL.md

Always a copy, with placeholders replaced:

| Placeholder | Replaced With |
|-------------|---------------|
| `{{PROJECT_NAME}}` | Auto-detected or provided |
| `{{GITHUB_ORG}}` | From git remote or `--org` |
| `{{REPO_NAME}}` | From git remote or directory name |
| `{{PROJECT_NAME_UPPER}}` | `MY_PROJECT` format for env vars |

Other placeholders (like `{{SERVICE_NAME}}`) are set to sensible defaults. Edit them.

---

## Security

The CLI validates all inputs:

| Check | Protection Against |
|-------|-------------------|
| No `..` or `/` in names | Path traversal |
| No shell metacharacters | Command injection |
| Cache directory ownership | Symlink attacks |
| Max 64 character names | Buffer overflow |

```bash
# These all fail safely
scaffold "../../../etc"
scaffold "foo;rm -rf /"
scaffold 'foo$(whoami)'
```

---

## Global Skills

After running scaffold, it prints instructions to make the skill available globally:

```bash
# OpenCode
mkdir -p ~/.config/opencode/skills/my-project
ln -sfn /path/to/my-project/SKILL.md ~/.config/opencode/skills/my-project/SKILL.md

# Claude Code
mkdir -p ~/.claude/skills/my-project
ln -sfn /path/to/my-project/SKILL.md ~/.claude/skills/my-project/SKILL.md
```

Now your agent knows about `my-project` even when you're working in a different repo.

---

## Source

The CLI is ~350 lines of Rust with embedded templates:

```rust
const AGENTS_TEMPLATE: &str = include_str!("../../AGENTS.md");
const SKILL_TEMPLATE: &str = include_str!("../../SKILL.template.md");
```

Templates are baked into the binary at compile time. No external files needed.

[View source on GitHub](https://github.com/Proteusiq/agentic/blob/main/scaffold/src/main.rs)
