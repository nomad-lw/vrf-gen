use eyre::{Result, eyre};
use vrf::openssl::{CipherSuite, ECVRF};
use vrf::VRF;

pub struct VrfContext {
    vrf: ECVRF,
    secret_key: [u8; 32],
    public_key: Vec<u8>,
}

impl VrfContext {
    pub fn new(secret_key: [u8; 32]) -> Result<Self> {
        let mut vrf = ECVRF::from_suite(CipherSuite::SECP256K1_SHA256_TAI)
            .map_err(|e| eyre!("VRF initialization error: {}", e))?;
        let public_key = vrf
            .derive_public_key(&secret_key)
            .map_err(|e| eyre!("Failed to derive public key: {}", e))?;

        Ok(Self {
            vrf,
            secret_key,
            public_key,
        })
    }

    pub fn generate_proof(&mut self, message: &[u8]) -> Result<(Vec<u8>, Vec<u8>)> {
        let pi = self.vrf
            .prove(&self.secret_key, message)
            .map_err(|e| eyre!("Failed to generate proof: {}", e))?;
        let hash = self.vrf
            .proof_to_hash(&pi)
            .map_err(|e| eyre!("Failed to convert proof to hash: {}", e))?;
        Ok((pi, hash))
    }

    pub fn verify_proof(&mut self, message: &[u8], pi: &[u8]) -> Result<()> {
        self.vrf
            .verify(&self.public_key, pi, message)
            .map_err(|e| eyre!("Proof verification failed: {}", e))?;
        Ok(())
    }

    pub fn public_key(&self) -> &[u8] {
        &self.public_key
    }

    pub fn secret_key(&self) -> &[u8; 32] {
        &self.secret_key
    }
}
