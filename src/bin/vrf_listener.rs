use clap::Parser;
use eyre::Result;
use tracing::info;
use vrf_genkit::{
    args::{DEFAULT_CONTRACT_ADDRESS, DEFAULT_RPC_URL},
    blockchain::{listen_to_blockchain_events, ListenerConfig},
    error::setup_error_handling,
};

#[derive(Parser, Debug)]
#[command(version, about = "Listen for VRF randomness requests", long_about = None)]
struct Args {
    /// Ethereum RPC URL
    #[arg(long, default_value = DEFAULT_RPC_URL)]
    rpc_url: String,

    /// Contract address to listen to
    #[arg(long, default_value = DEFAULT_CONTRACT_ADDRESS)]
    contract_address: String,

    /// Force generation of a new secret key
    #[arg(short, long)]
    force_key_gen: bool,

    /// Silent mode
    #[arg(short, long)]
    silent: bool,

    /// Start listening from latest block minus this value (0 means start from latest)
    #[arg(long)]
    start_block_delta: Option<u64>,
}

#[tokio::main]
async fn main() -> Result<()> {
    setup_error_handling()?;

    let args = Args::parse();

    if !args.silent {
        info!("Starting VRF listener...");
    }

    listen_to_blockchain_events(ListenerConfig::new(
        Some(args.rpc_url.clone()),
        Some(args.contract_address.clone()),
        None,
        None,
        Some(args.force_key_gen),
        Some(args.silent),
        args.start_block_delta,
    ))
    .await
}
