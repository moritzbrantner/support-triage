//! Synthetic-only policy boundary reference for Support Triage.
//!
//! This crate does not pseudonymize arbitrary text, protect against contextual
//! re-identification, or provide an agent sandbox. Never use it on real tickets.

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

/// Policies must be defined by the trusted side, never by the investigator.
#[derive(Default)]
pub struct FieldPolicy {
    rules: BTreeMap<String, ReleaseRule>,
}

enum ReleaseRule {
    ApprovedValues(BTreeSet<String>),
    ExactVaultToken,
}

impl FieldPolicy {
    pub fn new() -> Self {
        Self::default()
    }

    /// Only these exact, trusted literals may cross this field boundary.
    pub fn allow_exact(&mut self, field: &str, allowed: &[&str]) {
        self.rules.insert(
            field.to_owned(),
            ReleaseRule::ApprovedValues(allowed.iter().map(|value| (*value).to_owned()).collect()),
        );
    }

    /// The entire field value must be pre-seeded in the vault.
    pub fn tokenize(&mut self, field: &str) {
        self.rules
            .insert(field.to_owned(), ReleaseRule::ExactVaultToken);
    }
}

/// Raw fields remain in the trusted zone. Do not serialize or log this type.
#[derive(Default)]
pub struct ClearCase {
    fields: Vec<ClearField>,
}

struct ClearField {
    name: String,
    value: String,
}

impl ClearCase {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_field(&mut self, name: &str, value: &str) {
        self.fields.push(ClearField {
            name: name.to_owned(),
            value: value.to_owned(),
        });
    }
}

/// Only explicitly allowed literals or entire-field tokens are present here.
#[derive(Debug, PartialEq, Eq)]
pub struct AgentBundle {
    pub fields: BTreeMap<String, String>,
    pub omitted_fields: usize,
}

/// Error variants intentionally contain no caller-supplied text.
#[derive(Debug, PartialEq, Eq)]
pub enum GatewayError {
    InvalidVaultEntry,
    ConflictingVaultEntry,
    DuplicateField,
    UnapprovedValue,
    UnmappedValue,
    KnownValueInEgress,
    UnresolvedTenant,
}

impl fmt::Display for GatewayError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::InvalidVaultEntry => "invalid vault entry",
            Self::ConflictingVaultEntry => "conflicting vault entry",
            Self::DuplicateField => "duplicate input field",
            Self::UnapprovedValue => "field value not approved",
            Self::UnmappedValue => "field value not pre-seeded",
            Self::KnownValueInEgress => "known sensitive value found in output",
            Self::UnresolvedTenant => "unresolved tenant",
        };
        f.write_str(message)
    }
}

impl Error for GatewayError {}

/// Trusted lookup. Does not use a reversible hash or emit source strings in errors.
#[derive(Default)]
pub struct Vault {
    by_clear_value: BTreeMap<String, String>,
}

impl Vault {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, clear_value: &str, token: &str) -> Result<(), GatewayError> {
        if clear_value.is_empty() || token.is_empty() || clear_value == token {
            return Err(GatewayError::InvalidVaultEntry);
        }

        if self
            .by_clear_value
            .iter()
            .any(|(existing_clear, existing_token)| {
                existing_token == token && existing_clear != clear_value
            })
            || self
                .by_clear_value
                .get(clear_value)
                .is_some_and(|previous| previous != token)
        {
            return Err(GatewayError::ConflictingVaultEntry);
        }

        self.by_clear_value
            .insert(clear_value.to_owned(), token.to_owned());
        Ok(())
    }

    /// This test checks only known, pre-seeded clear values; it is not a
    /// comprehensive PII detector or a guarantee of anonymization.
    pub fn check_known_values(&self, candidate: &str) -> Result<(), GatewayError> {
        if self
            .by_clear_value
            .keys()
            .any(|known_clear| candidate.contains(known_clear))
        {
            return Err(GatewayError::KnownValueInEgress);
        }
        Ok(())
    }

    /// A deliberately restrictive reference: unknown fields are dropped,
    /// tokenized fields must match an entire pre-seeded value exactly.
    pub fn release(
        &self,
        policy: &FieldPolicy,
        clear_case: &ClearCase,
    ) -> Result<AgentBundle, GatewayError> {
        let mut seen = BTreeSet::new();
        let mut fields = BTreeMap::new();
        let mut omitted_fields = 0;

        for field in &clear_case.fields {
            if !seen.insert(&field.name) {
                return Err(GatewayError::DuplicateField);
            }

            match policy.rules.get(&field.name) {
                Some(ReleaseRule::ApprovedValues(approved)) => {
                    if !approved.contains(&field.value) {
                        return Err(GatewayError::UnapprovedValue);
                    }
                    fields.insert(field.name.clone(), field.value.clone());
                }
                Some(ReleaseRule::ExactVaultToken) => {
                    let token = self
                        .by_clear_value
                        .get(&field.value)
                        .ok_or(GatewayError::UnmappedValue)?;
                    fields.insert(field.name.clone(), token.clone());
                }
                None => omitted_fields += 1,
            }
        }

        // Independent from the policy lookup, but limited to known values.
        for value in fields.values() {
            self.check_known_values(value)?;
        }

        Ok(AgentBundle {
            fields,
            omitted_fields,
        })
    }
}

/// Opaque synthetic tenant identifier. Resolution of real tenants is out of scope.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TenantId(String);

impl TenantId {
    pub fn new(id: &str) -> Result<Self, GatewayError> {
        let id = id.trim();
        if id.is_empty() {
            return Err(GatewayError::UnresolvedTenant);
        }
        Ok(Self(id.to_owned()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Kind of a mapped entity; determines the typed placeholder prefix.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EntityKind {
    Person,
    Organization,
}

impl EntityKind {
    fn prefix(self) -> &'static str {
        match self {
            Self::Person => "PERSON",
            Self::Organization => "ORG",
        }
    }

    fn owns(self, token: &str) -> bool {
        token
            .strip_prefix(self.prefix())
            .and_then(|rest| rest.strip_prefix('_'))
            // Numbering starts at 1 and is canonical: no zero or zero-padded suffixes.
            .is_some_and(|n| {
                !n.is_empty() && !n.starts_with('0') && n.bytes().all(|b| b.is_ascii_digit())
            })
    }
}

/// Trusted, tenant-isolated vaults (Atlas 2.10): each registered tenant has its own
/// mapping namespace and placeholder numbering. Nothing here consults another
/// tenant's mappings, so no tenant's results reveal another tenant's contents.
#[derive(Default)]
pub struct TenantVaults {
    tenants: BTreeMap<TenantId, Vault>,
}

impl fmt::Debug for TenantVaults {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TenantVaults")
            .field("tenants", &self.tenants.len())
            .finish_non_exhaustive()
    }
}

impl TenantVaults {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, tenant: &TenantId) -> Result<(), GatewayError> {
        self.tenants.entry(tenant.clone()).or_default();
        Ok(())
    }

    fn vault(&self, tenant: &TenantId) -> Result<&Vault, GatewayError> {
        self.tenants
            .get(tenant)
            .ok_or(GatewayError::UnresolvedTenant)
    }

    fn vault_mut(&mut self, tenant: &TenantId) -> Result<&mut Vault, GatewayError> {
        self.tenants
            .get_mut(tenant)
            .ok_or(GatewayError::UnresolvedTenant)
    }

    /// Returns the tenant's existing placeholder for `clear`, or allocates the
    /// lowest-numbered unused `<PREFIX>_<n>` in that tenant. Deterministic.
    pub fn assign(
        &mut self,
        tenant: &TenantId,
        kind: EntityKind,
        clear: &str,
    ) -> Result<String, GatewayError> {
        let vault = self.vault_mut(tenant)?;
        if let Some(existing) = vault.by_clear_value.get(clear) {
            if !kind.owns(existing) {
                return Err(GatewayError::ConflictingVaultEntry);
            }
            return Ok(existing.clone());
        }
        // A placeholder may equal neither an existing token nor a mapped clear value, so a
        // generated token can never be another entity's clear text.
        let used: BTreeSet<&String> = vault
            .by_clear_value
            .values()
            .chain(vault.by_clear_value.keys())
            .collect();
        let token = (1u64..)
            .map(|n| format!("{}_{n}", kind.prefix()))
            .find(|candidate| !used.contains(candidate))
            .expect("an unused placeholder exists");
        vault.insert(clear, &token)?;
        Ok(token)
    }

    pub fn insert(
        &mut self,
        tenant: &TenantId,
        clear: &str,
        token: &str,
    ) -> Result<(), GatewayError> {
        self.vault_mut(tenant)?.insert(clear, token)
    }

    pub fn release(
        &self,
        tenant: &TenantId,
        policy: &FieldPolicy,
        clear_case: &ClearCase,
    ) -> Result<AgentBundle, GatewayError> {
        self.vault(tenant)?.release(policy, clear_case)
    }

    pub fn check_known_values(
        &self,
        tenant: &TenantId,
        candidate: &str,
    ) -> Result<(), GatewayError> {
        self.vault(tenant)?.check_known_values(candidate)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture() -> (Vault, FieldPolicy, ClearCase) {
        let mut vault = Vault::new();
        vault.insert("Alice Example", "PERSON_1").unwrap();
        let mut policy = FieldPolicy::new();
        policy.allow_exact("state", &["open", "closed"]);
        policy.tokenize("reporter");
        let mut clear_case = ClearCase::new();
        clear_case.add_field("state", "open");
        clear_case.add_field("reporter", "Alice Example");
        clear_case.add_field("free_text", "Sensitive free text not vetted");
        (vault, policy, clear_case)
    }

    #[test]
    fn releases_only_approved_literals_and_tokens() {
        let (vault, policy, clear_case) = fixture();
        let bundle = vault.release(&policy, &clear_case).unwrap();

        assert_eq!(bundle.fields.get("state").map(String::as_str), Some("open"));
        assert_eq!(
            bundle.fields.get("reporter").map(String::as_str),
            Some("PERSON_1")
        );
        assert!(!bundle.fields.contains_key("free_text"));
        assert_eq!(bundle.omitted_fields, 1);
    }

    #[test]
    fn unapproved_literals_fail_closed_without_leaking_values() {
        let (vault, policy, _) = fixture();
        let mut clear_case = ClearCase::new();
        clear_case.add_field("state", "unapproved-sensitive-value");
        let err = vault.release(&policy, &clear_case).unwrap_err();
        assert_eq!(err, GatewayError::UnapprovedValue);
        assert!(!err.to_string().contains("unapproved-sensitive-value"));
    }

    #[test]
    fn missing_vault_value_fails_closed() {
        let (_, policy, clear_case) = fixture();
        let err = Vault::new().release(&policy, &clear_case).unwrap_err();
        assert_eq!(err, GatewayError::UnmappedValue);
    }

    #[test]
    fn known_value_egress_detects_raw_value() {
        let (vault, _, _) = fixture();
        assert_eq!(
            vault.check_known_values("case mentions Alice Example"),
            Err(GatewayError::KnownValueInEgress)
        );
        vault.check_known_values("PERSON_1").unwrap();
    }

    #[test]
    fn duplicate_input_field_rejected() {
        let (vault, policy, mut clear_case) = fixture();
        clear_case.add_field("state", "closed");
        assert_eq!(
            vault.release(&policy, &clear_case),
            Err(GatewayError::DuplicateField)
        );
    }

    #[test]
    fn conflicting_tokens_are_rejected() {
        let mut vault = Vault::new();
        vault.insert("Alice Example", "PERSON_1").unwrap();
        vault.insert("Alice Example", "PERSON_1").unwrap();
        assert_eq!(
            vault.insert("Bob Example", "PERSON_1"),
            Err(GatewayError::ConflictingVaultEntry)
        );
        assert_eq!(
            vault.insert("Alice Example", "PERSON_2"),
            Err(GatewayError::ConflictingVaultEntry)
        );
    }

    #[test]
    fn approved_literal_cannot_expose_a_known_vault_value() {
        let (vault, mut policy, _) = fixture();
        policy.allow_exact("state", &["Alice Example"]);
        let mut clear_case = ClearCase::new();
        clear_case.add_field("state", "Alice Example");
        assert_eq!(
            vault.release(&policy, &clear_case),
            Err(GatewayError::KnownValueInEgress)
        );
    }
}
