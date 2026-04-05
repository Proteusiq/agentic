# Let's Stop the Markdown Hell

Maybe you don't need SKILL.md, ARCHITECTURE.md, CONVENTIONS.md, .cursorrules, or external skills.

**Maybe you need one file: [AGENTS.md](https://agents.md/)** - Maybe classical **CONTRIBUTIONS.md** 🤷🏾‍♂️ and it should be minimal.

**Docs:** [proteusiq.github.io/agentic](https://proteusiq.github.io/agentic)

---

## The Problems

### 1. Security

Skills are attack vectors. 12% of ClawHub's registry was [compromised with malware](https://snyk.io/articles/skill-md-shell-access/).

### 2. Context Rot

Pre-loaded context becomes stale. Models already know Python style and TypeScript rules. Your 300-line file conflicts with reality.

### 3. Complexity for No Gain

Context files [reduce task success rates and increase cost by 20%](https://arxiv.org/abs/2602.11988). More context = worse results.

---

## The Fix: RTFM

Read The Ducking Manual. In real-time.

```markdown
# Development Conventions

You are an expert software engineer. You write clean, maintainable code. You think before you code.

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
```

Copy [AGENTS.md](./AGENTS.md) to your project. Done.

---

## License

MIT
