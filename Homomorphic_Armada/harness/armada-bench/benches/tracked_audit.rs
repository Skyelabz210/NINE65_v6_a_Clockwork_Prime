//! # Tracked Audit Benchmark
//!
//! Uses the `TrackedFhe` wrapper to collect fine-grained boundary metrics
//! during a realistic workload, then exports canonical reports and CSV data.
//!
//! Unlike `cross_build.rs` (which uses criterion for statistical analysis),
//! this benchmark uses the built-in `MetricsCollector` for per-operation
//! audit trails. Useful for:
//!
//! - Identifying outliers and latency spikes
//! - Collecting operation-count vs wall-clock data
//! - Exporting CSV for external analysis tools
//!
//! ## Usage
//!
//! ```bash
//! cargo bench -p armada-bench --bench tracked_audit --features v5_live
//! ```

use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;

#[cfg(any(
    feature = "v01_original",
    feature = "v02_stable",
    feature = "v04_qclassic",
    feature = "v5_live",
))]
mod tracked_benches {
    use super::*;
    use armada_shim::{ArmadaFHE, CurrentFhe, TrackedFhe};

    pub fn bench_tracked_workload(c: &mut Criterion) {
        let tracked = TrackedFhe::<CurrentFhe>::setup_light_tracked();

        let mut group = c.benchmark_group("tracked_workload");
        group.sample_size(20);

        // Realistic workload: encrypt N values, add them pairwise, decrypt results
        group.bench_function("pipeline_16_values", |b| {
            b.iter(|| {
                tracked.reset_metrics();

                // Encrypt 16 values
                let cts: Vec<_> = (0u64..16).map(|v| tracked.encrypt(black_box(v))).collect();

                // Pairwise add (8 results)
                let sums: Vec<_> = cts.chunks(2)
                    .map(|pair| tracked.add(&pair[0], &pair[1]))
                    .collect();

                // Pairwise mul (4 results)
                let prods: Vec<_> = sums.chunks(2)
                    .map(|pair| tracked.mul(&pair[0], &pair[1]))
                    .collect();

                // Decrypt all
                let results: Vec<_> = prods.iter()
                    .map(|ct| tracked.decrypt(ct))
                    .collect();

                black_box(results)
            })
        });

        group.finish();

        // Final metrics report
        tracked.report();

        // Export CSV to stdout (can redirect to file)
        let csv = tracked.to_csv();
        if csv.lines().count() > 1 {
            eprintln!("\n--- Tracked CSV ({} records) ---", csv.lines().count() - 1);
            eprintln!("{}", &csv[..csv.len().min(2000)]);
        }
    }
}

#[cfg(any(
    feature = "v01_original",
    feature = "v02_stable",
    feature = "v04_qclassic",
    feature = "v5_live",
))]
criterion_group!(tracked, tracked_benches::bench_tracked_workload);

#[cfg(any(
    feature = "v01_original",
    feature = "v02_stable",
    feature = "v04_qclassic",
    feature = "v5_live",
))]
criterion_main!(tracked);

// Stub for non-FHE features
#[cfg(not(any(
    feature = "v01_original",
    feature = "v02_stable",
    feature = "v04_qclassic",
    feature = "v5_live",
)))]
fn main() {
    eprintln!("tracked_audit requires an FHE build feature (v01/v02/v04/v5)");
}
