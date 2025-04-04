use alloy::primitives::U256;
use eyre::{eyre, Result, Error};
use vrf::openssl::{BigNum, CipherSuite, EcPoint, Error as OpenSSLError, ECVRF};
use vrf::VRF;
use tracing::{debug, error, info};

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
        let pi = self
            .vrf
            .prove(&self.secret_key, message)
            .map_err(|e| eyre!("Failed to generate proof: {}", e))?;
        let hash = self
            .vrf
            .proof_to_hash(&pi)
            .map_err(|e| eyre!("Failed to convert proof to hash: {}", e))?;
        info!("Proof length: {}", pi.len());
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

    pub fn public_key_point(&mut self) -> Result<EcPoint, OpenSSLError> {
        self.vrf.decode_public_key(&self.public_key)
    }

    /// Decodes a VRF proof into its component parts.
    ///
    /// # Arguments
    /// * `pi` - The proof bytes to decode
    ///
    /// # Returns
    /// Returns a tuple containing:
    /// 1. `EcPoint`: The gamma point component of the proof
    /// 2. `BigNum`: The 'c' component of the proof
    /// 3. `BigNum`: The 's' component of the proof
    ///
    /// # Errors
    /// Returns an `OpenSSLError` if the proof cannot be decoded
    pub fn decode_proof(&mut self, pi: &[u8]) -> Result<(EcPoint, BigNum, BigNum), OpenSSLError> {
        self.vrf.decode_proof(pi)
    }

    pub fn decode_proof_as_u256(&mut self, pi: &[u8]) -> Result<[U256; 4], OpenSSLError> {
        let (gamma, c, s) = self.vrf.decode_proof(pi)?;
        let (gamma_x, gamma_y) = self.vrf.ecpoint_to_affine(&gamma)?;
        Ok([
            U256::from_be_slice(&gamma_x.to_vec_padded(32)?),
            U256::from_be_slice(&gamma_y.to_vec_padded(32)?),
            U256::from_be_slice(&c.to_vec_padded(32)?),
            U256::from_be_slice(&s.to_vec_padded(32)?),
        ])
    }

    /// Computes parameters needed for fast verification.
    ///
    /// Returns a tuple containing:
    /// 1. `pk`: The public key point
    /// 2. `decoded_proof`: A tuple of (gamma, c, s) values from the decoded proof
    /// 3. `u_point`: The computed U component point
    /// 4. `v_component`: An array containing the computed V component points
    pub fn compute_fast_params(
        &mut self,
        message: &[u8],
    ) -> Result<(EcPoint, (EcPoint, BigNum, BigNum), EcPoint, [EcPoint; 2]), OpenSSLError> {
        let pk = self.public_key_point()?;
        let (pi, hash) = self.generate_proof(message).unwrap();
        let decoded_proof = self.vrf.decode_proof(&pi)?;
        let u_point = self
            .vrf
            .compute_u_component(&pk, &decoded_proof.2, &decoded_proof.1)?;
        let v_component = self.vrf.compute_v_component(
            &pk,
            message,
            &(&decoded_proof.0, &decoded_proof.1, &decoded_proof.2),
        )?;
        assert_eq!(
            self.vrf
                .gamma_to_hash(&decoded_proof.0)
                .map_err(OpenSSLError::from)?,
            hash
        );
        Ok((pk, decoded_proof, u_point, v_component))
    }

    pub fn secret_key(&self) -> &[u8; 32] {
        &self.secret_key
    }

    pub fn ecpoint_to_affine(&mut self, point: &EcPoint) -> Result<(BigNum, BigNum), OpenSSLError> {
        self.vrf.ecpoint_to_affine(point)
    }
}
