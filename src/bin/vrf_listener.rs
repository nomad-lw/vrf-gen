use clap::Parser;
use eyre::Result;
use tracing::info;
use gen_vrf::blockchain::listen_to_blockchain_events;
use gen_vrf::error::setup_error_handling;

#[derive(Parser, Debug)]
#[command(version, about = "Listen for VRF randomness requests", long_about = None)]
struct Args {
    /// Ethereum RPC URL
    #[arg(long, default_value = "http://localhost:8545")]
    rpc_url: String,

    /// Contract address to listen to
    #[arg(long)]
    contract_address: String,

    /// Force generation of a new secret key
    #[arg(short, long)]
    force_key_gen: bool,

    /// Silent mode
    #[arg(short, long)]
    silent: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    setup_error_handling()?;

    let args = Args::parse();

    if !args.silent {
        info!("Starting VRF listener...");
    }

    listen_to_blockchain_events(
        &args.rpc_url,
        &args.contract_address,
        args.force_key_gen,
        args.silent,
    )
    .await
}
