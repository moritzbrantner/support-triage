//! Pre-implementation acceptance for issue #4 (independent acceptance agent/context).
//!
//! Authority: docs/DECISION-ATLAS.md Decision 2.10 (Answers 2.3) — within each
//! tenant the same mapped entity keeps the same typed placeholder across cases;
//! the trusted vault owns mappings; identities and mappings are isolated between
//! tenants without exposing raw values or lookup authority. Also
//! docs/adr/0001-trust-boundaries.md, README.md and AGENTS.md (negative tests for
//! missing identities, cross-tenant evidence and leakage into errors).
//! Retention, rotation, deletion and persistence are out of scope; these tests do
//! not prove production vault security.
//!
//! # Public API contract the implementation must add (src/lib.rs)
//!
//! Existing `Vault`, `FieldPolicy`, `ClearCase`, `AgentBundle` and their tests
//! stay valid and unchanged in behavior.
//!
//! ```text
//! GatewayError::UnresolvedTenant            // new unit variant, Display "unresolved tenant"
//!
//! #[derive(Clone, Debug, PartialEq, Eq)]
//! pub struct TenantId;                      // opaque synthetic tenant identifier
//! TenantId::new(&str) -> Result<TenantId, GatewayError>
//!     // empty or whitespace-only -> Err(UnresolvedTenant)
//!
//! #[derive(Clone, Copy, Debug, PartialEq, Eq)]
//! pub enum EntityKind { Person, Organization }
//!     // typed placeholders: Person -> "PERSON_<n>", Organization -> "ORG_<n>", n >= 1
//!
//! #[derive(Default)] + a Debug impl that prints NO clear values and NO tokens
//! pub struct TenantVaults;                  // one isolated mapping namespace per tenant
//! TenantVaults::new() -> TenantVaults
//! TenantVaults::register(&mut self, &TenantId) -> Result<(), GatewayError>
//! TenantVaults::assign(&mut self, &TenantId, EntityKind, clear: &str)
//!     -> Result<String, GatewayError>
//!     // returns the tenant's existing token for `clear`, else allocates the
//!     // lowest-numbered "<PREFIX>_<n>" (starting at 1) not yet used in that
//!     // tenant. Deterministic: no randomness, no hashing of clear values.
//!     // Numbering is per tenant and independent of every other tenant.
//! TenantVaults::insert(&mut self, &TenantId, clear: &str, token: &str)
//!     -> Result<(), GatewayError>           // same rules as Vault::insert, per tenant
//! TenantVaults::release(&self, &TenantId, &FieldPolicy, &ClearCase)
//!     -> Result<AgentBundle, GatewayError>  // Vault::release semantics, per tenant
//! TenantVaults::check_known_values(&self, &TenantId, candidate: &str)
//!     -> Result<(), GatewayError>           // seeded-value egress check, per tenant
//! ```
//!
//! Every tenant-taking method returns `Err(UnresolvedTenant)` for a tenant that
//! was not registered. Conflicts (same clear value to a different token or kind,
//! or one token for two clear values) inside a tenant are
//! `Err(ConflictingVaultEntry)`. A tenant's results never depend on any other
//! tenant's contents (no cross-tenant oracle).
//!
//! # Observable example contract
//!
//! `cargo run --locked --example tenant_pseudonyms` (examples/tenant_pseudonyms.rs)
//! uses synthetic fixtures: tenant `tenant-a` with work items `WI-101` and `WI-102`
//! whose reporter is `Alice Example`, tenant `tenant-b` with work item `WI-201`
//! whose reporter is also `Alice Example`, and an unregistered tenant `tenant-x`.
//! It prints one line per released work item:
//! `tenant=<id> case=<work item> reporter=<token>`
//! and for `tenant-x` a line containing `tenant=tenant-x` and
//! `error=unresolved tenant`. It never prints the clear value `Alice Example`.

use std::process::Command;

use support_triage::{
    AgentBundle, ClearCase, EntityKind, FieldPolicy, GatewayError, TenantId, TenantVaults,
};

const ALICE: &str = "Alice Example";
const BOB: &str = "Bob Example";
const CONTOSO: &str = "Contoso Synthetic Ltd";
const B_ONLY: &str = "Beatrix Tenantb";

fn tenant(id: &str) -> TenantId {
    TenantId::new(id).expect("valid synthetic tenant id")
}

fn policy() -> FieldPolicy {
    let mut policy = FieldPolicy::new();
    policy.allow_exact("state", &["open", "closed"]);
    policy.tokenize("reporter");
    policy.tokenize("customer");
    policy
}

fn work_item(reporter: &str, state: &str) -> ClearCase {
    let mut case = ClearCase::new();
    case.add_field("state", state);
    case.add_field("reporter", reporter);
    case.add_field("description", &format!("{reporter} cannot sign in"));
    case
}

fn reporter(bundle: &AgentBundle) -> &str {
    bundle.fields.get("reporter").map(String::as_str).unwrap()
}

fn registered(ids: &[&str]) -> TenantVaults {
    let mut vaults = TenantVaults::new();
    for id in ids {
        vaults.register(&tenant(id)).unwrap();
    }
    vaults
}

fn assert_no_leak(err: &GatewayError, secrets: &[&str]) {
    for secret in secrets {
        assert!(
            !err.to_string().contains(secret),
            "Display leaked {secret:?}"
        );
        assert!(
            !format!("{err:?}").contains(secret),
            "Debug leaked {secret:?}"
        );
    }
}

fn is_typed(token: &str, prefix: &str) -> bool {
    token
        .strip_prefix(prefix)
        .and_then(|rest| rest.strip_prefix('_'))
        .is_some_and(|n| !n.is_empty() && n.bytes().all(|b| b.is_ascii_digit()) && n != "0")
}

/// Same clear value in two distinct cases of one tenant -> identical typed token.
#[test]
fn same_entity_gets_same_typed_pseudonym_across_cases_in_a_tenant() {
    let a = tenant("tenant-a");
    let mut vaults = registered(&["tenant-a"]);

    let first = vaults.assign(&a, EntityKind::Person, ALICE).unwrap();
    let again = vaults.assign(&a, EntityKind::Person, ALICE).unwrap();
    assert_eq!(first, "PERSON_1");
    assert_eq!(again, first, "re-assignment must be stable");
    assert_eq!(
        vaults.assign(&a, EntityKind::Person, BOB).unwrap(),
        "PERSON_2"
    );
    assert_eq!(
        vaults
            .assign(&a, EntityKind::Organization, CONTOSO)
            .unwrap(),
        "ORG_1"
    );

    let wi_101 = vaults
        .release(&a, &policy(), &work_item(ALICE, "open"))
        .unwrap();
    let wi_102 = vaults
        .release(&a, &policy(), &work_item(ALICE, "closed"))
        .unwrap();
    assert_eq!(reporter(&wi_101), "PERSON_1");
    assert_eq!(reporter(&wi_102), "PERSON_1");
    // Free-text description is not policy-approved and stays omitted.
    assert_eq!(wi_101.omitted_fields, 1);
    for bundle in [&wi_101, &wi_102] {
        let rendered = format!("{bundle:?}");
        assert!(!rendered.contains(ALICE));
    }
}

/// Assignment is deterministic and does not embed or hash-encode the clear value.
#[test]
fn assignment_is_deterministic_and_typed() {
    let run = || {
        let a = tenant("tenant-a");
        let mut vaults = registered(&["tenant-a"]);
        [ALICE, BOB, ALICE, CONTOSO]
            .iter()
            .enumerate()
            .map(|(i, clear)| {
                let kind = if i == 3 {
                    EntityKind::Organization
                } else {
                    EntityKind::Person
                };
                vaults.assign(&a, kind, clear).unwrap()
            })
            .collect::<Vec<_>>()
    };
    let first = run();
    assert_eq!(first, run(), "fresh vaults must assign identically");
    assert_eq!(first, ["PERSON_1", "PERSON_2", "PERSON_1", "ORG_1"]);
    for token in &first {
        assert!(
            is_typed(token, "PERSON") || is_typed(token, "ORG"),
            "{token}"
        );
        for clear in [ALICE, BOB, CONTOSO, "Alice", "Example", "Contoso"] {
            assert!(!token.contains(clear));
        }
    }
}

/// Allocation never reuses a token that a pre-seeded entry already holds.
#[test]
fn assignment_skips_tokens_already_mapped_in_the_tenant() {
    let a = tenant("tenant-a");
    let mut vaults = registered(&["tenant-a"]);
    vaults.insert(&a, BOB, "PERSON_1").unwrap();
    let alice = vaults.assign(&a, EntityKind::Person, ALICE).unwrap();
    assert!(is_typed(&alice, "PERSON"));
    assert_ne!(alice, "PERSON_1");
    assert_eq!(
        vaults.assign(&a, EntityKind::Person, BOB).unwrap(),
        "PERSON_1"
    );
    let bundle = vaults
        .release(&a, &policy(), &work_item(ALICE, "open"))
        .unwrap();
    assert_eq!(reporter(&bundle), alice);
}

/// Identical clear values in two tenants: separate namespaces, no shared mapping,
/// and tenant B's results are identical whether or not tenant A holds data.
#[test]
fn identical_clear_values_in_two_tenants_do_not_share_mappings() {
    let a = tenant("tenant-a");
    let b = tenant("tenant-b");

    // Tenant B observed alone.
    let mut alone = registered(&["tenant-b"]);
    let b_alone_first = alone.assign(&b, EntityKind::Person, B_ONLY).unwrap();
    let b_alone_alice = alone.assign(&b, EntityKind::Person, ALICE).unwrap();

    // Tenant A populated first, then the same operations for tenant B.
    let mut shared = registered(&["tenant-a", "tenant-b"]);
    let a_alice = shared.assign(&a, EntityKind::Person, ALICE).unwrap();
    shared.assign(&a, EntityKind::Person, BOB).unwrap();
    shared
        .assign(&a, EntityKind::Organization, CONTOSO)
        .unwrap();
    let b_first = shared.assign(&b, EntityKind::Person, B_ONLY).unwrap();
    let b_alice = shared.assign(&b, EntityKind::Person, ALICE).unwrap();

    assert_eq!(a_alice, "PERSON_1");
    // Tenant B numbering is independent: A's allocations are not observable.
    assert_eq!(b_first, b_alone_first);
    assert_eq!(b_alice, b_alone_alice);
    assert_eq!(b_first, "PERSON_1");
    assert_eq!(b_alice, "PERSON_2");
    // Same clear value, different tenant-local placeholder: no reuse of A's mapping.
    assert_ne!(b_alice, a_alice);

    // Each tenant's release resolves only its own mapping.
    let in_a = shared
        .release(&a, &policy(), &work_item(ALICE, "open"))
        .unwrap();
    let in_b = shared
        .release(&b, &policy(), &work_item(ALICE, "open"))
        .unwrap();
    assert_eq!(reporter(&in_a), "PERSON_1");
    assert_eq!(reporter(&in_b), "PERSON_2");
}

/// Tenant B cannot look up or reconstruct tenant A's values or tokens.
#[test]
fn cross_tenant_lookup_and_reconstruction_fail_closed() {
    let a = tenant("tenant-a");
    let b = tenant("tenant-b");
    let mut vaults = registered(&["tenant-a", "tenant-b"]);
    let a_org = vaults
        .assign(&a, EntityKind::Organization, CONTOSO)
        .unwrap();
    vaults.assign(&a, EntityKind::Person, ALICE).unwrap();
    vaults.assign(&b, EntityKind::Person, B_ONLY).unwrap();

    let mut case = ClearCase::new();
    case.add_field("customer", CONTOSO);
    let err = vaults.release(&b, &policy(), &case).unwrap_err();
    assert_eq!(err, GatewayError::UnmappedValue, "A's clear value in B");
    assert_no_leak(&err, &[CONTOSO, a_org.as_str()]);

    let mut case = ClearCase::new();
    case.add_field("customer", &a_org);
    let err = vaults.release(&b, &policy(), &case).unwrap_err();
    assert_eq!(err, GatewayError::UnmappedValue, "A's token in B");
    assert_no_leak(&err, &[CONTOSO, a_org.as_str()]);

    let err = vaults
        .release(&b, &policy(), &work_item(ALICE, "open"))
        .unwrap_err();
    assert_eq!(err, GatewayError::UnmappedValue, "A-only person in B");
    assert_no_leak(&err, &[ALICE, "PERSON_1"]);

    // Tenant A still resolves its own value.
    let mut case = ClearCase::new();
    case.add_field("customer", CONTOSO);
    let bundle = vaults.release(&a, &policy(), &case).unwrap();
    assert_eq!(bundle.fields.get("customer"), Some(&a_org));
}

/// Mappings in another tenant never cause conflicts (no existence oracle).
#[test]
fn other_tenant_mappings_never_conflict() {
    let a = tenant("tenant-a");
    let b = tenant("tenant-b");
    let mut vaults = registered(&["tenant-a", "tenant-b"]);
    vaults.insert(&a, ALICE, "PERSON_1").unwrap();
    vaults.insert(&b, BOB, "PERSON_1").unwrap();
    vaults.insert(&b, ALICE, "PERSON_7").unwrap();
    assert_eq!(
        vaults
            .release(&b, &policy(), &work_item(ALICE, "open"))
            .map(|bundle| reporter(&bundle).to_owned()),
        Ok("PERSON_7".to_owned())
    );
    assert_eq!(
        vaults
            .release(&a, &policy(), &work_item(ALICE, "open"))
            .map(|bundle| reporter(&bundle).to_owned()),
        Ok("PERSON_1".to_owned())
    );
}

/// Unresolved tenants fail explicitly on every operation.
#[test]
fn unresolved_tenant_fails_explicitly() {
    for bad in ["", "   "] {
        assert_eq!(TenantId::new(bad), Err(GatewayError::UnresolvedTenant));
    }
    let a = tenant("tenant-a");
    let unknown = tenant("tenant-x");
    let mut vaults = registered(&["tenant-a"]);
    vaults.assign(&a, EntityKind::Person, ALICE).unwrap();

    let errors = [
        vaults
            .assign(&unknown, EntityKind::Person, ALICE)
            .unwrap_err(),
        vaults.insert(&unknown, ALICE, "PERSON_1").unwrap_err(),
        vaults
            .release(&unknown, &policy(), &work_item(ALICE, "open"))
            .unwrap_err(),
        vaults
            .check_known_values(&unknown, "nothing sensitive")
            .unwrap_err(),
    ];
    for err in &errors {
        assert_eq!(*err, GatewayError::UnresolvedTenant);
        assert_no_leak(err, &[ALICE, "PERSON_1", "tenant-a"]);
    }
    assert_eq!(
        GatewayError::UnresolvedTenant.to_string(),
        "unresolved tenant"
    );
    // The failed calls did not implicitly create the tenant.
    assert_eq!(
        vaults.check_known_values(&unknown, "x"),
        Err(GatewayError::UnresolvedTenant)
    );
}

/// Inconsistent mappings inside a tenant are rejected as conflicts.
#[test]
fn inconsistent_mappings_are_rejected() {
    let a = tenant("tenant-a");
    let mut vaults = registered(&["tenant-a"]);
    assert_eq!(
        vaults.assign(&a, EntityKind::Person, ALICE).unwrap(),
        "PERSON_1"
    );
    vaults.insert(&a, ALICE, "PERSON_1").unwrap(); // idempotent re-insert

    let errors = [
        vaults.insert(&a, ALICE, "PERSON_2").unwrap_err(),
        vaults.insert(&a, BOB, "PERSON_1").unwrap_err(),
        vaults
            .assign(&a, EntityKind::Organization, ALICE)
            .unwrap_err(),
    ];
    for err in &errors {
        assert_eq!(*err, GatewayError::ConflictingVaultEntry);
        assert_no_leak(err, &[ALICE, BOB, "PERSON_1", "PERSON_2"]);
    }
    // Rejected attempts left the mapping untouched.
    let bundle = vaults
        .release(&a, &policy(), &work_item(ALICE, "open"))
        .unwrap();
    assert_eq!(reporter(&bundle), "PERSON_1");
    let err = vaults
        .release(&a, &policy(), &work_item(BOB, "open"))
        .unwrap_err();
    assert_eq!(err, GatewayError::UnmappedValue);
}

/// Debug output of the vault set and of errors never shows clear values or tokens.
#[test]
fn debug_output_never_contains_clear_values_or_tokens() {
    let a = tenant("tenant-a");
    let b = tenant("tenant-b");
    let mut vaults = registered(&["tenant-a", "tenant-b"]);
    let alice_a = vaults.assign(&a, EntityKind::Person, ALICE).unwrap();
    let org_a = vaults
        .assign(&a, EntityKind::Organization, CONTOSO)
        .unwrap();
    vaults.insert(&b, B_ONLY, "PERSON_42").unwrap();

    let rendered = format!("{vaults:?} {vaults:#?}");
    for secret in [
        ALICE,
        CONTOSO,
        B_ONLY,
        alice_a.as_str(),
        org_a.as_str(),
        "PERSON_42",
    ] {
        assert!(
            !rendered.contains(secret),
            "TenantVaults Debug leaked {secret:?}"
        );
    }

    // Tenant B's released bundle contains only B's own placeholder.
    let mut case = ClearCase::new();
    case.add_field("reporter", B_ONLY);
    let bundle = vaults.release(&b, &policy(), &case).unwrap();
    let rendered = format!("{bundle:?}");
    assert!(rendered.contains("PERSON_42"));
    for secret in [ALICE, CONTOSO, B_ONLY, org_a.as_str()] {
        assert!(!rendered.contains(secret), "bundle leaked {secret:?}");
    }
}

/// The seeded-value egress check is per tenant and still finds embedded values.
#[test]
fn egress_check_is_per_tenant_and_catches_embedded_values() {
    let a = tenant("tenant-a");
    let b = tenant("tenant-b");
    let mut vaults = registered(&["tenant-a", "tenant-b"]);
    vaults.assign(&a, EntityKind::Person, ALICE).unwrap();
    vaults.assign(&b, EntityKind::Person, B_ONLY).unwrap();

    let a_text = format!("log: user{ALICE}failed");
    let err = vaults.check_known_values(&a, &a_text).unwrap_err();
    assert_eq!(err, GatewayError::KnownValueInEgress);
    assert_no_leak(&err, &[ALICE, "PERSON_1"]);
    assert_eq!(
        vaults.check_known_values(&b, &format!("ticket by {B_ONLY}.")),
        Err(GatewayError::KnownValueInEgress)
    );
    vaults
        .check_known_values(&a, "PERSON_1 reported it")
        .unwrap();

    // Tenant B's verdict does not depend on tenant A's vault (no oracle).
    let mut b_only = registered(&["tenant-b"]);
    b_only.assign(&b, EntityKind::Person, B_ONLY).unwrap();
    assert_eq!(
        vaults.check_known_values(&b, &a_text),
        b_only.check_known_values(&b, &a_text)
    );

    // An approved literal equal to a tenant's known value is still blocked.
    let mut leaky = policy();
    leaky.allow_exact("state", &[ALICE]);
    let mut case = ClearCase::new();
    case.add_field("state", ALICE);
    assert_eq!(
        vaults.release(&a, &leaky, &case),
        Err(GatewayError::KnownValueInEgress)
    );
}

/// Observable outcome: the example prints per-tenant pseudonyms for the fixtures.
#[test]
fn example_prints_per_tenant_pseudonyms_without_clear_values() {
    let output = Command::new(env!("CARGO"))
        .args([
            "run",
            "--quiet",
            "--locked",
            "--example",
            "tenant_pseudonyms",
        ])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("cargo runs the example");
    assert!(output.status.success(), "example failed: {output:?}");
    let stdout = String::from_utf8(output.stdout).unwrap();
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        !stdout.contains(ALICE) && !stderr.contains(ALICE),
        "clear value printed"
    );

    let token_for = |tenant: &str, case: &str| -> String {
        let prefix = format!("tenant={tenant} case={case} reporter=");
        let line = stdout
            .lines()
            .find(|line| line.trim_start().starts_with(&prefix))
            .unwrap_or_else(|| panic!("missing line {prefix:?} in:\n{stdout}"));
        line.trim_start()[prefix.len()..]
            .split_whitespace()
            .next()
            .unwrap()
            .to_owned()
    };
    let a1 = token_for("tenant-a", "WI-101");
    let a2 = token_for("tenant-a", "WI-102");
    let b1 = token_for("tenant-b", "WI-201");
    assert!(is_typed(&a1, "PERSON"), "{a1}");
    assert_eq!(a1, a2, "same entity, same tenant, same pseudonym");
    assert!(is_typed(&b1, "PERSON"), "{b1}");
    assert!(
        stdout
            .lines()
            .any(|l| l.contains("tenant=tenant-x") && l.contains("error=unresolved tenant")),
        "missing unresolved-tenant line in:\n{stdout}"
    );
}
