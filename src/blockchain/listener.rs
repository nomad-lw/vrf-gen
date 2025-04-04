use crate::bindings::Wyrd;
use crate::blockchain::contract::VrfContract;
use crate::vrf::{get_secret_key, VrfContext};
use alloy::contract::Event;
use alloy::primitives::{b256, Bytes, FixedBytes, B256, U256};
use alloy::providers::Provider;
use alloy::rpc::types::Log;
use contract_bindings::wyrd::Wyrd::{compute_fast_verify_paramsReturn, WyrdEvents};
use eyre::{eyre, Result};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tracing::{debug, error, info};

/// Flag indicating SAV VRF is the source (value = 4)
const SAV_FLAG: u8 = 4;

/// Configuration for blockchain listening
pub struct ListenerConfig {
    /// RPC URL for the blockchain node
    pub rpc_url: String,
    /// Contract address of the Wyrd VRF provider
    pub contract_address: String,
    /// Block range to check for events in one query
    pub block_range: u64,
    /// Polling interval in seconds
    pub poll_interval: u64,
    /// Whether to force key generation
    pub force_key_gen: bool,
    /// Whether to run in silent mode
    pub silent: bool,
    /// Start listening from latest block minus this value (0 means start from latest)
    pub start_blocks_behind: u64,
}

impl Default for ListenerConfig {
    fn default() -> Self {
        Self {
            rpc_url: "http://localhost:8545".to_string(),
            contract_address: "0x0000000000000000000000000000000000000000".to_string(),
            block_range: 2000,
            poll_interval: 15,
            force_key_gen: false,
            silent: false,
            start_blocks_behind: 0,
        }
    }
}

impl ListenerConfig {
    /// Create a new ListenerConfig with optional parameters
    /// Any parameter not provided will use the default value
    pub fn new(
        rpc_url: Option<String>,
        contract_address: Option<String>,
        block_range: Option<u64>,
        poll_interval: Option<u64>,
        force_key_gen: Option<bool>,
        silent: Option<bool>,
        start_blocks_behind: Option<u64>,
    ) -> Self {
        let default = Self::default();
        Self {
            rpc_url: rpc_url.unwrap_or(default.rpc_url),
            contract_address: contract_address.unwrap_or(default.contract_address),
            block_range: block_range.unwrap_or(default.block_range),
            poll_interval: poll_interval.unwrap_or(default.poll_interval),
            force_key_gen: force_key_gen.unwrap_or(default.force_key_gen),
            silent: silent.unwrap_or(default.silent),
            start_blocks_behind: start_blocks_behind.unwrap_or(default.start_blocks_behind),
        }
    }
}

/// Blockchain event listener for VRF requests
pub struct BlockchainListener {
    config: ListenerConfig,
    vrf_ctx: Arc<Mutex<VrfContext>>,
    latest_block: u64,
    contract: Option<VrfContract>,
}

impl BlockchainListener {
    /// Create a new blockchain listener with the given configuration
    pub async fn new(config: ListenerConfig) -> Result<Self> {
        // Initialize VRF context with the secret key
        let secret_key = get_secret_key(config.force_key_gen, config.silent)?;
        let vrf_ctx = Arc::new(Mutex::new(VrfContext::new(secret_key)?));

        if !config.silent {
            info!("VRF context initialized with public key");
        }

        Ok(Self {
            config,
            vrf_ctx,
            latest_block: 0,
            contract: None,
        })
    }

    /// Connect to the blockchain and initialize the contract
    pub async fn connect(&mut self) -> Result<()> {
        // Create VrfContract instance
        let contract = VrfContract::new(&self.config.contract_address, &self.config.rpc_url)?;
        self.contract = Some(contract);

        if !self.config.silent {
            info!("Connected to blockchain node at {}", self.config.rpc_url);
            info!(
                "Listening for RandomnessRequested events with flag={} (SAV VRF)",
                SAV_FLAG
            );
            info!("Contract address: {}", self.config.contract_address);
        }

        // Get latest block from the provider
        let provider = (self.contract.as_ref().unwrap()).contract.provider();
        let block_number = provider.get_block_number().await?;

        // Calculate starting block based on start_blocks_behind
        if self.config.start_blocks_behind > 0 {
            if block_number >= self.config.start_blocks_behind {
                self.latest_block = block_number - self.config.start_blocks_behind;
                if !self.config.silent {
                    info!(
                        "Starting from {} blocks behind latest (block {})",
                        self.config.start_blocks_behind, self.latest_block
                    );
                }
            } else {
                self.latest_block = 0;
                if !self.config.silent {
                    info!("Requested to start {} blocks behind, but chain only has {} blocks. Starting from block 0.",
                         self.config.start_blocks_behind, block_number);
                }
            }
        } else {
            self.latest_block = block_number;
            if !self.config.silent {
                info!("Starting from latest block {}", self.latest_block);
            }
        }

        debug!("Starting from block {}", self.latest_block);

        Ok(())
    }

    /// Listen for blockchain events and process them
    pub async fn listen(&mut self) -> Result<()> {
        if self.contract.is_none() {
            return Err(eyre!("Not connected to blockchain. Call connect() first."));
        }

        let contract = self.contract.as_ref().unwrap();
        let mut interval = tokio::time::interval(Duration::from_secs(self.config.poll_interval));

        loop {
            interval.tick().await;

            if !self.config.silent {
                debug!("Polling for new randomness requests...");
            }

            // Get the current block
            let provider = contract.contract.provider();
            match provider.get_block_number().await {
                Ok(current_block) => {
                    if current_block <= self.latest_block {
                        continue;
                    }

                    // Calculate from_block and to_block
                    let from_block = self.latest_block + 1;
                    let to_block =
                        std::cmp::min(current_block, from_block + self.config.block_range - 1);

                    // Update latest_block for next iteration
                    self.latest_block = to_block;

                    if !self.config.silent {
                        debug!("Scanning blocks {} to {}", from_block, to_block);
                    }

                    // Process events for the block range
                    match self.process_events(from_block, to_block).await {
                        Ok(count) => {
                            if count > 0 && !self.config.silent {
                                info!("Processed {} SAV VRF requests", count);
                            }
                        }
                        Err(e) => {
                            error!("Error processing events: {}", e);
                        }
                    }
                }
                Err(e) => {
                    error!("Failed to get latest block: {}", e);
                    tokio::time::sleep(Duration::from_secs(5)).await;
                }
            }
        }
    }

    /// Process RandomnessRequested events in the given block range
    async fn process_events(&self, from_block: u64, to_block: u64) -> Result<usize> {
        let contract = self.contract.as_ref().unwrap();

        // Get raw contract instance to access event filters
        let raw_contract = &contract.contract;

        // Create a filter for RandomnessRequested events
        let events: Vec<(Wyrd::RandomnessRequested, Log)> = raw_contract
            .RandomnessRequested_filter()
            .from_block(from_block)
            .to_block(to_block)
            .topic2(U256::from(SAV_FLAG))
            .query()
            .await
            .unwrap();

        let mut processed_count = 0;

        for event in events {
            let req_id = event.0.req_id;

            if !self.config.silent {
                info!("Found SAV VRF request with ID: {}", req_id);
            }

            // Process the request
            if let Err(e) = self.process_vrf_request(req_id).await {
                error!("Failed to process VRF request {}: {}", req_id, e);
                continue;
            }

            processed_count += 1;
        }

        Ok(processed_count)
    }

    /// Process a single VRF request
    async fn process_vrf_request(&self, req_id: impl Into<U256>) -> Result<()> {
        let contract = self.contract.as_ref().unwrap();
        let req_id_u256 = U256::from(req_id.into());
        let req_id_u64 = req_id_u256.to::<u64>();

        // Get the alpha value from the contract
        let alpha: B256 = contract.get_alpha(req_id_u64).await?;

        if !self.config.silent {
            debug!(
                "Retrieved alpha for request {}: {:?}",
                req_id_u256.to_string(),
                alpha.to_string()
            );
        }

        // Generate VRF proof
        // Generate proof WITHOUT holding the lock during async operations
        let (proof, hash) = {
            let mut vrf_ctx = self.vrf_ctx.lock().await;
            let alpha_bytes = alpha.as_slice();
            info!("Alpha value: 0x{}", hex::encode(alpha_bytes));
            vrf_ctx.generate_proof(alpha_bytes)?
        };

        if !self.config.silent {
            info!("Generated VRF proof: 0x{}", hex::encode(&proof));
            info!("Generated VRF hash: 0x{}", hex::encode(&hash));
        }

        // Check if request is still active and SAV is the last remaining source
        let mut retries = 0;
        const MAX_RETRIES: u8 = 5;
        const RETRY_DELAY: Duration = Duration::from_secs(5);

        loop {
            let (is_active, remaining_sources) = contract.is_request_active(req_id_u64).await?;

            if !is_active {
                return Err(eyre!("Request {} is no longer active", req_id_u64));
            }

            if remaining_sources == SAV_FLAG {
                break;
            }

            if retries >= MAX_RETRIES {
                return Err(eyre!(
                    "SAV not the last remaining source for request {} after {} retries",
                    req_id_u64,
                    MAX_RETRIES
                ));
            }

            if !self.config.silent {
                info!(
                    "Waiting for SAV to be last source (current: {}) for request {}, retry {}/{}",
                    remaining_sources,
                    req_id_u64,
                    retries + 1,
                    MAX_RETRIES
                );
            }

            tokio::time::sleep(RETRY_DELAY).await;
            retries += 1;
        }

        // Prepare the proof and send to the contract
        self.submit_vrf_proof(req_id_u256, alpha, proof, hash)
            .await?;

        Ok(())
    }

    /// Submit VRF proof back to the contract
    async fn submit_vrf_proof(
        &self,
        req_id: U256,
        alpha: B256,
        proof: Vec<u8>,
        hash: Vec<u8>,
    ) -> Result<()> {
        let contract = self.contract.as_ref().unwrap();
        let raw_contract = &contract.contract;

        // Convert proof to the format expected by the contract
        // Assuming proof is laid out as [gamma_x, gamma_y, c, s] in consecutive blocks
        if !self.config.silent {
            debug!("Proof length: {}", proof.len());
            debug!("Proof bytes: 0x{}", hex::encode(&proof));
        }
        if proof.len() < 51 {
            return Err(eyre!("Proof is too short, expected 51 bytes"));
        }

        let mut vrf_ctx_lock = self
            .vrf_ctx
            .try_lock()
            .map_err(|_| eyre!("Failed to acquire VRF context lock"))?;

        let proof_array = vrf_ctx_lock.decode_proof_as_u256(&proof).unwrap();

        // Get U and V values for fast verification
        let result: compute_fast_verify_paramsReturn = raw_contract
            .compute_fast_verify_params(proof_array, Bytes::copy_from_slice(alpha.as_slice()))
            .call()
            .await?;

        let u_values = result.U;
        let v_values = result.V;

        if !self.config.silent {
            info!("Submitting VRF proof for request {}", req_id);
        }

        // Convert hash to B256
        let hash_b256 = if hash.len() == 32 {
            let mut bytes = [0u8; 32];
            bytes.copy_from_slice(&hash);
            B256::from(bytes)
        } else {
            return Err(eyre!("Hash must be 32 bytes"));
        };

        // Send the callback with the proof
        let tx = raw_contract
            .sav_callback(
                req_id,
                Bytes::copy_from_slice(alpha.as_slice()),
                hash_b256,
                proof_array,
                u_values,
                v_values,
            )
            .send()
            .await?;

        if !self.config.silent {
            info!(
                "VRF proof submitted in transaction: {:?}",
                tx.get_receipt().await?
            );
        }

        Ok(())
    }
}

/// Listen to blockchain events and process VRF requests
pub async fn listen_to_blockchain_events(config: ListenerConfig) -> Result<()> {
    let mut listener = BlockchainListener::new(config).await?;
    listener.connect().await?;
    listener.listen().await
}
