# Workflow: how to work with the agent

## One-time setup (you)
1. Install Rust (rustup), git, and create a GitHub repo.
2. Copy this kit into the repo root (`AGENTS.md` at the root, everything else in `docs/`). Commit.
3. Open your agent in the repo folder. (OpenAI Codex reads `AGENTS.md` automatically. If you use another agent, copy `AGENTS.md` to the filename it expects, for example `CLAUDE.md` for Claude Code.)
4. Paste the kickoff prompt below as the first message.

## Kickoff prompt (first message)
```
Read AGENTS.md first, then docs/CONVENTIONS.md, docs/DECISIONS.md, docs/TASKS.md,
docs/ROADMAP.md and docs/VISION.md. Do not write any code yet.

Reply with:
1. A summary of the project in your own words (10 lines max).
2. Any ambiguities, contradictions, or missing information you found in these documents.
3. Questions you need answered before starting Task T0.
4. Your plan for Task T0.

Do not assume anything. Wait for my answers before coding.
```

## Per-task prompt
```
Do Task <ID> from docs/TASKS.md and nothing else. Follow AGENTS.md and docs/CONVENTIONS.md.
Write the tests first. If anything is unclear or a decision is still OPEN, stop and ask.
When finished, report in the format defined in AGENTS.md.
```

## Review prompt (use a fresh session or a second agent)
```
You are a reviewer, not an author. Do not modify any files.
Review the most recent commit against AGENTS.md and docs/CONVENTIONS.md.
Look specifically for: sign errors, frame mix-ups, unit mistakes, numbers without a cited
source, tests that cannot fail, determinism hazards, new dependencies, scope creep,
and claims of accuracy without comparison data.
Output a list with severity and file:line. List separately anything you could not verify.
```

## Per-task loop
1. Give the agent the per-task prompt.
2. Read its report. Ask about anything you do not understand; add it to docs/LEARNING.md.
3. Run the three quality gates yourself.
4. Run the review prompt in a fresh session. Fix real findings.
5. Commit with a clear message. Mark the task done in docs/TASKS.md.

## Rules of thumb
- If the agent proposes a decision listed as OPEN in docs/DECISIONS.md, you decide, not the agent.
- If a test fails, understand why before changing anything. Do not let the agent weaken a test silently.
- Never accept a number without a source.
- Start sessions fresh when context gets long; the docs carry the memory.
- Commit often, in small pieces.
