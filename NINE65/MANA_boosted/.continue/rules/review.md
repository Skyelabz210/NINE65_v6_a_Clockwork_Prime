---
invokable: true
---

Review this code for potential issues, including:

- **Integer-only arithmetic violations**: Ensure no f32/f64 usage in critical crypto paths; all arithmetic should be exact integer operations
- **Constant-time requirements**: Check for timing side-channel vulnerabilities in modular arithmetic and secret-dependent operations
- **Memory safety with FFI integration**: Verify proper memory management when interfacing with lower-level operations
- **K-Elimination exact division implementation**: Confirm correctness of the dual-RNS reconstruction and anchor verification mechanisms
- **Noise bound calculations**: Validate that GSO-FHE noise bounding correctly prevents overflow in deep circuits
- **Entropy source security**: Verify proper CSPRNG usage in key generation and randomization
- **Zeroize implementation**: Check that all key material and sensitive data is properly cleared when dropped
- **RNS modulus selection**: Validate that chosen moduli maintain security and correctness requirements
- **Performance optimizations**: Ensure SIMD and parallel operations don't compromise security
- **Parameter validation**: Confirm all FHE parameters are properly validated before use
- **Integer overflow protection**: Verify safe bounds checking in arithmetic operations
- **Cross-crate incompatibilities**: Check that feature flags and optional dependencies work correctly across the workspace

Provide specific, actionable feedback for improvements.