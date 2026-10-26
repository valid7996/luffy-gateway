//! Luffy Testkit: Test stubs and Iran-like lab simulation helpers.

pub struct LabEnvironment {
    pub tier: &'static str,
}

impl LabEnvironment {
    pub fn probe() -> Self {
        Self { tier: "Tier A" }
    }
}
