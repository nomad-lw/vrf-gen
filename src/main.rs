use clap::Parser;
use eyre::{Result, eyre};
use tracing::{info, error};

mod args;
use args::Args;

use vrf_genkit::{
    blockchain::{listen_to_blockchain_events,ListenerConfig},
    cli::{generate_vrf_proof, verify_vrf_proof},
    error::{setup_error_handling, wrap_input_err, wrap_vrf_err, wrap_blockchain_err},
};

#[tokio::main]
async fn main() -> Result<()> {
    setup_error_handling()?;

    let args = Args::parse();

    match args.operation.as_str() {
        "prove" => {
            let message = args.message.ok_or_else(||
                eyre!("Message is required for the 'prove' operation")
            )?;

            let message = decode_hex_string(&message)
                .map_err(|e| wrap_input_err(e, "Failed to decode message hex string"))?;

            generate_vrf_proof(
                &message,
                args.force_key_gen,
                args.silent,
                args.soft,
                &args.json,
            )
            .map_err(|e| wrap_vrf_err(e, "Failed to generate VRF proof"))?;

            if !args.silent {
                info!("VRF proof generated and exported to vrf_proof{}.json", args.json);
            } else if !args.soft {
                print!("Ok");
            }
        }
        "verify" => {
            let message = args.message.ok_or_else(||
                eyre!("Message is required for the 'verify' operation")
            )?;

            let message = decode_hex_string(&message)
                .map_err(|e| wrap_input_err(e, "Failed to decode message hex string"))?;

            let pi = args.pi.ok_or_else(||
                eyre!("Proof (pi) is required for the 'verify' operation")
            )?;

            let pi = decode_hex_string(&pi)
                .map_err(|e| wrap_input_err(e, "Failed to decode proof hex string"))?;

            verify_vrf_proof(&message, &pi, args.force_key_gen, args.silent)
                .map_err(|e| wrap_vrf_err(e, "Failed to verify VRF proof"))?;

            if !args.silent {
                info!("VRF proof verified successfully");
            } else {
                print!("Ok");
            }
        }
        "listen" => {
            let config = ListenerConfig::new(
                Some(args.rpc_url.clone()),
                Some(args.contract_address.clone()),
                None,
                None,
                Some(args.force_key_gen),
                Some(args.silent),
                None
            );

            listen_to_blockchain_events(config)
            .await
            .map_err(|e| wrap_blockchain_err(e, "Blockchain listener failed"))?;
        }
        op => {
            error!("Unsupported operation: {}", op);
            return Err(eyre!("Unsupported operation: {}", op));
        }
    }

    Ok(())
}

fn decode_hex_string(hex_str: &str) -> Result<Vec<u8>> {
    if hex_str.starts_with("0x") {
        hex::decode(&hex_str[2..]).map_err(Into::into)
    } else {
        hex::decode(hex_str).map_err(Into::into)
    }
}
