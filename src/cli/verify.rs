use eyre::Result;
use crate::vrf::{get_secret_key, VrfContext};

pub fn verify_vrf_proof(
    message: &[u8],
    pi: &[u8],
    force_key_gen: bool,
    silent: bool,
) -> Result<()> {
    let secret_key = get_secret_key(force_key_gen, silent)?;
    let mut vrf_ctx = VrfContext::new(secret_key)?;
    vrf_ctx.verify_proof(message, pi)?;

    if !silent {
        println!("Proof is valid");
    }
    Ok(())
}
