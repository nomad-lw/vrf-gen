use clap::Parser;
use dotenv::dotenv;
use hex;
use rand::random;
use serde_json::json;
use std::env;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::Write;
use vrf::openssl::Error as OpenSSLError;
use vrf::openssl::{CipherSuite, ECVRF};
use vrf::VRF;

const SECRET_KEY_ENV: &str = "VRF_SECRET_KEY";

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Operation to perform: "prove" or "verify"
    #[arg(short, long)]
    operation: String,

    /// Message to use for the operation (hex-string)
    #[arg(short, long)]
    message: String,

    /// proof (pi) value to use for the operation (hex-string) (only if operation is "verify")
    #[arg(short, long)]
    pi: Option<String>,

    /// Force generation of a new secret key
    #[arg(short, long)]
    force_key_gen: bool,

    /// Silent mode
    #[arg(short, long)]
    silent: bool,

    /// Soft mode
    #[arg(long)]
    soft: bool,

    // JSON output file suffix
    #[arg(short, long, default_value = "")]
    json: String,
}

#[derive(Debug)]
struct VrfError {
    inner: OpenSSLError,
}

impl fmt::Display for VrfError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "VRF error: {}", self.inner)
    }
}

impl Error for VrfError {}

impl From<OpenSSLError> for VrfError {
    fn from(error: OpenSSLError) -> Self {
        VrfError { inner: error }
    }
}

fn generate_secret_key() -> [u8; 32] {
    println!("Generating a new secret key...");
    let key: [u8; 32] = random();
    println!("Secret key generated: {}", hex::encode(&key));
    // Save the key to .env file
    let env_file_content = format!("{}={}\n", SECRET_KEY_ENV, hex::encode(&key));
    let mut env_file = match File::options().append(true).open(".env") {
        Ok(file) => file,
        Err(_) => File::create(".env").expect("Unable to create .env file"),
    };
    env_file
        .write_all(env_file_content.as_bytes())
        .expect("Unable to write to .env file");
    key
}

fn get_secret_key(force: bool, silent: bool) -> [u8; 32] {
    // let force = force.unwrap_or(false);
    if force {
        generate_secret_key()
    } else if let Ok(secret_key) = env::var(SECRET_KEY_ENV) {
        if !silent {
            println!("Using existing secret key: {}", secret_key);
            println!("Decoding secret key...");
        }
        hex::decode(secret_key)
            .expect("Invalid secret key format")
            .try_into()
            .expect("Secret key must be 32 bytes")
    } else {
        generate_secret_key()
    }
}

fn generate_vrf_proof(
    message: &[u8],
    force_key_gen: bool,
    silent: bool,
    soft: bool,
    json: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Initialization of VRF context by providing a curve
    let mut vrf = ECVRF::from_suite(CipherSuite::SECP256K1_SHA256_TAI).unwrap();

    // Generate a new secret key
    let secret_key = get_secret_key(force_key_gen, silent);
    // Derive the public key from the secret key
    let public_key = vrf.derive_public_key(&secret_key).unwrap();

    // Create a VRF proof of the message
    let pi = vrf.prove(&secret_key, &message).unwrap();

    // Convert the proof to a hash
    let hash = vrf.proof_to_hash(&pi).unwrap();

    // Create a JSON object with the secret key, public key, message, proof, and hash
    let json_data = json!({
        "secret_key": format!("0x{}", hex::encode(&secret_key)),
        "public_key": format!("0x{}", hex::encode(&public_key)),
        "message": format!("0x{}", hex::encode(&message)),
        "proof": format!("0x{}", hex::encode(&pi)),
        "hash": format!("0x{}", hex::encode(&hash)),
    });

    if soft {
        print!("{}", json_data.to_string());
    } else {
        // Write the JSON object to a file
        let mut file = File::create(format!("vrf_proof{}.json", json))?;
        file.write_all(json_data.to_string().as_bytes())?;
    }

    Ok(())
}

fn verify_vrf_proof(
    message: &[u8],
    pi: &[u8],
    force_key_gen: bool,
    silent: bool,
) -> Result<(), Box<dyn Error>> {
    let mut vrf = ECVRF::from_suite(CipherSuite::SECP256K1_SHA256_TAI).unwrap();
    let secret_key = get_secret_key(force_key_gen, silent);
    let public_key = vrf.derive_public_key(&secret_key).unwrap();
    // let pi = vrf.prove(&secret_key, &message).unwrap();
    // let hash = vrf.proof_to_hash(&pi).unwrap();
    match vrf.verify(&public_key, &pi, &message) {
        Ok(_) => {
            if !silent {
                println!("Proof is valid");
            }
        }
        Err(e) => {
            if !silent {
                eprintln!("Proof was invalid: {}", e);
            }
            return Err(Box::new(VrfError::from(e)));
        }
    }

    Ok(())
}

fn decode_hex_string(hex_str: &str) -> Result<Vec<u8>, hex::FromHexError> {
    if hex_str.starts_with("0x") {
        hex::decode(&hex_str[2..])
    } else {
        hex::decode(hex_str)
    }
}

fn main() {
    dotenv().ok();
    let args = Args::parse();

    let message = decode_hex_string(&args.message).expect("Invalid message format");
    if !args.silent {
        println!("Number of bytes in message: {}", message.len());
    }
    // Generate the VRF proof and export to JSON file
    match args.operation.as_str() {
        "prove" => {
            if let Err(e) = generate_vrf_proof(&message, args.force_key_gen, args.silent, args.soft, &args.json) {
                if !args.silent {
                    eprintln!("Error generating VRF proof: {}", e);
                } else {
                    print!("Error");
                }
                std::process::exit(1);
            }
            if !args.silent {
                println!("VRF proof generated and exported to vrf_proof{}.json", args.json);
            } else {
                if args.soft {
                    // No operation if soft is true
                } else {
                    print!("Ok");
                }
            }
        }
        "verify" => {
            let pi: Vec<u8> = match args.pi {
                Some(ref pi_str) => decode_hex_string(pi_str).expect("Invalid proof format"),
                None => {
                    eprintln!("Proof (pi) is required for the 'verify' operation");
                    std::process::exit(1);
                }
            };
            if let Err(e) = verify_vrf_proof(&message, &pi, args.force_key_gen, args.silent) {
                if !args.silent {
                    eprintln!("Error verifying VRF proof: {}", e);
                } else {
                    print!("Error");
                }
                std::process::exit(1);
            }
            if !args.silent {
                println!("VRF proof verified successfully");
            } else {
                print!("Ok");
            }
        }
        _ => {
            eprintln!("Unsupported operation: {}", args.operation);
            std::process::exit(1);
        }
    }
}
