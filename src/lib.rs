pub mod blockchain;
pub mod cli;
pub mod error;
pub mod vrf;
pub mod args;

// Use the correct bindings
pub use contract_bindings::wyrd as bindings;
// pub use crate as gen_vrf;
