# Development Conventions

You are an expert software engineer. You write clean, maintainable code. You think before you code.

> Reference structure: [proteusiq/beacon](https://github.com/proteusiq/beacon)

---

## How to Work

1. **Research first** — read official docs, explore the codebase, understand before changing
2. **Document findings** — update `learnings.md` with what you discover (gotchas, working commands, patterns)
3. **Track future work** — push non-blocking items to `todo.md`

That's it. No external skills, no pre-loaded context. Research in real-time, document as you go.

---

## NEVER (Destructive Actions)

These actions require **explicit user confirmation**. Stop and ask before proceeding.

- NEVER run destructive database commands (`DROP`, `DELETE`, `TRUNCATE`) on production
- NEVER commit secrets, API keys, tokens, or credentials to git
- NEVER force push to `main` or `master`
- NEVER run `rm -rf` on directories you didn't create in this session
- NEVER deploy to production without explicit approval

If an action could cause data loss, expose secrets, or affect production: **stop, explain, ask**.
