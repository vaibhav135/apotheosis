# Repository instructions for AI agents

This repository is a learner-driven Rust curriculum. The learner writes exercise solutions; an AI scaffolds, tests, reviews, and helps maintain the curriculum.

## Mandatory startup

Before modifying anything:

1. Read `README.md`.
2. Read `docs/philosophy.md`.
3. Read `docs/curriculum.md`.
4. Read `docs/llm-onboarding.md`.
5. When creating, changing, testing, or reviewing an exercise, read `docs/exercise-authoring.md`, the relevant file under `docs/tracks/`, and the complete README, starter code, and tests of every affected exercise.
6. When working on Event Horizon or The New Beginning, read its dedicated document.
7. Produce the comprehension-gate response required by `docs/llm-onboarding.md`.
8. Wait for the learner or evaluator to confirm that the gate passed.

Do not edit files before the gate passes unless the user explicitly asks only to repair the onboarding documentation itself.

## Quick task routing

This table is an index, not a replacement for the mandatory reading order. The linked documents remain canonical.

| Task | Additional source to read |
|---|---|
| Understand educational principles or granularity | `docs/philosophy.md` |
| Inspect levels, curriculum classification, progression, roadmap, or aggregate status | `docs/curriculum.md` |
| Produce or evaluate the comprehension gate | `docs/llm-onboarding.md` |
| Create, change, test, or review an exercise | `docs/exercise-authoring.md`, the relevant `docs/tracks/<track>.md`, and the complete exercise README, starter code, and tests |
| Work on the current Serde track | `docs/tracks/data-and-serde.md` |
| Scaffold a newly approved exercise | `templates/exercise-readme.md` |
| Work on Event Horizon | `docs/event-horizon.md` |
| Work on The New Beginning | `docs/the-new-beginning.md` |
| Inspect prior onboarding evidence | `docs/validation/` |

If a new track is added, its canonical boundary document belongs under `docs/tracks/` and must be linked from the curriculum and human README.

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
- Treat `README.md` as the human landing page; place detailed rules in their canonical documents and link rather than duplicate them.

If repository documents conflict, stop and ask the learner which rule should prevail. Do not silently choose an interpretation.
