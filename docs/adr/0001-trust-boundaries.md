# ADR 0001 — Separate support trust boundaries and bootstrap with synthetic evidence

Status: accepted for **reference bootstrap implementation only** (2026-10-14); production safeguards await owner decisions.

## Context

The owner draft proposes automatic investigation of technical support cases with a trusted environment containing customer data and an investigator environment that sees only mediated pseudonymized evidence. Search/NLP/media/workflow capabilities already have repository owners. Pseudonymization is not legal anonymization, and an after-the-fact scan cannot undo a disclosure to an external model.

## Decision for this bootstrap

- Support Triage owns case intake, adapter configuration, trusted gateway policy, investigation orchestration and review/delivery decisions. It is not a replacement for reusable engines.
- Start with a dependency-free, synthetic-only Rust policy reference. Only exact allowlisted literals and exact values explicitly mapped to tokens may be released; unknown fields are omitted; unapproved values fail closed.
- Check outgoing synthetic bundles against known vault values. Error types must not interpolate input values. Make detection limitations explicit.
- Do not add model API calls, real tenant data, free-text pseudonymization, persistence or network access to this reference.
- In the planned system, every investigator-bound flow—including tool results and errors—must cross the policy boundary **before** model exposure; model output and final delivery require separate controls.
- All proposed remediation scripts remain human-reviewed drafts.

## Consequences and alternatives

A synthetic policy reference provides an independently testable starting point but is **not** a security-tested pseudonymization service, privacy guarantee, real diagnostic investigator or deployment design. It deliberately rejects/omits much of the data a useful investigation will eventually need.

A generic workflow runner was rejected for this bootstrap because support-domain trust boundaries are primary and Workflow Editor need not become the execution authority. Generalized adapters, vault storage, NER and visual workflow authoring require real consumer acceptance and separate decisions.

The first full end-to-end demonstration is tracked in [issue #1](https://github.com/moritzbrantner/support-triage/issues/1).
