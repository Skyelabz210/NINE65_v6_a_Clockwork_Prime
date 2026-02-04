# SPANKY Phase 2 Integration To-Do

## Core Integration
- [ ] Wire `persistent_montgomery.py` into chaos/Lorenz paths used by MYSTIC/SPANKY (replace scalar ops).
- [ ] Replace remaining transcendentals with `pade_engine.py` in attractor + Liouville paths; tune tolerances.
- [ ] Align `spanky_forecaster` and `mystic_v3_production` entrypoints behind a single CLI/API with mode selection.

## Probability Layer (14–60 days)
- [ ] Integrate `liouville_evolver.py` into the main forecast flow; expose ensemble size, timestep, grid resolution.
- [ ] Add runtime conservation/error monitors (mass/energy-like invariants) with alerts when >0.1‰.
- [ ] Calibrate attractor basin priors/signatures using historical events (Harvey/Blanco/Camp Fire/Joplin).

## Performance & Verification
- [ ] Benchmark chaos step time before/after Persistent Montgomery (target ≥1.5×) and record results.
- [ ] Run long-horizon ensemble tests (float vs QMNF vs Liouville); document divergence day and accuracy bands.
- [ ] Add property-based tests for Poisson bracket symmetry and conservation using `mobius_int.py`.

## Data & FFI Hardening
- [ ] Define clean Rust/Python FFI boundary per `FFI_FORMULA_FOR_MYSTIC_PYTHON_INTEGRATION.md`; prep Rust stub for Monty/NTT.
- [ ] Add ingress validation/range checks on all API inputs to guarantee integer-safe scaling.
- [ ] Review cache/tiering policy for probability computations (set sensible defaults/caps).

## Ops & Delivery
- [ ] Expose unified forecast API/CLI: `trajectory (0–14d)`, `probability (14–60d)`, `attractor-alert`.
- [ ] Update runbooks/docs to cover SPANKY layers, new knobs, and monitoring points.
- [ ] Prep demo script: flood-basin early warning + 30-day Liouville run with conservation stats.
