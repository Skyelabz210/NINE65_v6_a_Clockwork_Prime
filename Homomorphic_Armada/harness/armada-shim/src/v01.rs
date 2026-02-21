//! v01 Original — Zero-dep BFV FHE
//!
//! The first QMNF FHE build. No external dependencies.
//! API: FHEConfig::light(), KeySet::generate(), BFVEncoder/Encryptor/Decryptor/Evaluator

use std::cell::RefCell;
use crate::ArmadaFHE;

use fhe_v01::prelude::*;

/// Opaque FHE context wrapping v01's BFV pipeline.
pub struct V01Fhe {
    config: FHEConfig,
    ntt: NTTEngine,
    rng: RefCell<ShadowHarvester>,
    encoder: BFVEncoder,
    public_key: PublicKey,
    secret_key: SecretKey,
    eval_key: EvaluationKey,
}

impl ArmadaFHE for V01Fhe {
    type Ciphertext = Ciphertext;

    fn setup_light() -> Self {
        let config = FHEConfig::light();
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
