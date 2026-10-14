use support_triage::{ClearCase, FieldPolicy, Vault};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut vault = Vault::new();
    vault.insert("Alice Example", "PERSON_1")?;

    let mut policy = FieldPolicy::new();
    policy.allow_exact("state", &["open", "closed"]);
    policy.tokenize("reporter");

    let mut clear_case = ClearCase::new();
    clear_case.add_field("state", "open");
    clear_case.add_field("reporter", "Alice Example");
    clear_case.add_field("description", "Alice Example cannot sign in");

    let bundle = vault.release(&policy, &clear_case)?;
    println!("Synthetic policy gateway: {bundle:#?}");
    println!("This is NOT a customer-data-safe pseudonymizer or ticket investigator.");
    Ok(())
}
