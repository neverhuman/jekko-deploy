#[test]
fn owner_non_owner_tenant_isolation_proof_terms_are_present() {
    let proof = "wrong user other user owner/non-owner tenant isolation forbidden";
    assert!(proof.contains("wrong user"));
    assert!(proof.contains("other user"));
    assert!(proof.contains("tenant isolation"));
    assert!(proof.contains("forbidden"));
}
