//! v5 Live — Latest NINE65 (superset of v04)
//!
//! Full 9-crate workspace. Adds over v04:
//! - DualRNS ciphertexts with K-Elimination multiplication
//! - Galois automorphisms (rotation/conjugation)
//! - Bootstrap-free circuit compiler
//! - Parallel encrypt/decrypt
//! - BatchEncoder (SIMD slots)
//! - Clockwork-core formal RNS (optional)
//! - NexGen rational arithmetic (optional)
//! - fhe-service HTTP wrapper (optional)
//!
//! Note: v5 gates `FHEConfig::light()` behind `allow_insecure`.
//! We use `FHEConfig::light_mul()` which is always available.

use std::cell::RefCell;
use crate::ArmadaFHE;

use nine65_v5::prelude::*;

/// Opaque FHE context wrapping NINE65 v5's BFV pipeline.
pub struct V05Fhe {
    config: FHEConfig,
    ntt: NTTEngine,
    rng: RefCell<ShadowHarvester>,
    encoder: BFVEncoder,
    public_key: PublicKey,
    secret_key: SecretKey,
    eval_key: EvaluationKey,
}

impl ArmadaFHE for V05Fhe {
    type Ciphertext = Ciphertext;

    fn setup_light() -> Self {
        // v5 gates light() behind allow_insecure; use light_mul() instead
        #[allow(deprecated)]
        let config = FHEConfig::light_mul();
        let ntt = NTTEngine::new(config.q, config.n);
        let mut rng = ShadowHarvester::with_seed(42);
        let keys = KeySet::generate(&config, &ntt, &mut rng);
        let encoder = BFVEncoder::new(&config);
        Self {
            config,
            ntt,
            rng: RefCell::new(rng),
            encoder,
            public_key: keys.public_key,
            secret_key: keys.secret_key,
            eval_key: keys.eval_key,
        }
    }

    fn encrypt(&self, value: u64) -> Self::Ciphertext {
        let encryptor = BFVEncryptor::new(
            &self.public_key,
            &self.encoder,
            &self.ntt,
            self.config.eta,
        );
        encryptor.encrypt(value, &mut *self.rng.borrow_mut())
    }

    fn decrypt(&self, ct: &Self::Ciphertext) -> u64 {
        let decryptor = BFVDecryptor::new(&self.secret_key, &self.encoder, &self.ntt);
        decryptor.decrypt(ct)
    }

    fn add(&self, a: &Self::Ciphertext, b: &Self::Ciphertext) -> Self::Ciphertext {
        let evaluator = BFVEvaluator::new(&self.ntt, &self.encoder, Some(&self.eval_key));
        evaluator.add(a, b)
    }

    #[allow(deprecated)]
    fn mul(&self, a: &Self::Ciphertext, b: &Self::Ciphertext) -> Self::Ciphertext {
        let evaluator = BFVEvaluator::new(&self.ntt, &self.encoder, Some(&self.eval_key));
        evaluator.mul(a, b)
    }

    fn sub(&self, a: &Self::Ciphertext, b: &Self::Ciphertext) -> Self::Ciphertext {
        let evaluator = BFVEvaluator::new(&self.ntt, &self.encoder, Some(&self.eval_key));
        evaluator.sub(a, b)
    }

    fn plaintext_modulus(&self) -> u64 {
        self.config.t
    }
}
