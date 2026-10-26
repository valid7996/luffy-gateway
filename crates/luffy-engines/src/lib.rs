//! Luffy Engines: Adapter trait and implementations for Soar, MasterDnsVPN, SlipGate, GooseRelay.

pub mod masterdns;
pub mod soar;
pub mod traits;

pub use masterdns::MasterDnsAdapter;
pub use soar::SoarAdapter;
pub use traits::{Engine, ServiceManager};
