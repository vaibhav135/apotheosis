# <ID>: <Exercise title>

- **ID:** `<TRACK-NUMBER>`
- **Track:** `<canonical track name>`
- **Level:** `Level N — Name`
- **Curriculum:** `Core | Advanced`
- **Participation:** `Required | Optional specialization`
- **Trial:** `None | The Unknown`
- **Field Quest:** `Yes | No`
- **Status:** `Planned | Scaffolded | In Progress | Tests Pass | Reviewed | Mastered`
- **Prerequisites:** `None | <exercise IDs or named gates>`

## Learning objectives

- <transferable objective>
- <transferable objective>

## Scenario

<Why this problem exists in real software or research.>

## Task

<What the learner must build.>

## Public API

```rust
// Approved signatures only. Do not include a completed implementation.
```

## Required behavior

1. <Observable requirement>
2. <Observable requirement>

## Error behavior

- <Stable error category or explicit policy>
- <Panic policy>

## Constraints

- <Only constraints required by the lesson>

## Non-goals

- <Behavior or feature intentionally excluded>

## Examples

```text
<Representative input and output>
```

## Commands

```bash
cargo test --manifest-path exercises/<track>/<exercise>/Cargo.toml
cargo fmt --manifest-path exercises/<track>/<exercise>/Cargo.toml --check
cargo clippy --manifest-path exercises/<track>/<exercise>/Cargo.toml -- -D warnings
```

## Visible-test scope

- <Representative success case>
- <Representative failure case>

## Review-time adversarial scope

- <Boundary already implied by the contract>
- <Malformed or interaction case>

## Progressive hints

<details>
<summary>Hint 1 — Concept</summary>

<Name the concept and give the smallest useful direction.>

</details>

<details>
<summary>Hint 2 — Documentation</summary>

<Link to the exact authoritative documentation section.>

</details>

<details>
<summary>Hint 3 — Decomposition</summary>

<Suggest structure without implementing it.>

</details>

<details>
<summary>Final rescue</summary>

<Structural pseudocode or partial scaffolding, not a finished solution.>

</details>

## Mastery review

### Explain

<What the learner must explain in their own words.>

### Diagnose

<The intentionally broken variation to analyze.>

### Transfer

<A changed requirement to implement after initial review.>

### Limitations

<Boundaries the learner must identify.>

## Primary sources

- `<Source title> — <authoritative URL>`

## Authoring notes

These notes constrain future maintainers and are not an implementation solution.

- **Reserved concepts:** <concepts intentionally left for later exercises>
- **Level rationale:** <why this level is appropriate>
- **Expected failure modes:** <mistakes the tests and review should reveal>
- **Approval record:** <date or decision reference when available>
