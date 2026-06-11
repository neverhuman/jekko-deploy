/// Canonical identity for the jekko-deploy split-family checkout.
pub const REPOSITORY: &str = "jekko-deploy";

/// Role recorded in the split-family manifest.
pub const ROLE: &str = "deploy";

/// Profile recorded in the split-family manifest.
pub const PROFILE: &str = "ops";

/// Return the repo identity tuple used by the smoke tests.
pub fn identity() -> (&'static str, &'static str, &'static str) {
    (REPOSITORY, ROLE, PROFILE)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identity_is_stable() {
        assert_eq!(identity(), (REPOSITORY, ROLE, PROFILE));
    }
}
