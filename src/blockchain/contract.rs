use crate::bindings::Wyrd;
use crate::vrf::get_prover_private_key;
use alloy::network::EthereumWallet;
use alloy::primitives::{Address, B256, U256};
use alloy::providers::fillers::{
    BlobGasFiller, ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller, WalletFiller,
};
use alloy::providers::{builder, Identity, ProviderBuilder, RootProvider};
use alloy::signers::local::PrivateKeySigner;
use contract_bindings::wyrd::Wyrd::{get_alphaReturn, get_request_statusReturn};
use eyre::{eyre, Result};
use std::str::FromStr;

/// VRF contract wrapper for interacting with the Wyrd contract
pub struct VrfContract {
    pub contract: Wyrd::WyrdInstance<
        (),
        FillProvider<
            JoinFill<
                JoinFill<
                    Identity,
                    JoinFill<
                        GasFiller,
                        JoinFill<BlobGasFiller, JoinFill<NonceFiller, ChainIdFiller>>,
                    >,
                >,
                WalletFiller<EthereumWallet>,
            >,
            RootProvider,
        >,
    >,
}

impl VrfContract {
    /// Create a new contract instance
    pub fn new(address_str: &str, rpc_url: &str) -> Result<Self> {
        // Create provider
        let private_key = get_prover_private_key()?;
        let signer: PrivateKeySigner = private_key.parse().expect("should parse private key");
        let wallet = EthereumWallet::from(signer);
        let provider = ProviderBuilder::new()
            .wallet(wallet)
            .on_http(rpc_url.parse()?);

        // Parse contract address
        let address =
            Address::from_str(address_str).map_err(|e| eyre!("Invalid contract address: {}", e))?;

        // Create contract instance
        let contract = Wyrd::new(address, provider.clone());

        Ok(Self { contract })
    }

    /// Get alpha value for a VRF request
    pub async fn get_alpha(&self, req_id: u64) -> Result<B256> {
        let req_id_u256 = U256::from(req_id);
        tracing::debug!("Getting alpha for request ID: {}", req_id_u256.to_string());
        let result: get_alphaReturn = self.contract.get_alpha(req_id_u256).call().await.unwrap();
        tracing::debug!("Received alpha value: {:?}", result._0.to_string());
        Ok(result._0.clone())
    }

    /// Verify if a beta value is valid for the given proof
    pub async fn verify_beta(&self, proof: [U256; 4], beta: Vec<u8>) -> Result<bool> {
        // Call the contract's verify function
        let beta_bytes = beta.into();
        let result = self
            .contract
            .verify_beta(proof, beta_bytes)
            .call()
            .await
            .unwrap();
        Ok(result._0)
    }

    /// Check if a randomness request is active
    pub async fn is_request_active(&self, req_id: u64) -> Result<(bool, u8)> {
        let result: get_request_statusReturn = self
            .contract
            .get_request_status(U256::from(req_id))
            .call()
            .await.unwrap();
        Ok((result.active, result.remaining_sources))
    }

    /// Get the SAV public key from the contract
    pub async fn get_sav_public_key(&self) -> Result<[U256; 2]> {
        let pk_x = self.contract.SAV_PUB_KEY(U256::ZERO).call().await?._0;
        let pk_y = self.contract.SAV_PUB_KEY(U256::ONE).call().await?._0;
        Ok([pk_x, pk_y])
    }

    /// Check if SAV VRF is enabled
    pub async fn is_sav_enabled(&self) -> Result<bool> {
        let result = self.contract.sav_enabled().call().await?;
        Ok(result._0)
    }
}
