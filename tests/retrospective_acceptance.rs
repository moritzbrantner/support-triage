//! Retrospective acceptance (independent agent, PR #2) — not pre-implementation authorship.
//!
//! Obligations derived from README.md, CONTEXT.md, AGENTS.md and
//! docs/adr/0001-trust-boundaries.md for the synthetic-only gateway reference:
//! exact-literal allowlisting, whole-field vault tokens, unknown-field omission,
//! fail-closed unapproved values, errors that do not interpolate input values,
//! and a seeded known-value egress check. Tenant-stable/tenant-isolated
//! pseudonyms (Decision 2.10) are unchecked in the Atlas and not claimed for
//! this reference, so they are intentionally not asserted here.

use support_triage::{ClearCase, FieldPolicy, GatewayError, Vault};

const SECRET: &str = "Zelda Synthetic";
const UNKNOWN_RAW: &str = "synthetic-unknown-raw-value-7781";

fn vault() -> Vault {
    let mut vault = Vault::new();
    vault.insert(SECRET, "PERSON_9").unwrap();
    vault
}

fn policy() -> FieldPolicy {
    let mut policy = FieldPolicy::new();
    policy.allow_exact("state", &["open", "closed"]);
    policy.tokenize("reporter");
    policy
}

fn single(name: &str, value: &str) -> ClearCase {
    let mut case = ClearCase::new();
    case.add_field(name, value);
    case
}

fn assert_no_leak(err: &GatewayError, raw: &str) {
    assert!(!err.to_string().contains(raw), "Display leaked input");
    assert!(!format!("{err:?}").contains(raw), "Debug leaked input");
}

#[test]
fn allowlist_matches_exact_values_only() {
    let vault = vault();
    let policy = policy();
    for variant in [
        "Open", "OPEN", " open", "open ", "ope", "opened", "open\n", "",
    ] {
        let err = vault
            .release(&policy, &single("state", variant))
            .unwrap_err();
        assert_eq!(err, GatewayError::UnapprovedValue, "variant {variant:?}");
    }
    let bundle = vault.release(&policy, &single("state", "closed")).unwrap();
    assert_eq!(
        bundle.fields.get("state").map(String::as_str),
        Some("closed")
    );
}

#[test]
fn tokenized_fields_require_whole_field_match() {
    let vault = vault();
    let policy = policy();
    for variant in [
        "Zelda Synthetic ",
        " Zelda Synthetic",
        "zelda synthetic",
        "Zelda",
        "Contact Zelda Synthetic today",
    ] {
        let err = vault
            .release(&policy, &single("reporter", variant))
            .unwrap_err();
        assert_eq!(err, GatewayError::UnmappedValue, "variant {variant:?}");
        assert_no_leak(&err, variant.trim());
    }
    let bundle = vault.release(&policy, &single("reporter", SECRET)).unwrap();
    assert_eq!(
        bundle.fields.get("reporter").map(String::as_str),
        Some("PERSON_9")
    );
    assert!(!format!("{bundle:?}").contains(SECRET));
}

#[test]
fn unknown_fields_are_omitted_without_their_values() {
    let vault = vault();
    let policy = policy();
    let mut case = ClearCase::new();
    case.add_field("state", "open");
    case.add_field("notes", UNKNOWN_RAW);
    case.add_field("description", &format!("{SECRET} cannot sign in"));
    let bundle = vault.release(&policy, &case).unwrap();
    assert_eq!(bundle.omitted_fields, 2);
    assert_eq!(bundle.fields.len(), 1);
    let rendered = format!("{bundle:?}");
    assert!(!rendered.contains(UNKNOWN_RAW));
    assert!(!rendered.contains(SECRET));
    assert!(!rendered.contains("notes"));
}

#[test]
fn fail_closed_releases_nothing_when_any_field_is_unapproved() {
    let vault = vault();
    let policy = policy();
    let mut case = ClearCase::new();
    case.add_field("reporter", SECRET);
    case.add_field("state", UNKNOWN_RAW);
    let err = vault.release(&policy, &case).unwrap_err();
    assert_eq!(err, GatewayError::UnapprovedValue);
    assert_no_leak(&err, UNKNOWN_RAW);
    assert_no_leak(&err, SECRET);
}

#[test]
fn every_error_variant_is_free_of_input_values() {
    let mut vault = vault();
    let errors = [
        vault.insert("", "PERSON_X").unwrap_err(),
        vault.insert(SECRET, SECRET).unwrap_err(),
        vault.insert(SECRET, "PERSON_OTHER").unwrap_err(),
        vault.insert(UNKNOWN_RAW, "PERSON_9").unwrap_err(),
        vault
            .check_known_values(&format!("log line with {SECRET}"))
            .unwrap_err(),
        vault
            .release(&policy(), &single("reporter", UNKNOWN_RAW))
            .unwrap_err(),
    ];
    for err in &errors {
        assert_no_leak(err, SECRET);
        assert_no_leak(err, UNKNOWN_RAW);
        assert_no_leak(err, "PERSON_OTHER");
    }
}

#[test]
fn seeded_value_egress_check_finds_embedded_values_only() {
    let vault = vault();
    assert_eq!(
        vault.check_known_values(&format!("prefix{SECRET}suffix")),
        Err(GatewayError::KnownValueInEgress)
    );
    // Documented limitation: unseeded personal data is not detected.
    vault
        .check_known_values("Unseeded Person reported PERSON_9")
        .unwrap();
}
