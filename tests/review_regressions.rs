//! Review regressions for PR #9 (tenant pseudonyms).

use support_triage::{EntityKind, GatewayError, TenantId, TenantVaults};

fn tenant_a() -> (TenantId, TenantVaults) {
    let tenant = TenantId::new("tenant-a").unwrap();
    let mut vaults = TenantVaults::new();
    vaults.register(&tenant).unwrap();
    (tenant, vaults)
}

/// A generated placeholder must never equal a clear value the tenant already maps.
#[test]
fn generated_tokens_never_equal_a_mapped_clear_value() {
    let (a, mut vaults) = tenant_a();
    vaults.insert(&a, "PERSON_1", "PERSON_9").unwrap();
    let token = vaults
        .assign(&a, EntityKind::Person, "Alice Example")
        .unwrap();
    assert_ne!(token, "PERSON_1");
    assert_ne!(token, "PERSON_9");
    vaults.check_known_values(&a, &token).unwrap();
}

/// Numbering starts at 1: a pre-seeded zero or zero-padded placeholder is not a typed token.
#[test]
fn zero_valued_placeholders_are_not_typed_tokens() {
    for seeded in ["PERSON_0", "PERSON_01"] {
        let (a, mut vaults) = tenant_a();
        vaults.insert(&a, "Alice Example", seeded).unwrap();
        assert_eq!(
            vaults.assign(&a, EntityKind::Person, "Alice Example"),
            Err(GatewayError::ConflictingVaultEntry)
        );
    }
}
