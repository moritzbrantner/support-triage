//! Synthetic demonstration of tenant-stable pseudonyms (Atlas 2.10, issue #4).
//! Never use on real tickets: all values here are invented fixtures.

use support_triage::{ClearCase, EntityKind, FieldPolicy, TenantId, TenantVaults};

fn main() {
    let mut policy = FieldPolicy::new();
    policy.allow_exact("state", &["open", "closed"]);
    policy.tokenize("reporter");

    let mut vaults = TenantVaults::new();
    for id in ["tenant-a", "tenant-b"] {
        let tenant = TenantId::new(id).expect("synthetic tenant id");
        vaults.register(&tenant).expect("register tenant");
        vaults
            .assign(&tenant, EntityKind::Person, "Alice Example")
            .expect("assign synthetic reporter");
    }

    let work_items = [
        ("tenant-a", "WI-101", "open"),
        ("tenant-a", "WI-102", "closed"),
        ("tenant-b", "WI-201", "open"),
        ("tenant-x", "WI-901", "open"),
    ];
    for (tenant, work_item, state) in work_items {
        let tenant_id = TenantId::new(tenant).expect("synthetic tenant id");
        let mut case = ClearCase::new();
        case.add_field("state", state);
        case.add_field("reporter", "Alice Example");
        match vaults.release(&tenant_id, &policy, &case) {
            Ok(bundle) => println!(
                "tenant={tenant} case={work_item} reporter={}",
                bundle.fields["reporter"]
            ),
            Err(error) => println!("tenant={tenant} case={work_item} error={error}"),
        }
    }
    println!("Synthetic only: this is NOT a customer-data-safe pseudonymizer.");
}
