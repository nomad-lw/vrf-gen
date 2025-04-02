use eyre::{Result, eyre};
use dotenv::dotenv;
use hex;
use rand::random;
use std::env;
use std::fs::File;
use std::io::Write;

const SECRET_KEY_ENV: &str = "VRF_SECRET_KEY";

pub fn generate_secret_key() -> Result<[u8; 32]> {
    println!("Generating a new secret key...");
    let key: [u8; 32] = random();
    println!("Secret key generated: {}", hex::encode(&key));

    let env_file_content = format!("{}={}\n", SECRET_KEY_ENV, hex::encode(&key));
    let mut env_file = File::options()
        .append(true)
        .open(".env")
        .or_else(|_| File::create(".env"))?;

    env_file.write_all(env_file_content.as_bytes())?;
    Ok(key)
}

pub fn get_secret_key(force: bool, silent: bool) -> Result<[u8; 32]> {
    dotenv().ok();

    if force {
        generate_secret_key()
    } else if let Ok(secret_key) = env::var(SECRET_KEY_ENV) {
        if !silent {
            println!("Using existing secret key: {}", secret_key);
            println!("Decoding secret key...");
        }
        let bytes = hex::decode(secret_key)?;
        bytes.try_into()
            .map_err(|_| eyre!("Secret key must be 32 bytes"))
    } else {
        generate_secret_key()
    }
}
