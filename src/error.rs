use eyre::{Result, eyre};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum VrfGenError {
    #[error("VRF error: {0}")]
    VrfError(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Blockchain error: {0}")]
    BlockchainError(String),
}

// Instead of accepting a string reference, we'll use String to avoid lifetime issues
pub fn wrap_vrf_err<E: std::fmt::Display>(err: E, context: impl Into<String>) -> eyre::Report {
    eyre!(VrfGenError::VrfError(err.to_string())).wrap_err(context.into())
}

pub fn wrap_input_err<E: std::fmt::Display>(err: E, context: impl Into<String>) -> eyre::Report {
    eyre!(VrfGenError::InvalidInput(err.to_string())).wrap_err(context.into())
}

pub fn wrap_blockchain_err<E: std::fmt::Display>(err: E, context: impl Into<String>) -> eyre::Report {
    eyre!(VrfGenError::BlockchainError(err.to_string())).wrap_err(context.into())
}

pub fn setup_error_handling() -> Result<()> {
    color_eyre::install()?;
    tracing_subscriber::fmt::init();
    Ok(())
}
