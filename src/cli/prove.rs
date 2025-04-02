use eyre::Result;
use crate::vrf::{get_secret_key, VrfContext};
use serde_json::json;
use std::fs::File;
use std::io::Write;

pub fn generate_vrf_proof(
    message: &[u8],
    force_key_gen: bool,
    silent: bool,
    soft: bool,
    json_suffix: &str,
) -> Result<()> {
    let secret_key = get_secret_key(force_key_gen, silent)?;
    let mut vrf_ctx = VrfContext::new(secret_key)?;
    let (pi, hash) = vrf_ctx.generate_proof(message)?;

    let json_data = json!({
        "secret_key": format!("0x{}", hex::encode(vrf_ctx.secret_key())),
        "public_key": format!("0x{}", hex::encode(vrf_ctx.public_key())),
        "message": format!("0x{}", hex::encode(message)),
        "proof": format!("0x{}", hex::encode(&pi)),
        "hash": format!("0x{}", hex::encode(&hash)),
    });

    if soft {
        print!("{}", json_data.to_string());
    } else {
        let mut file = File::create(format!("vrf_proof{}.json", json_suffix))?;
        file.write_all(json_data.to_string().as_bytes())?;
    }

    Ok(())
}
