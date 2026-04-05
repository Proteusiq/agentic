# Research

What the studies say about AGENTS.md and context files.

---

## The Conflicting Findings

Two studies, two different conclusions:

### Study 1: Bloated Context Hurts

**[arXiv:2602.11988](https://arxiv.org/abs/2602.11988)** — *Evaluating AGENTS.md: Are Repository-Level Context Files Helpful for Coding Agents?*

Gloaguen et al. (Feb 2026)

> "Context files tend to **reduce task success rates** compared to providing no repository context, while also **increasing inference cost by over 20%**."

Key findings:
- LLM-generated context files perform worse than no context
- Human-written only marginally better
- Agents follow instructions but extra work hurts outcomes
- Unnecessary requirements make tasks harder

**Conclusion:** "Human-written context files should describe only **minimal requirements**."

---

### Study 2: Minimal Context Helps

**[arXiv:2601.20404](https://arxiv.org/abs/2601.20404)** — *On the Impact of AGENTS.md Files on the Efficiency of AI Coding Agents*

Lulla et al. (Jan 2026)

> "The presence of AGENTS.md is associated with a **lower median runtime (Δ 28.64%)** and **reduced output token consumption (Δ 16.58%)**, while maintaining comparable task completion behavior."

Key findings:
- AGENTS.md reduces runtime
- Reduces token usage
- Maintains task completion
- Helps efficiency

**Conclusion:** AGENTS.md files help when configured properly.

---

## The Reconciliation

The studies don't conflict — they measure different things:

| Aspect | Study 1 (Hurts) | Study 2 (Helps) |
|--------|-----------------|-----------------|
| Measures | Task success rate | Runtime & token usage |
| Context type | Bloated, LLM-generated | Repository-configured |
| Problem | Unnecessary requirements | N/A |

**The pattern:**
- **Bloated** context files hurt task success
- **Minimal** AGENTS.md helps efficiency

Both papers agree: **minimal is better**.

---

## What This Means

### Don't

- Generate comprehensive context files with LLMs
- Include everything the model "might need"
- Copy generic best practices
- Pre-load knowledge the model already has

### Do

- Keep AGENTS.md minimal
- Include only project-specific rules
- Focus on workflow and guardrails
- Let the model research the rest

---

## Our Approach

Based on this research, we use:

**AGENTS.md** — Minimal rules. Workflow + NEVER rules. Nothing the LLM already knows.

**learnings.md** — LLM-discovered knowledge. Grows organically. Project-specific.

**todo.md** — Working memory. Ephemeral.

This gives us:
- Efficiency gains from minimal AGENTS.md (Study 2)
- Avoids task success reduction from bloat (Study 1)
- Dynamic knowledge via learnings.md (not measured, but logically follows)

---

## References

- [arXiv:2602.11988](https://arxiv.org/abs/2602.11988) — Gloaguen et al., "Evaluating AGENTS.md"
- [arXiv:2601.20404](https://arxiv.org/abs/2601.20404) — Lulla et al., "On the Impact of AGENTS.md Files"
