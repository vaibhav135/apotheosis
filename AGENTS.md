# Repository instructions for AI agents

This repository is a learner-driven Rust curriculum. The learner writes exercise solutions; an AI scaffolds, tests, reviews, and helps maintain the curriculum.

## Mandatory startup

Before modifying anything:

1. Read `README.md`.
2. Read `docs/llm-onboarding.md`.
3. Read `docs/exercise-authoring.md` when creating or changing exercises.
4. Read the relevant file under `docs/tracks/`.
5. Read the complete README, starter code, and tests of every exercise being changed.
6. Produce the comprehension-gate response required by `docs/llm-onboarding.md`.
7. Wait for the learner or evaluator to confirm that the gate passed.

Do not edit files before the gate passes unless the user explicitly asks only to repair the onboarding documentation itself.

## Non-negotiable behavior

- Explain your understanding, assumptions, alternatives, and trade-offs before implementation.
- Discuss and approve exercise contracts before creating files.
- Work in small steps and keep the learner informed.
- Scaffold exercises; do not implement learner solutions unless explicitly requested.
- Review and explain before rewriting learner code.
- Validate observable behavior by default; enforce internals only when they are the stated lesson.
- Do not silently add features, requirements, architecture, or unrelated cleanup.
- Prefer authoritative primary sources for technical and scientific claims.
- Follow safety, reproducibility, and external-review requirements for advanced work.
- Update durable documentation with conclusions, not conversation transcripts.

If repository documents conflict, stop and ask the learner which rule should prevail. Do not silently choose an interpretation.
