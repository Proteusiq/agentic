# Learnings

Project-specific knowledge. Discovered through research, not pre-loaded.

---

## AGENTS.md / Context Files

**Source:** [arXiv:2602.11988](https://arxiv.org/abs/2602.11988) (Feb 2026)

- Context files **reduce task success rates** vs no context
- **+20% inference cost** with no benefit
- LLM-generated context files perform worse than human-written
- Human-written only marginally better than nothing
- Context files are **redundant documentation** — 100% duplicate existing docs
- Repository overviews in context files don't help agents find files faster
- Agents follow instructions but the extra work hurts outcomes

**Conclusion:** Minimal requirements only. No pre-loaded opinions.

---

## Skills Security

**Source:** [Snyk Research](https://snyk.io/articles/skill-md-shell-access/) (Jan 2026)

- 12% of ClawHub skill registry compromised (341 malicious skills)
- Attack vector: "Prerequisites" section with install instructions
- Targets: API keys, SSH keys, browser passwords, agent memory files
- Memory poisoning enables persistent behavioral changes

**Conclusion:** Don't load external skills. No attack surface.

---

## Python Packaging

**Source:** [packaging.python.org](https://packaging.python.org/en/latest/tutorials/packaging-projects/)

- Use `pyproject.toml` (PEP 517/518) — not `setup.py`
- Build with `python -m build`
- Upload with `twine upload dist/*`
- Test on TestPyPI first: `twine upload --repository testpypi dist/*`
- Modern tools: `uv`, `hatch`, `flit`, `pdm`

---

## Reference Structure

**Source:** [proteusiq/beacon](https://github.com/proteusiq/beacon)

Follow this repo's patterns for project structure.

---

## Before Release: Security Scan

Before any release, take the role of a security red team:

1. **Find** — scan for vulnerabilities, try to break it, exploit weaknesses
2. **Patch** — fix what you find
3. **Repeat** — find again, patch again, until secure

Attack vectors to check:
- Injection (SQL, command, template)
- Authentication/authorization bypasses
- Secrets exposure (logs, errors, responses)
- Dependency vulnerabilities (`uv audit`, `npm audit`, `cargo audit`)
- Input validation gaps
- Privilege escalation paths

**Conclusion:** Only release when you can't break it.

---

## Quick Reference

| Lang | Format | Lint | Type Check | Test |
|------|--------|------|------------|------|
| Python | `ruff format .` | `ruff check --fix .` | `ty check .` | `pytest` |
| Rust | `cargo fmt` | `cargo clippy` | (built-in) | `cargo test` |
| TS (Bun) | `biome format --write .` | `biome check --fix .` | `tsc --noEmit` | `bun test` |
| TS (Deno) | `deno fmt` | `deno lint` | `deno check .` | `deno test` |
