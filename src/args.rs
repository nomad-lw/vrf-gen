use clap::Parser;

const DEFAULT_RPC_URL: &str = "ws://localhost:8545";
const DEFAULT_CONTRACT_ADDRESS: &str = "0xYourContractAddressHere";

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Operation to perform: "prove", "verify", or "listen"
    #[arg(short, long)]
    pub operation: String,

    /// Message to use for the operation (hex-string)
    #[arg(short, long)]
    pub message: Option<String>,

    /// Proof (pi) value to use for the operation (hex-string) (only if operation is "verify")
    #[arg(short, long)]
    pub pi: Option<String>,

    /// Force generation of a new secret key
    #[arg(short, long)]
    pub force_key_gen: bool,

    /// Silent mode
    #[arg(short, long)]
    pub silent: bool,

    /// Soft mode
    #[arg(long)]
    pub soft: bool,

    /// JSON output file suffix
    #[arg(short, long, default_value = "")]
    pub json: String,

    /// Ethereum RPC URL (only for listen mode)
    #[arg(long, default_value = DEFAULT_RPC_URL)]
    pub rpc_url: String,

    /// Contract address to listen to (only for listen mode)
    #[arg(long, default_value = DEFAULT_CONTRACT_ADDRESS)]
    pub contract_address: String,
}
