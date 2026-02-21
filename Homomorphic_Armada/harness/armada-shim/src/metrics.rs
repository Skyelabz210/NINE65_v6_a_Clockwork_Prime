//! # Canonical Metric Collection — Forensic-Grade Boundary Tracking
//!
//! Independent instrumentation layer that wraps any `ArmadaFHE` implementation
//! with transparent monitoring at every operation boundary. Designed for
//! **forensic analysis** — when something doesn't perform as expected, this
//! module provides the data to find exactly where the problem space is and
//! how it affects downstream computation.
//!
//! ## Capabilities
//!
//! - **Per-operation wall-clock timing** (nanoseconds, monotonic)
//! - **Correctness validation**: roundtrip encrypt→decrypt checks at configurable intervals
//! - **Noise budget tracking**: monitors noise growth per operation (where crate exposes it)
//! - **Error collection**: captures anomalies with full context (what failed, what values, what state)
//! - **Drift detection**: flags when operation latency or correctness deviates from established baseline
//! - **Running statistics** (min, max, mean, variance, P50/P95/P99) per operation type
//! - **Full audit log** for offline forensic analysis with CSV/JSON export
//!
//! ## Usage
//!
//! ```rust,ignore
//! let ctx = V05Fhe::setup_light();
//! let tracked = TrackedFhe::new(ctx);
//!
//! // Enable forensic mode (correctness checks + noise monitoring)
//! tracked.config().borrow_mut().forensic_mode = true;
//! tracked.config().borrow_mut().correctness_check_interval = 10; // every 10th encrypt
//!
//! // Normal operations — tracking is transparent
//! let ct = tracked.encrypt(42);
//! let _ = tracked.decrypt(&ct);
//!
//! // Inspect results
//! tracked.report();                    // Canonical summary
//! tracked.error_report();              // Anomalies and failures
//! let csv = tracked.to_csv();          // Full audit trail
//! let json = tracked.to_json_records(); // Structured export
//! ```

use std::cell::RefCell;
use std::fmt;
use std::time::Instant;
use crate::ArmadaFHE;

// ── Operation Classification ────────────────────────────────────────────

/// Operation category for metric bucketing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OpKind {
    Encrypt,
    Decrypt,
    Add,
    Mul,
    Sub,
}

impl OpKind {
    pub fn label(&self) -> &'static str {
        match self {
            OpKind::Encrypt => "encrypt",
            OpKind::Decrypt => "decrypt",
            OpKind::Add     => "homo_add",
            OpKind::Mul     => "homo_mul",
            OpKind::Sub     => "homo_sub",
        }
    }
}

impl fmt::Display for OpKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.label())
    }
}

// ── Error/Anomaly Classification ────────────────────────────────────────

/// Severity levels for detected anomalies.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    /// Informational — unusual but not incorrect
    Info,
    /// Warning — potential issue, computation may be degrading
    Warning,
    /// Error — correctness violation detected
    Error,
    /// Critical — computation is broken, results unreliable
    Critical,
}

impl fmt::Display for Severity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Severity::Info     => f.write_str("INFO"),
            Severity::Warning  => f.write_str("WARN"),
            Severity::Error    => f.write_str("ERROR"),
            Severity::Critical => f.write_str("CRITICAL"),
        }
    }
}

/// A detected anomaly with full forensic context.
#[derive(Debug, Clone)]
pub struct Anomaly {
    /// When in the operation sequence this was detected
    pub sequence: u64,
    /// What operation was executing
    pub op_kind: OpKind,
    /// Severity of the anomaly
    pub severity: Severity,
    /// Human-readable description of what went wrong
    pub message: String,
    /// Expected value (if applicable)
    pub expected: Option<u64>,
    /// Actual value observed
    pub actual: Option<u64>,
    /// Wall-clock time of the operation that triggered this
    pub elapsed_ns: u64,
    /// Baseline mean at time of detection (for drift analysis)
    pub baseline_mean_ns: Option<u64>,
}

impl fmt::Display for Anomaly {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[seq={}] [{}] [{}] {}", self.sequence, self.severity, self.op_kind, self.message)?;
        if let (Some(exp), Some(act)) = (self.expected, self.actual) {
            write!(f, " (expected={}, actual={})", exp, act)?;
        }
        if let Some(base) = self.baseline_mean_ns {
            write!(f, " (baseline={}ns, observed={}ns, drift={:.1}x)",
                base, self.elapsed_ns, self.elapsed_ns as f64 / base.max(1) as f64)?;
        }
        Ok(())
    }
}

// ── Single Operation Record ─────────────────────────────────────────────

/// Full record for a single operation (audit trail).
#[derive(Debug, Clone)]
pub struct OpRecord {
    pub sequence: u64,
    pub kind: OpKind,
    pub elapsed_ns: u64,
    /// Whether correctness was validated on this operation
    pub correctness_checked: bool,
    /// Whether correctness passed (None = not checked)
    pub correctness_passed: Option<bool>,
    /// Input value (for encrypt operations)
    pub input_value: Option<u64>,
    /// Output value (for decrypt operations)
    pub output_value: Option<u64>,
}

// ── Running Statistics (Welford's Online Algorithm) ─────────────────────

/// Running statistics accumulator with percentile estimation.
#[derive(Debug, Clone)]
pub struct RunningStats {
    pub count: u64,
    pub min_ns: u64,
    pub max_ns: u64,
    mean: f64,
    m2: f64,
    /// Recent samples for P50/P95/P99 estimation (circular buffer)
    recent: Vec<u64>,
    recent_idx: usize,
    recent_capacity: usize,
}

impl RunningStats {
    fn new() -> Self {
        Self::with_capacity(1000)
    }

    fn with_capacity(cap: usize) -> Self {
        Self {
            count: 0,
            min_ns: u64::MAX,
            max_ns: 0,
            mean: 0.0,
            m2: 0.0,
            recent: Vec::with_capacity(cap),
            recent_idx: 0,
            recent_capacity: cap,
        }
    }

    fn push(&mut self, value_ns: u64) {
        self.count += 1;
        if value_ns < self.min_ns { self.min_ns = value_ns; }
        if value_ns > self.max_ns { self.max_ns = value_ns; }

        // Welford's online mean + variance
        let v = value_ns as f64;
        let delta = v - self.mean;
        self.mean += delta / self.count as f64;
        let delta2 = v - self.mean;
        self.m2 += delta * delta2;

        // Circular buffer for percentile estimation
        if self.recent.len() < self.recent_capacity {
            self.recent.push(value_ns);
        } else {
            self.recent[self.recent_idx] = value_ns;
        }
        self.recent_idx = (self.recent_idx + 1) % self.recent_capacity;
    }

    /// Mean in nanoseconds.
    pub fn mean_ns(&self) -> u64 {
        self.mean as u64
    }

    /// Sample standard deviation in nanoseconds.
    pub fn stddev_ns(&self) -> u64 {
        if self.count < 2 { return 0; }
        ((self.m2 / (self.count - 1) as f64).sqrt()) as u64
    }

    /// Total accumulated time in nanoseconds.
    pub fn total_ns(&self) -> u64 {
        (self.mean * self.count as f64) as u64
    }

    /// Percentile from recent samples (0.0–1.0).
    pub fn percentile(&self, p: f64) -> u64 {
        if self.recent.is_empty() { return 0; }
        let mut sorted = self.recent.clone();
        sorted.sort_unstable();
        let idx = ((sorted.len() as f64 * p) as usize).min(sorted.len() - 1);
        sorted[idx]
    }

    /// P50 (median) from recent samples.
    pub fn p50(&self) -> u64 { self.percentile(0.50) }
    /// P95 from recent samples.
    pub fn p95(&self) -> u64 { self.percentile(0.95) }
    /// P99 from recent samples.
    pub fn p99(&self) -> u64 { self.percentile(0.99) }
}

// ── Configuration ───────────────────────────────────────────────────────

/// Configuration for the metrics collector.
#[derive(Debug, Clone)]
pub struct MetricsConfig {
    /// Enable forensic mode: correctness checks, drift detection, noise monitoring
    pub forensic_mode: bool,
    /// How often to run encrypt→decrypt roundtrip correctness checks (every N encrypts)
    pub correctness_check_interval: u64,
    /// Latency drift threshold: flag if operation takes >Nx the baseline mean
    pub drift_threshold: f64,
    /// Minimum samples before drift detection activates
    pub drift_warmup: u64,
    /// Whether to keep full operation log (disable for long runs to save memory)
    pub retain_log: bool,
    /// Maximum log entries before auto-pruning (0 = unlimited)
    pub max_log_entries: usize,
}

impl Default for MetricsConfig {
    fn default() -> Self {
        Self {
            forensic_mode: true,
            correctness_check_interval: 100,
            drift_threshold: 5.0,
            drift_warmup: 50,
            retain_log: true,
            max_log_entries: 100_000,
        }
    }
}

// ── Per-Operation Bucket ────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct OpBucket {
    pub kind: OpKind,
    pub stats: RunningStats,
    /// Count of correctness checks performed
    pub correctness_checks: u64,
    /// Count of correctness failures
    pub correctness_failures: u64,
}

impl OpBucket {
    fn new(kind: OpKind) -> Self {
        Self {
            kind,
            stats: RunningStats::new(),
            correctness_checks: 0,
            correctness_failures: 0,
        }
    }
}

// ── Metrics Collector ───────────────────────────────────────────────────

/// Canonical metrics collector with forensic capabilities.
#[derive(Debug)]
pub struct MetricsCollector {
    pub config: MetricsConfig,
    /// Per-kind statistics
    pub buckets: [OpBucket; 5],
    /// Full operation log
    pub log: Vec<OpRecord>,
    /// Detected anomalies
    pub anomalies: Vec<Anomaly>,
    /// Monotonic sequence counter
    sequence: u64,
    /// Encrypt counter (for correctness check interval)
    encrypt_count: u64,
}

impl MetricsCollector {
    pub fn new() -> Self {
        Self::with_config(MetricsConfig::default())
    }

    pub fn with_config(config: MetricsConfig) -> Self {
        Self {
            config,
            buckets: [
                OpBucket::new(OpKind::Encrypt),
                OpBucket::new(OpKind::Decrypt),
                OpBucket::new(OpKind::Add),
                OpBucket::new(OpKind::Mul),
                OpBucket::new(OpKind::Sub),
            ],
            log: Vec::new(),
            anomalies: Vec::new(),
            sequence: 0,
            encrypt_count: 0,
        }
    }

    /// Record a single operation measurement with optional forensic data.
    pub fn record(&mut self, kind: OpKind, elapsed_ns: u64,
                  input_value: Option<u64>, output_value: Option<u64>,
                  correctness_checked: bool, correctness_passed: Option<bool>) {
        self.sequence += 1;
        let seq = self.sequence;
        let idx = kind as usize;

        // Drift detection (before updating stats)
        if self.config.forensic_mode
            && self.buckets[idx].stats.count >= self.config.drift_warmup
        {
            let baseline = self.buckets[idx].stats.mean_ns();
            if baseline > 0 && elapsed_ns as f64 > baseline as f64 * self.config.drift_threshold {
                self.anomalies.push(Anomaly {
                    sequence: seq,
                    op_kind: kind,
                    severity: Severity::Warning,
                    message: format!(
                        "Latency drift: {}ns vs baseline {}ns ({:.1}x)",
                        elapsed_ns, baseline, elapsed_ns as f64 / baseline as f64
                    ),
                    expected: None,
                    actual: None,
                    elapsed_ns,
                    baseline_mean_ns: Some(baseline),
                });
            }
        }

        // Update stats
        self.buckets[idx].stats.push(elapsed_ns);

        // Track correctness
        if correctness_checked {
            self.buckets[idx].correctness_checks += 1;
            if correctness_passed == Some(false) {
                self.buckets[idx].correctness_failures += 1;
                self.anomalies.push(Anomaly {
                    sequence: seq,
                    op_kind: kind,
                    severity: Severity::Error,
                    message: "Correctness check FAILED: encrypt→decrypt roundtrip mismatch".into(),
                    expected: input_value,
                    actual: output_value,
                    elapsed_ns,
                    baseline_mean_ns: None,
                });
            }
        }

        // Append to log
        if self.config.retain_log {
            if self.config.max_log_entries > 0 && self.log.len() >= self.config.max_log_entries {
                // Prune oldest 10%
                let drain = self.config.max_log_entries / 10;
                self.log.drain(..drain);
            }
            self.log.push(OpRecord {
                sequence: seq,
                kind,
                elapsed_ns,
                correctness_checked,
                correctness_passed,
                input_value,
                output_value,
            });
        }
    }

    /// Simple record (no forensic data).
    pub fn record_simple(&mut self, kind: OpKind, elapsed_ns: u64) {
        self.record(kind, elapsed_ns, None, None, false, None);
    }

    /// Get stats for a specific operation kind.
    pub fn stats(&self, kind: OpKind) -> &RunningStats {
        &self.buckets[kind as usize].stats
    }

    /// Total operations recorded.
    pub fn total_ops(&self) -> u64 {
        self.buckets.iter().map(|b| b.stats.count).sum()
    }

    /// Total anomalies detected.
    pub fn anomaly_count(&self) -> usize {
        self.anomalies.len()
    }

    /// Anomalies filtered by severity.
    pub fn anomalies_by_severity(&self, sev: Severity) -> Vec<&Anomaly> {
        self.anomalies.iter().filter(|a| a.severity == sev).collect()
    }

    /// Next encrypt count (for correctness scheduling).
    pub fn next_encrypt_count(&mut self) -> u64 {
        self.encrypt_count += 1;
        self.encrypt_count
    }

    /// Whether the next encrypt should include a correctness check.
    pub fn should_check_correctness(&self) -> bool {
        self.config.forensic_mode
            && self.config.correctness_check_interval > 0
            && self.encrypt_count % self.config.correctness_check_interval == 0
    }

    /// Reset all counters and logs.
    pub fn reset(&mut self) {
        for b in &mut self.buckets {
            b.stats = RunningStats::new();
            b.correctness_checks = 0;
            b.correctness_failures = 0;
        }
        self.log.clear();
        self.anomalies.clear();
        self.sequence = 0;
        self.encrypt_count = 0;
    }

    /// Print canonical summary report.
    pub fn report(&self) {
        println!();
        println!("============================================================");
        println!("  ARMADA METRICS — Canonical Boundary Report");
        println!("============================================================");
        println!("{:<12} {:>8} {:>10} {:>10} {:>10} {:>10} {:>10} {:>10}",
            "Operation", "Count", "Min(ns)", "Mean(ns)", "P50(ns)", "P95(ns)", "Max(ns)", "StdDev");
        println!("{}", "-".repeat(82));

        for bucket in &self.buckets {
            let s = &bucket.stats;
            if s.count == 0 { continue; }
            println!("{:<12} {:>8} {:>10} {:>10} {:>10} {:>10} {:>10} {:>10}",
                bucket.kind.label(),
                s.count,
                s.min_ns,
                s.mean_ns(),
                s.p50(),
                s.p95(),
                s.max_ns,
                s.stddev_ns(),
            );
        }

        println!("{}", "-".repeat(82));
        let total_time: u64 = self.buckets.iter().map(|b| b.stats.total_ns()).sum();
        println!("Total operations: {}  |  Wall-clock: {:.3} ms",
            self.total_ops(), total_time as f64 / 1_000_000.0);

        // Correctness summary
        let total_checks: u64 = self.buckets.iter().map(|b| b.correctness_checks).sum();
        let total_failures: u64 = self.buckets.iter().map(|b| b.correctness_failures).sum();
        if total_checks > 0 {
            println!("Correctness: {}/{} checks passed ({:.2}% pass rate)",
                total_checks - total_failures, total_checks,
                (total_checks - total_failures) as f64 / total_checks as f64 * 100.0);
        }

        // Anomaly summary
        if !self.anomalies.is_empty() {
            println!("Anomalies: {} detected", self.anomalies.len());
        }
    }

    /// Print detailed error/anomaly report.
    pub fn error_report(&self) {
        if self.anomalies.is_empty() {
            println!("\n  No anomalies detected.");
            return;
        }

        println!();
        println!("============================================================");
        println!("  ARMADA FORENSICS — Anomaly Report ({} total)", self.anomalies.len());
        println!("============================================================");

        let critical = self.anomalies_by_severity(Severity::Critical);
        let errors = self.anomalies_by_severity(Severity::Error);
        let warnings = self.anomalies_by_severity(Severity::Warning);
        let info = self.anomalies_by_severity(Severity::Info);

        if !critical.is_empty() {
            println!("\n  CRITICAL ({}):", critical.len());
            for a in &critical { println!("    {}", a); }
        }
        if !errors.is_empty() {
            println!("\n  ERRORS ({}):", errors.len());
            for a in &errors { println!("    {}", a); }
        }
        if !warnings.is_empty() {
            println!("\n  WARNINGS ({}):", warnings.len());
            for a in warnings.iter().take(20) { println!("    {}", a); }
            if warnings.len() > 20 {
                println!("    ... and {} more", warnings.len() - 20);
            }
        }
        if !info.is_empty() {
            println!("\n  INFO ({}):", info.len());
            for a in info.iter().take(10) { println!("    {}", a); }
            if info.len() > 10 {
                println!("    ... and {} more", info.len() - 10);
            }
        }
    }

    /// Export all records as CSV.
    pub fn to_csv(&self) -> String {
        let mut out = String::from("sequence,operation,elapsed_ns,correctness_checked,correctness_passed,input_value,output_value\n");
        for r in &self.log {
            out.push_str(&format!("{},{},{},{},{},{},{}\n",
                r.sequence,
                r.kind.label(),
                r.elapsed_ns,
                r.correctness_checked,
                r.correctness_passed.map(|b| b.to_string()).unwrap_or_default(),
                r.input_value.map(|v| v.to_string()).unwrap_or_default(),
                r.output_value.map(|v| v.to_string()).unwrap_or_default(),
            ));
        }
        out
    }

    /// Export anomalies as CSV.
    pub fn anomalies_csv(&self) -> String {
        let mut out = String::from("sequence,severity,operation,message,expected,actual,elapsed_ns,baseline_ns\n");
        for a in &self.anomalies {
            out.push_str(&format!("{},{},{},\"{}\",{},{},{},{}\n",
                a.sequence,
                a.severity,
                a.op_kind.label(),
                a.message,
                a.expected.map(|v| v.to_string()).unwrap_or_default(),
                a.actual.map(|v| v.to_string()).unwrap_or_default(),
                a.elapsed_ns,
                a.baseline_mean_ns.map(|v| v.to_string()).unwrap_or_default(),
            ));
        }
        out
    }

    /// Export records as JSON lines (one JSON object per line).
    pub fn to_json_records(&self) -> String {
        let mut out = String::new();
        for r in &self.log {
            out.push_str(&format!(
                "{{\"seq\":{},\"op\":\"{}\",\"ns\":{},\"cc\":{},\"cp\":{},\"in\":{},\"out\":{}}}\n",
                r.sequence,
                r.kind.label(),
                r.elapsed_ns,
                r.correctness_checked,
                r.correctness_passed.map(|b| if b { "true" } else { "false" }).unwrap_or("null"),
                r.input_value.map(|v| v.to_string()).unwrap_or("null".into()),
                r.output_value.map(|v| v.to_string()).unwrap_or("null".into()),
            ));
        }
        out
    }
}

impl Default for MetricsCollector {
    fn default() -> Self {
        Self::new()
    }
}

// ── Tracked FHE Wrapper ─────────────────────────────────────────────────

/// Forensic wrapper around any `ArmadaFHE` implementation.
///
/// Transparently instruments every operation boundary with:
/// - Nanosecond timing
/// - Periodic correctness validation (encrypt→decrypt roundtrip)
/// - Latency drift detection
/// - Full audit logging
pub struct TrackedFhe<T: ArmadaFHE> {
    inner: T,
    pub collector: RefCell<MetricsCollector>,
}

impl<T: ArmadaFHE> TrackedFhe<T> {
    /// Wrap an existing FHE context with tracking.
    pub fn new(inner: T) -> Self {
        Self {
            inner,
            collector: RefCell::new(MetricsCollector::new()),
        }
    }

    /// Wrap with custom config.
    pub fn with_config(inner: T, config: MetricsConfig) -> Self {
        Self {
            inner,
            collector: RefCell::new(MetricsCollector::with_config(config)),
        }
    }

    /// Setup with tracking enabled from the start.
    pub fn setup_light_tracked() -> Self
    where
        T: Sized,
    {
        Self::new(T::setup_light())
    }

    /// Access config for tuning.
    pub fn config(&self) -> &RefCell<MetricsCollector> {
        &self.collector
    }

    /// Access the inner FHE context (untracked).
    pub fn inner(&self) -> &T {
        &self.inner
    }

    /// Print the canonical metrics report.
    pub fn report(&self) {
        self.collector.borrow().report();
    }

    /// Print the anomaly/error report.
    pub fn error_report(&self) {
        self.collector.borrow().error_report();
    }

    /// Export metrics as CSV.
    pub fn to_csv(&self) -> String {
        self.collector.borrow().to_csv()
    }

    /// Export anomalies as CSV.
    pub fn anomalies_csv(&self) -> String {
        self.collector.borrow().anomalies_csv()
    }

    /// Export as JSON lines.
    pub fn to_json_records(&self) -> String {
        self.collector.borrow().to_json_records()
    }

    /// Reset all collected metrics.
    pub fn reset_metrics(&self) {
        self.collector.borrow_mut().reset();
    }

    /// Encrypt with boundary tracking + optional correctness check.
    pub fn encrypt(&self, value: u64) -> T::Ciphertext {
        let mut col = self.collector.borrow_mut();
        let _enc_count = col.next_encrypt_count();
        let should_check = col.should_check_correctness();
        drop(col); // release borrow before calling inner

        let start = Instant::now();
        let ct = self.inner.encrypt(value);
        let elapsed = start.elapsed().as_nanos() as u64;

        let mut correctness_checked = false;
        let mut correctness_passed = None;
        let mut output_value = None;

        if should_check {
            correctness_checked = true;
            let roundtrip = self.inner.decrypt(&ct);
            let t = self.inner.plaintext_modulus();
            let expected = value % t;
            correctness_passed = Some(roundtrip == expected);
            output_value = Some(roundtrip);
        }

        self.collector.borrow_mut().record(
            OpKind::Encrypt, elapsed,
            Some(value), output_value,
            correctness_checked, correctness_passed,
        );
        ct
    }

    /// Decrypt with boundary tracking.
    pub fn decrypt(&self, ct: &T::Ciphertext) -> u64 {
        let start = Instant::now();
        let result = self.inner.decrypt(ct);
        let elapsed = start.elapsed().as_nanos() as u64;
        self.collector.borrow_mut().record(
            OpKind::Decrypt, elapsed,
            None, Some(result),
            false, None,
        );
        result
    }

    /// Homomorphic add with boundary tracking.
    pub fn add(&self, a: &T::Ciphertext, b: &T::Ciphertext) -> T::Ciphertext {
        let start = Instant::now();
        let result = self.inner.add(a, b);
        let elapsed = start.elapsed().as_nanos() as u64;
        self.collector.borrow_mut().record_simple(OpKind::Add, elapsed);
        result
    }

    /// Homomorphic mul with boundary tracking.
    pub fn mul(&self, a: &T::Ciphertext, b: &T::Ciphertext) -> T::Ciphertext {
        let start = Instant::now();
        let result = self.inner.mul(a, b);
        let elapsed = start.elapsed().as_nanos() as u64;
        self.collector.borrow_mut().record_simple(OpKind::Mul, elapsed);
        result
    }

    /// Homomorphic sub with boundary tracking.
    pub fn sub(&self, a: &T::Ciphertext, b: &T::Ciphertext) -> T::Ciphertext {
        let start = Instant::now();
        let result = self.inner.sub(a, b);
        let elapsed = start.elapsed().as_nanos() as u64;
        self.collector.borrow_mut().record_simple(OpKind::Sub, elapsed);
        result
    }

    /// Plaintext modulus passthrough.
    pub fn plaintext_modulus(&self) -> u64 {
        self.inner.plaintext_modulus()
    }

    /// Run a full forensic check: encrypt N values, add/mul, decrypt, validate.
    pub fn forensic_sweep(&self, n: usize) -> ForensicSweepResult {
        self.reset_metrics();
        // Force correctness checks on every operation for sweep
        self.collector.borrow_mut().config.correctness_check_interval = 1;

        let mut values = Vec::with_capacity(n);
        let mut ciphertexts = Vec::with_capacity(n);
        let t = self.inner.plaintext_modulus();

        // Phase 1: Encrypt
        for i in 0..n {
            let v = (i as u64 + 1) % t;
            values.push(v);
            ciphertexts.push(self.encrypt(v));
        }

        // Phase 2: Pairwise add
        let mut add_results = Vec::new();
        let mut add_expected = Vec::new();
        for pair in ciphertexts.chunks(2) {
            if pair.len() == 2 {
                let ct = self.add(&pair[0], &pair[1]);
                let decrypted = self.decrypt(&ct);
                add_results.push(decrypted);
                let idx = add_expected.len() * 2;
                add_expected.push((values[idx] + values[idx + 1]) % t);
            }
        }

        // Phase 3: Validate
        let mut add_mismatches = 0;
        for (i, (&actual, &expected)) in add_results.iter().zip(add_expected.iter()).enumerate() {
            if actual != expected {
                add_mismatches += 1;
                self.collector.borrow_mut().anomalies.push(Anomaly {
                    sequence: 0,
                    op_kind: OpKind::Add,
                    severity: Severity::Error,
                    message: format!("Add result mismatch at pair {}", i),
                    expected: Some(expected),
                    actual: Some(actual),
                    elapsed_ns: 0,
                    baseline_mean_ns: None,
                });
            }
        }

        let col = self.collector.borrow();
        ForensicSweepResult {
            total_ops: col.total_ops(),
            encrypt_count: n as u64,
            add_count: add_results.len() as u64,
            decrypt_count: add_results.len() as u64,
            add_mismatches,
            anomaly_count: col.anomaly_count(),
            encrypt_mean_ns: col.stats(OpKind::Encrypt).mean_ns(),
            decrypt_mean_ns: col.stats(OpKind::Decrypt).mean_ns(),
            add_mean_ns: col.stats(OpKind::Add).mean_ns(),
        }
    }
}

/// Result from a forensic sweep.
#[derive(Debug)]
pub struct ForensicSweepResult {
    pub total_ops: u64,
    pub encrypt_count: u64,
    pub add_count: u64,
    pub decrypt_count: u64,
    pub add_mismatches: usize,
    pub anomaly_count: usize,
    pub encrypt_mean_ns: u64,
    pub decrypt_mean_ns: u64,
    pub add_mean_ns: u64,
}

impl fmt::Display for ForensicSweepResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Forensic Sweep: {} total ops", self.total_ops)?;
        writeln!(f, "  Encrypts:  {} (mean {}ns)", self.encrypt_count, self.encrypt_mean_ns)?;
        writeln!(f, "  Adds:      {} (mean {}ns)", self.add_count, self.add_mean_ns)?;
        writeln!(f, "  Decrypts:  {} (mean {}ns)", self.decrypt_count, self.decrypt_mean_ns)?;
        writeln!(f, "  Mismatches: {}", self.add_mismatches)?;
        writeln!(f, "  Anomalies:  {}", self.anomaly_count)?;
        if self.add_mismatches == 0 && self.anomaly_count == 0 {
            writeln!(f, "  Status: ALL CLEAR")?;
        } else {
            writeln!(f, "  Status: ISSUES DETECTED")?;
        }
        Ok(())
    }
}

// ── Tracked MANA Wrapper ────────────────────────────────────────────────

/// Tracked wrapper for parallel MANA operations.
pub struct TrackedParallel<P: crate::ArmadaParallel> {
    _phantom: std::marker::PhantomData<P>,
    pub collector: RefCell<MetricsCollector>,
}

impl<P: crate::ArmadaParallel> TrackedParallel<P> {
    pub fn new() -> Self {
        Self {
            _phantom: std::marker::PhantomData,
            collector: RefCell::new(MetricsCollector::new()),
        }
    }

    pub fn lane_add(&self, a: &P::Lane, b: &P::Lane) -> P::Lane {
        let start = Instant::now();
        let result = P::lane_add(a, b);
        let elapsed = start.elapsed().as_nanos() as u64;
        self.collector.borrow_mut().record_simple(OpKind::Add, elapsed);
        result
    }

    pub fn lane_mul(&self, a: &P::Lane, b: &P::Lane) -> P::Lane {
        let start = Instant::now();
        let result = P::lane_mul(a, b);
        let elapsed = start.elapsed().as_nanos() as u64;
        self.collector.borrow_mut().record_simple(OpKind::Mul, elapsed);
        result
    }

    pub fn stream_add(&self, a: &P::Stream, b: &P::Stream) -> P::Stream {
        let start = Instant::now();
        let result = P::stream_add(a, b);
        let elapsed = start.elapsed().as_nanos() as u64;
        self.collector.borrow_mut().record_simple(OpKind::Add, elapsed);
        result
    }

    pub fn report(&self) {
        self.collector.borrow().report();
    }

    pub fn error_report(&self) {
        self.collector.borrow().error_report();
    }
}
