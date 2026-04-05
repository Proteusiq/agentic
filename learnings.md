# Learnings

Project-specific knowledge. Discovered through research, not pre-loaded.

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
