use jekko_deploy::{identity, validate_identity};

#[test]
fn public_identity_contract_is_stable() {
    validate_identity().expect("identity validates");
    let (repo, role, profile) = identity();
    assert_eq!(repo, "jekko-deploy");
    assert_eq!(role, "deploy");
    assert_eq!(profile, "ops");
}
