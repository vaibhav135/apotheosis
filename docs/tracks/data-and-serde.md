# Track: Reliable data boundaries with `serde`

This is the first core track. It assumes Rust fundamentals but no substantial prior experience with Serde.

The track teaches Serde through challenging, practical boundaries. It begins with derives and a custom domain type, then progresses through configuration, API modeling, legacy normalization, streaming, and schema evolution.

## Track philosophy

Serialization is not merely converting bytes into structs. It is the boundary between untrusted wire data and trusted domain meaning.

The progression should establish this model:

```text
bytes or text
    ↓
syntax and structural decoding
    ↓
wire-format representation
    ↓
semantic validation and normalization
    ↓
trusted domain representation
```

Each exercise should make one part of this boundary more explicit without prematurely introducing every advanced Serde mechanism.

## Prerequisites

The learner should already understand:

- Structs and enums
- `impl` blocks and traits
- Ownership, borrowing, and basic lifetimes
- `Option` and `Result`
- Pattern matching
- Modules and visibility
- Cargo and ordinary Rust tests

No previous Serde experience is required.

## Track-wide dependencies

The initial exercises use:

- `serde` with its `derive` feature
- `serde_json`
- The Rust standard library

Additional format crates or testing libraries require discussion before being added. JSON remains the primary format so difficulty comes from data-boundary design rather than repeatedly learning format-specific syntax.

## Provisional roadmap

Only SERDE-01 has an approved complete contract. Later entries are planning hypotheses. They may be merged, moved into integration-focused tracks, split, reordered, or removed after reviewing learner progress and applying the repository's granularity rubric.

The table does not require six consecutive Serde exercises.

| ID | Difficulty | Exercise | Planning state | Prerequisites |
|---|---|---|---|---|
| SERDE-01 | Level 1 — First Light | Job Manifest Codec | Scaffolded | Rust fundamentals |
| SERDE-02 | Level 1 — First Light | Reliable Configuration | Provisional | SERDE-01 reviewed |
| SERDE-03 | Level 1 — First Light | API Events | Provisional | SERDE-02 reviewed |
| SERDE-04 | Level 2 — Ascent | Legacy Data Normalizer | Provisional | SERDE-01 through SERDE-03 mastered |
| SERDE-05 | Level 2 — Ascent | Streaming Records | Provisional | SERDE-04 reviewed; standard I/O basics |
| SERDE-06 | Level 3 — Crucible | Versioned Protocol | Provisional | SERDE-04 and SERDE-05 mastered |

None of the initial six is The Unknown. The learner is encountering Serde for the first time, so this track teaches through difficult progressive scaffolding before independent-discovery trials appear elsewhere.

## Concept ownership

This table prevents one exercise from silently stealing the primary lesson of another.

| Concept | Primary exercise | Earlier incidental use allowed? |
|---|---|---|
| Derived outer type invoking custom inner traits | SERDE-01 | No earlier exercise |
| Manual `Serialize` and `Deserialize` for a domain newtype | SERDE-01 | No |
| Renaming, field defaults, and unknown-field rejection | SERDE-02 | Basic derives only in SERDE-01 |
| Structural decoding versus semantic validation | SERDE-02 | Introduced lightly by `JobId` validation in SERDE-01 |
| Tagged enums and nested API payloads | SERDE-03 | No |
| Deliberate missing-versus-`null` semantics | SERDE-03 | Ordinary `Option` may appear earlier, but nuanced behavior is reserved |
| Untagged alternatives and inconsistent legacy representations | SERDE-04 | No |
| Wire types converted into clean domain types | SERDE-04 | A simple newtype boundary appears in SERDE-01 |
| Incremental NDJSON processing and per-record errors | SERDE-05 | No |
| Schema versions, migrations, and compatibility architecture | SERDE-06 | No |
| Borrowed deserialization and `Cow<'de, str>` | Later parsing/lifetimes track | No |
| Implementing a complete Serde data format | Later Level 4 work | No |
| Async I/O | Async/networking track | No; SERDE-05 uses synchronous `BufRead` |

## SERDE-01: Job Manifest Codec

### Purpose

Introduce ordinary derives and manual trait implementations through a validated `JobId(u32)` whose canonical JSON representation is `job_` plus eight lowercase hexadecimal digits.

### Required lesson

- Domain and wire representations may differ.
- Derived outer implementations call custom inner implementations.
- Deserialization can enforce an invariant without manually parsing the surrounding JSON.
- Invalid external data returns errors rather than panicking.

### Boundaries

- Unknown manifest fields retain Serde's default behavior.
- No semantic validation beyond `JobId` format.
- No borrowed deserialization or custom visitor requirement.
- No legacy aliases in the initial contract.

### Mastery variation

Accept an uppercase legacy ID while continuing to serialize only canonical lowercase IDs, then explain why accepted input and generated output may intentionally differ.

The approved complete contract is in the [exercise README](../../exercises/data-and-serde/01-job-manifest-codec/README.md).

## SERDE-02: Reliable Configuration

### Purpose

Teach declarative Serde attributes and the boundary between structural decoding and semantic validation.

### Required concepts

- Field or container renaming
- A specific default for an absent field
- An optional field
- Rejection of unknown fields
- Validation after successful structural decoding
- Structured distinction between decoding and domain errors

### Reserved boundaries

- Nuanced missing-versus-explicit-`null` behavior remains primarily SERDE-03.
- Custom visitors and untagged legacy alternatives remain SERDE-04.
- Multiple configuration formats, environment merging, and live file watching are out of scope.
- URL parsing and unrelated domain validation should not be added unless explicitly approved.

### Decisions still requiring learner approval

- Exact scenario and field schema
- Default values and validation ranges
- Public domain API and error variants
- Serialization being required or explicitly out of scope
- Duplicate-field, whitespace, and casing policies
- Mastery variation

A future LLM must propose these choices with trade-offs rather than silently adopting a plausible schema.

## SERDE-03: API Events

### Purpose

Model nested external payloads with tagged enums and deliberate absence semantics.

### Required concepts

- Nested structures and collections
- One explicitly selected Serde enum-tagging representation
- Known event variants with variant-specific payloads
- Wire names differing from Rust names
- Deliberate distinction among missing, `null`, and present values where the scenario requires it
- Serialization and round-trip behavior where semantically valid

### Reserved boundaries

- Preserving arbitrary unknown event payloads is a stretch goal unless approved as part of the contract.
- Legacy number-or-string fields remain SERDE-04.
- Streaming transport remains SERDE-05.
- Network requests and HTTP clients belong in another track.

### Decisions still requiring learner approval

- Event domain and tag representation
- Unknown-variant behavior
- Which values distinguish missing from `null`
- Round-trip and canonicalization requirements
- Error API and mastery variation

## SERDE-04: Legacy Data Normalizer

### Purpose

Accept inconsistent historical representations and normalize them into one trusted domain model.

### Required concepts

- Untagged helper representations or focused custom field deserialization
- Multiple accepted wire forms
- Explicit rejection of ambiguous or lossy forms
- Conversion from private wire types into public domain types
- Typed decoding and validation failures
- Canonical output that need not mirror every accepted input

### Reserved boundaries

- Do not require a fully manual Serde visitor unless separately approved.
- Do not introduce streaming, async I/O, schema-version architecture, or borrowed data.
- Legacy compatibility must be explicit and bounded; “accept anything reasonable” is forbidden.

### Decisions still requiring learner approval

- Legacy representations and canonical form
- Ambiguity and overflow policies
- Error categories
- Whether canonical serialization is required
- Mastery variation

## SERDE-05: Streaming Records

### Purpose

Process newline-delimited JSON incrementally while preserving deterministic, line-aware error behavior and bounded memory.

### Required concepts

- Synchronous `BufRead`
- Incremental NDJSON processing
- Per-record deserialization
- A documented continue-or-stop error policy
- Line-aware failures
- Stateful aggregation
- Memory use bounded independently of total input size, except for explicitly bounded output

### Reserved boundaries

- Async I/O remains in the async track.
- Zero-copy borrowed deserialization remains in the parsing/lifetimes track.
- Parallel processing and ordering constraints remain later concurrency work.
- Do not benchmark wall-clock performance as a proxy for bounded memory.

### Decisions still requiring learner approval

- Event record and report schema
- Error continuation policy
- Empty-line and trailing-newline behavior
- How bounded memory is validated
- Stable report ordering
- Mastery variation

## SERDE-06: Versioned Protocol

### Purpose

Design a maintainable boundary for schema evolution and migration into one current domain model.

### Required concepts

- Explicit version detection
- Version-specific private wire representations
- Migration and semantic validation
- One current public domain representation
- Compatibility and canonical-output policy
- Public error and API design
- A documented path for adding another version

### Reserved boundaries

- No live networking or distributed protocol negotiation.
- No custom binary format.
- No zero-copy requirement.
- Performance constraints require a separately approved learning reason.

### Design artifact

As Level 3 work, the learner creates `learner/design.md` explaining boundaries, alternatives, compatibility promises, and how a future version would be added.

### Decisions still requiring learner approval

- Protocol domain and supported versions
- Version-discriminator format
- Migration rules and lossy-data policy
- Canonical output version
- Public API and errors
- Adversarial compatibility tests
- Mastery variation

## Error policy across the track

- Do not panic on invalid external data.
- Do not test complete `serde_json::Error` wording.
- Use ordinary Serde errors when callers do not need stable categories.
- Introduce structured domain errors when the exercise teaches the distinction between decoding and validation.
- Error paths should identify useful context such as a field, record, line, or protocol version when the contract requires it.
- Exact categories must be approved per exercise.

## Test progression

Each exercise begins with a small `tests/behavior.rs` demonstrating representative success and failure behavior.

After the learner's implementation is reviewed, `tests/adversarial.rs` may add contract-preserving checks for:

- Boundary values
- Wrong JSON types
- Malformed and Unicode input
- Missing, duplicate, or unknown data according to the contract
- Overflow and canonicalization
- Panic resistance
- Streaming and compatibility interactions

Adversarial tests must not claim concepts reserved for later exercises.

## Track exit gates

### Minimum mastery

- Master SERDE-01 through SERDE-03.
- Explain derives, custom type implementations, common attributes, tagged enums, and absence semantics.
- Diagnose a malformed boundary implementation.
- Adapt one accepted wire format without weakening domain invariants.

This is sufficient for ordinary typed JSON work but not complex ingestion or compatibility design.

### Recommended mastery

- Complete the approved exercises and mastery variations needed to demonstrate the outcomes below; do not use a fixed exercise count as the gate.
- Design explicit wire/domain boundaries.
- Normalize legacy data without accidental ambiguity.
- Process NDJSON with bounded memory and clear error policy.
- Evolve versioned schemas through maintainable migrations.

This is the expected prerequisite for later API, parsing, async, and advanced-campaign work.

### Research-depth mastery

- Complete all ordinary mastery variations.
- Compare at least two valid designs for one custom deserialization problem.
- Build property tests for a canonicalization or migration invariant.
- Explain where allocation occurs and which later lifetime concepts would be needed to reduce it safely.

Implementing a complete Serde data format and zero-copy borrowed decoding remain separate later exercises rather than hidden requirements for this gate.

## Primary sources

- [Serde overview](https://serde.rs/)
- [Using derive](https://serde.rs/derive.html)
- [Attributes](https://serde.rs/attributes.html)
- [Custom serialization](https://serde.rs/custom-serialization.html)
- [Implementing `Serialize`](https://serde.rs/impl-serialize.html)
- [Implementing `Deserialize`](https://serde.rs/impl-deserialize.html)
- [`serde_json`](https://docs.rs/serde_json/)
- [`std::io::BufRead`](https://doc.rust-lang.org/std/io/trait.BufRead.html)

Exercise READMEs should cite the exact subset they rely on.

## Current next action

The learner should implement SERDE-01 from its approved scaffold. Do not create SERDE-02 until SERDE-01 has been reviewed and its track status updated, unless the learner explicitly changes direction.
