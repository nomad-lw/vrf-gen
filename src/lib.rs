// src/lib.rs
pub mod blockchain;
pub mod cli;
pub mod error;
pub mod vrf;

// Use the correct bindings
pub use iwyrd_bindings::wyrd as iwyrd_bindings;
