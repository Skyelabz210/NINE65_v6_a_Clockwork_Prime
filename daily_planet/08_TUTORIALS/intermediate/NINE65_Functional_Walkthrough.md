# NINE65 FHE System: A Functional Walkthrough

**Date**: January 2026
**Version**: 1.0
**Difficulty**: Intermediate

---

## 1. Introduction

This document provides a comprehensive, hands-on walkthrough of the NINE65 Fully Homomorphic Encryption (FHE) system, with a specific focus on its **fully functional capabilities**. The goal is to equip developers with the practical knowledge needed to set up the FHE environment, perform core cryptographic operations, and leverage the library's unique features, such as its deep circuit capacity.

We will proceed step-by-step, from initial configuration to executing complex homomorphic computations.

---

## 2. Core Functional Components

The following components are fully implemented, tested, and ready for use:

| Component | Functionality | Status |
|-----------|---------------|--------|
| **Parameter Configuration** | `SecureConfig` for 128-bit and 192-bit security levels | Fully Functional |
| **Key Generation** | Secure (CSPRNG) and deterministic key generation for `SecretKey`, `PublicKey`, and `EvaluationKey` | Fully Functional |
| **Encoding/Decoding** | `BFVEncoder` for converting integer messages into plaintext polynomials | Fully Functional |
| **Encryption/Decryption** | `BFVEncryptor` and `BFVDecryptor` for both symmetric and asymmetric (public key) encryption | Fully Functional |
| **Error Handling** | Robust `Result`-based error handling (`Nine65Error`) for fallible operations | Fully Functional |
| **Homomorphic Addition** | `BFVEvaluator::add` for ciphertext-ciphertext addition | Fully Functional |
| **Homomorphic Subtraction** | `BFVEvaluator::sub` for ciphertext-ciphertext subtraction | Fully Functional |
| **Homomorphic Multiplication** | `BFVEvaluator::mul` and `relinearize` for ciphertext-ciphertext multiplication | Fully Functional |
| **Deep Circuits (Symmetric)** | Multi-level consecutive multiplications without bootstrapping in symmetric mode | Fully Functional |

---

## 3. Practical Walkthrough: From Setup to Computation

This section provides executable Rust code snippets that demonstrate the end-to-end workflow.

### 3.1. Step 1: Environment Setup

First, we must configure the FHE environment. This involves selecting a security level and initializing the necessary cryptographic engines and context objects.

> **Security Note**: For any production use, always start with a `SecureConfig`. The `secure_128()` preset provides a 128-bit classical security level, compliant with the [Homomorphic Encryption Standard](https://homomorphicencryption.org/).

```rust
use nine65::prelude::*;

// 1. Select a production-ready, 128-bit secure parameter set.
let config = SecureConfig::secure_128().into_config();

// 2. Initialize the NTT (Number Theoretic Transform) engine, which is crucial
//    for efficient polynomial multiplication.
let ntt = NTTEngine::new(config.q, config.n);

// 3. For this walkthrough, we use a deterministic Random Number Generator (RNG)
//    to ensure the examples are reproducible. In production, you MUST use
//    `KeySet::generate_secure(&config, &ntt)` which uses the OS's secure RNG.
let mut rng = ShadowHarvester::with_seed(42);

// 4. Generate the complete set of keys: secret, public, and evaluation keys.
let keys = KeySet::generate(&config, &ntt, &mut rng);

// 5. Instantiate the core FHE components using the generated keys and config.
let encoder = BFVEncoder::new(&config);
let encryptor = BFVEncryptor::new(&keys.public_key, &encoder, &ntt, config.eta);
let decryptor = BFVDecryptor::new(&keys.secret_key, &encoder, &ntt);
let evaluator = BFVEvaluator::new(&ntt, &encoder, Some(&keys.eval_key));

println!("FHE Environment Initialized Successfully!");
```

### 3.2. Step 2: Basic Encryption & Decryption

The most fundamental workflow is the encryption of a plaintext message, followed by its decryption. This confirms that the keys and parameters are configured correctly.

```rust
// Assume the setup from Step 1 is complete.
let message = 12345u64;

// Encrypt the message using the public key and the deterministic RNG.
let ciphertext = encryptor.encrypt(message, &mut rng);

// Decrypt the resulting ciphertext using the secret key.
let decrypted_message = decryptor.decrypt(&ciphertext);

println!("Original Message: {}", message);
println!("Decrypted Message: {}", decrypted_message);

assert_eq!(message, decrypted_message);
println!("Encryption/Decryption Roundtrip Successful!");
```

### 3.3. Step 3: Performing Homomorphic Operations

With the setup confirmed, we can now perform computations directly on encrypted data.

#### Addition and Subtraction

These are the simplest homomorphic operations and have a very low impact on the noise within the ciphertext.

```rust
// Encrypt two separate messages.
let m1 = 100u64;
let m2 = 25u64;
let ct1 = encryptor.encrypt(m1, &mut rng);
let ct2 = encryptor.encrypt(m2, &mut rng);

// Perform homomorphic addition.
let ct_sum = evaluator.add(&ct1, &ct2);
let sum_result = decryptor.decrypt(&ct_sum);
println!("Homomorphic Addition: {} + {} = {}", m1, m2, sum_result);
assert_eq!(sum_result, m1 + m2);

// Perform homomorphic subtraction.
let ct_diff = evaluator.sub(&ct1, &ct2);
let diff_result = decryptor.decrypt(&ct_diff);
println!("Homomorphic Subtraction: {} - {} = {}", m1, m2, diff_result);
assert_eq!(diff_result, m1 - m2);
```

#### Multiplication

Homomorphic multiplication is the most powerful and noise-intensive operation. After each multiplication, a `relinearize` step is required. This step uses the `EvaluationKey` to reduce the size of the resulting ciphertext back to its original form, making it suitable for further operations.

```rust
let m3 = 7u64;
let m4 = 8u64;
let ct3 = encryptor.encrypt(m3, &mut rng);
let ct4 = encryptor.encrypt(m4, &mut rng);

// Perform homomorphic multiplication. This operation returns a `Result`.
let ct_prod_large = evaluator.mul(&ct3, &ct4).unwrap();

// Relinearize the resulting ciphertext to reduce its size and make it usable.
let ct_prod_relin = evaluator.relinearize(&ct_prod_large).unwrap();

let prod_result = decryptor.decrypt(&ct_prod_relin);
println!("Homomorphic Multiplication: {} * {} = {}", m3, m4, prod_result);
assert_eq!(prod_result, m3 * m4);
```

### 3.4. Step 4: Leveraging Deep Circuits (Symmetric Mode)

NINE65's flagship capability is executing deep computational graphs **without bootstrapping**. This functionality is fully operational and most effective in **symmetric encryption mode**. In this mode, the `SecretKey` is used for both encryption and decryption, which is ideal for scenarios where the data owner is also performing the computation.

This example calculates `3^16` homomorphically, a task that requires a multiplicative depth of 4 and would necessitate bootstrapping in most other FHE libraries.

```rust
// 1. Create a new encryptor that uses the secret key (symmetric mode).
let symmetric_encryptor = BFVEncryptor::new_symmetric(
    &keys.secret_key, &encoder, &ntt, config.eta
);

// 2. Encrypt the base number.
let base = 3u64;
let mut ct_power = symmetric_encryptor.encrypt(base, &mut rng);

// 3. Perform 4 consecutive squaring operations (multiplications).
println!("Calculating 3^16 homomorphically...");
for i in 1..=4 {
    let ct_squared = evaluator.mul(&ct_power, &ct_power).unwrap();
    ct_power = evaluator.relinearize(&ct_squared).unwrap();

    let power = 2u64.pow(i + 1);
    let current_val = decryptor.decrypt(&ct_power);
    println!("  Depth {}: Homomorphic result for 3^{} = {}", i, power, current_val);
}

// 4. Verify the final result.
let final_result = decryptor.decrypt(&ct_power);
let expected_result = (3u128.pow(16) % config.t as u128) as u64;

println!("Final Homomorphic Result: {}", final_result);
println!("Expected Result: {}", expected_result);

assert_eq!(final_result, expected_result);
println!("Deep circuit (depth 4) computed successfully!");
```

---

## 4. Security and Error Handling in Practice

NINE65's API is designed to guide developers toward secure and robust implementations.

### Secure Methods

For production, always use methods with a `_secure` suffix (e.g., `encrypt_secure`, `generate_secure`). These methods are guaranteed to use a cryptographically secure random number generator (CSPRNG) from the underlying operating system.

### Fallible Operations

Any operation that can potentially fail (e.g., encoding a message that is too large) returns a `Nine65Result<T>`. This enforces explicit error handling and prevents unexpected panics.

```rust
// Attempt to encode a message that is larger than the plaintext modulus `t`.
let large_message = config.t + 1;
let encode_result = encoder.try_encode(large_message);

match encode_result {
    Ok(_) => panic!("This should have failed!"),
    Err(Nine65Error::MessageOutOfBounds { message, modulus }) => {
        println!("Correctly caught expected error: Message {} is out of bounds for modulus {}.",
                 message, modulus);
    }
    Err(e) => panic!("Caught an unexpected error type: {:?}", e),
}
```

---

## 5. Summary and Conclusion

This walkthrough has demonstrated the core, fully functional capabilities of the NINE65 FHE library. Developers can confidently use the system for:

- **Setting up** a secure FHE environment with 128-bit+ security
- **Performing** basic encryption, decryption, addition, subtraction, and multiplication
- **Executing** deep, multi-level homomorphic computations in symmetric mode without the need for bootstrapping
- **Implementing** robust applications with built-in secure defaults and comprehensive error handling

The primary limitation to be aware of is that the deep circuit capability is currently optimized for **symmetric mode**. Public key operations are fully functional but are best suited for shallower circuits with the existing parameter sets.

NINE65 provides a powerful and efficient toolset for privacy-preserving computation, offering a unique and practical solution for deep circuit evaluation on standard CPU hardware.

---

## 6. References

1. Homomorphic Encryption Standardization. (2024). *Homomorphic Encryption Standard*. https://homomorphicencryption.org/
2. Ahmad, J., et al. (2025). *Cross-Platform Benchmarking of the FHE Libraries*. ePrint Archive. https://eprint.iacr.org/2025/473
3. Pambudi, D. S., et al. (2025). *A Benchmark Study of SEAL, HElib, OpenFHE, and Lattigo*. ACM Digital Library. https://dl.acm.org/doi/10.1145/3729706.3729711

---

## Next Steps

- [API Reference](/03_API_REFERENCE/) - Detailed documentation of all NINE65 types and functions
- [Advanced Tutorials](/08_TUTORIALS/advanced/) - Batching, neural networks, and RNS operations
- [Benchmarks](/05_BENCHMARKS/) - Performance characteristics and comparisons
