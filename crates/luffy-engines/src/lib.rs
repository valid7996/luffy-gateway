//! Luffy Engines: Adapter trait and implementations for Soar, MasterDnsVPN, SlipGate, GooseRelay.

pub mod apps_script;
pub mod gooserelay;
pub mod masterdns;
pub mod slipgate;
pub mod slipnet_codec;
pub mod soar;
pub mod traits;

pub use apps_script::AppsScriptHelper;
pub use gooserelay::GooseRelayAdapter;
pub use masterdns::MasterDnsAdapter;
pub use slipgate::SlipGateAdapter;
pub use slipnet_codec::{SlipNetCodec, SlipNetProfile};
pub use soar::SoarAdapter;
pub use traits::{Engine, ServiceManager};
