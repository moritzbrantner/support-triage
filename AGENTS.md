# Agent instructions

Read `README.md`, `CONTEXT.md`, `docs/DECISION-ATLAS.md`, and `docs/adr/0001-trust-boundaries.md` before changing implementation.

## Authority and boundaries

- This is a **support investigation application**; do not create a generic agent orchestration framework, duplicate search/NLP/media engines, or move canonical support-case ownership into shared libraries.
- The trusted zone owns credentials, raw information, tenant authorization, the vault, release policy, re-identification, and delivery. Investigator outputs are untrusted and require checks before release.
- Do not claim the existing reference gateway is production-safe. Unknown personally identifying information, contextual re-identification, alternate network paths and model-provider handling are **not** solved by an allowlist plus dictionary.
- No production credentials, real support tickets, customer data, persistent plaintext vault, or provider model calls until the threat model, release policy, runtime isolation, and relevant owner decisions are resolved.
- Never execute agent-generated scripts; present inert drafts for explicit human review.
- For a new feature, deliver a bounded end-to-end vertical slice with independently observable behavior and focused adversarial regressions. Prefer a validated Pages evidence/demo artifact only when it genuinely uses the real Rust contracts.

## Conventions

This repo began empty with no installed conventions. `conventions.json` selects current upstream `rust` and `security` modules, but no `conventions.lock.json` or `.conventions/` snapshot is installed; convention versions are not pinned. The canonical discovery URL is https://moritzbrantner.github.io/coding-agent-conventions/agent-tool.json. Resolve the **current** registry, run `coding-tooling conventions update` to install managed material when tooling is available, and honor repository-local exceptions before broader shared rules. Do not fabricate managed snapshots or pin stale policy.

## Verify

```sh
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test --locked
cargo run --locked --example synthetic_case
```

For security-boundary changes, add negative tests for unknown fields, missing identities, unauthorized/cross-tenant evidence, prompt injection, leakage into errors/transcripts, and attempted gateway bypass as applicable. Verify exact-head CI before merge; do not merge this bootstrap or introduce real integrations without an explicit request.
