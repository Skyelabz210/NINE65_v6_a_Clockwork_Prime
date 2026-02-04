# MOC: Architecture

> Auto-generated Map of Content for Architecture
> Last updated: 2025-12-16 21:05

## Overview

This MOC provides navigation for all architecture-related content in QMNF.

## Contents

### src

- [[crates/qmnf-arithmetic/src/kfree_crt.rs|kfree_crt]] - K-Free CRT: Phase-Locked Modular Geometry Implementation
//!
//! CRT arithmetic 
- [[crates/qmnf-arithmetic/src/plmg.rs|plmg]] - PLMG - Polyphonic Logarithmic Modular Gearing (G4-02)
//!
//! Adaptive multi-bas
- [[crates/qmnf-arithmetic/src/garner.rs|garner]] - Garner's Algorithm - CRT Reconstruction
//!
//! Efficient O(k²) reconstruction o
- [[crates/qmnf-arithmetic/src/nnt.rs|nnt]] - Number Theoretic Transform (NNT) - Integer-Only FFT
//!
//! Cooley-Tukey NNT imp
- [[crates/qmnf-arithmetic/src/dcbigint.rs|dcbigint]] - DCBigInt - Dual Codex BigInt (G3-05)
//!
//! Dual representation system that mai
- [[crates/qmnf-arithmetic/src/k_elimination.rs|k_elimination]] - K-Elimination Theorem - 60-Year RNS Division Breakthrough
//!
//! Enables **exac
- [[crates/qmnf-arithmetic/src/rational.rs|rational]] - Rational - Exact rational number arithmetic
//!
//! Unlimited-precision rational
- [[crates/qmnf-arithmetic/src/modint.rs|modint]] - ModInt - Modular arithmetic with Mersenne prime 2^31 - 1
//!
//! High-performanc
- [[crates/qmnf-arithmetic/src/lib.rs|lib]] - QMNF Arithmetic - Layer 1 Core Arithmetic
//!
//! This crate provides CRT-based 
- [[crates/qmnf-arithmetic/src/crt.rs|crt]] - CRTBigInt - Chinese Remainder Theorem based integers
//!
//! High-performance bo
- [[crates/qmnf-fhe/src/fhe_ahop.rs|fhe_ahop]] - FHE-AHOP - Attractor-Homomorphic Optimization Protocol (G6-01)
//!
//! FHE varia
- [[crates/qmnf-fhe/src/polynomial.rs|polynomial]] - Polynomial Ring Operations for FHE
//!
//! Implements polynomial arithmetic over
- [[crates/qmnf-fhe/src/params.rs|params]] - FHE Security Parameters and Configuration
//!
//! Defines security levels and cr
- [[crates/qmnf-fhe/src/lib.rs|lib]] - QMNF FHE - Fully Homomorphic Encryption Layer
//!
//! Integer-only FHE using Rin
- [[crates/qmnf-fhe/src/error.rs|error]] - Error types for FHE operations
//!
//! Production-grade error handling for all F
- [[crates/qmnf-fhe/src/fhe_plmg.rs|fhe_plmg]] - FHE-PLMG - PLMG-Accelerated Fully Homomorphic Encryption (G6-02)
//!
//! FHE var
- [[crates/qmnf-optimization/src/gso_swarm.rs|gso_swarm]] - GSO - Galactic Swarm Optimization (G5-01)
//!
//! Gravitationally-inspired optim
- [[crates/qmnf-optimization/src/pade.rs|pade]] - Padé Approximation - Rational Function Approximation (G5-02)
//!
//! Integer-onl
- [[crates/qmnf-optimization/src/lib.rs|lib]] - QMNF Optimization - GSO Swarm and Padé Approximation
//!
//! Integer-only optimi
- [[crates/qmnf-primitives/src/bigint.rs|bigint]] - HCVLangBigInt - Arbitrary precision signed integers
//!
//! This is a minimal, f
- [[crates/qmnf-primitives/src/lib.rs|lib]] - QMNF Primitives - Layer 0 Foundation
//!
//! This crate provides the absolute fo

