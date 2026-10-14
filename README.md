# Support Triage

Privacy-gated investigation of technical support cases. This repository is the **application owner** for ticket intake, investigator access policy, evidence-backed assessments, review, and audit.

**Current status:** experimental bootstrap. Only a deterministic, synthetic-only gateway policy reference is implemented. There is **no** ticket integration, AI model, persistent vault, production pseudonymization, compliance certification, or real-data-safe deployment. Do not pass customer data to this prototype.

## Product goal

A ticket leads to a bounded investigation over selected evidence from code, logs, and read-only data sources. A report gives a classification, suspected cause with resolvable evidence, uncertainty, and next steps. The operator remains responsible for execution and delivery.

## Trust boundaries

- **Trusted/clear zone:** input adapters, tenant resolution and authorization, raw evidence, credentials, policy enforcement, vault, egress checks, re-identification, and delivery.
- **Investigator zone:** a constrained investigator receiving only policy-approved bundles and obtaining further evidence only through trusted gateway tools.
- **No bypass:** the investigator must not have raw datasource credentials, general filesystem/network access to customer data, or an alternative data channel. This is a target deployment property, **not** an implemented guarantee.
- **Pseudonymization is not anonymization.** A seeded-value egress check cannot detect unknown personal data or inference through combinations of otherwise innocuous facts. Actual deployments need threat modeling, layered validation, and independent tests.

## Current executable reference

Requires Rust 1.85 or newer. Uses no third-party runtime dependencies.

```sh
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test --locked
cargo run --locked --example synthetic_case
```

The example demonstrates exact-value allowlisting, exact vault token substitution, unknown-field omission and known-value egress rejection against **synthetic** fields only. It deliberately does not process arbitrary free text, build an MCP server, or investigate tickets.

## Ecosystem responsibilities

- [nlp-stack](https://github.com/moritzbrantner/nlp-stack): linguistic and model-based NER capabilities.
- [search-engine](https://github.com/moritzbrantner/search-engine): reusable search/index mechanics, not tenant authorization or ticket ingestion.
- [media-intelligence](https://github.com/moritzbrantner/media-intelligence): optional multi-modal evidence coordination, not generic policy enforcement.
- [workflow-editor](https://github.com/moritzbrantner/workflow-editor): optional future authoring UI, not a required agent execution engine.

Do not duplicate these capabilities inside this application. Adopt them only when a concrete end-to-end consumer needs them.

## Development

- [Decision Atlas](docs/DECISION-ATLAS.md) records product choices and unresolved questions; unchecked entries are **not** implemented.
- [Architecture decision](docs/adr/0001-trust-boundaries.md) records the synthetic-first bootstrap and its limits.
- [First product slice](https://github.com/moritzbrantner/support-triage/issues/1) defines the path from case intake to inspectable diagnosis and review.

`conventions.json` selects the current `rust` and `security` convention modules. There is no installed `conventions.lock.json` or `.conventions/` snapshot yet; the repository does **not** pin convention versions. Run `coding-tooling conventions update` with the current canonical registry to install managed snapshots rather than manufacturing lockfiles.

As a temporary bootstrap exception to the public-repository Pages expectation, no Pages site is published yet. The first vertical slice will use a real Rust-owned fixture/report path, not a parallel browser policy engine.
