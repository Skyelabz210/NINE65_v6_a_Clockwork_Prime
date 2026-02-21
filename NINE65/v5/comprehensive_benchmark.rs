use std::time::Instant;
use nine65::params::secure_configs::SecureConfig;
use nine65::ops::gso_fhe::*;

// Additional benchmarks for comprehensive analysis

/// Benchmark memory usage during FHE operations
#[test]
fn benchmark_memory_usage() {
    println!("\n┌────────────────────────────────────────────────────────────┐");
    println!("│  MEMORY USAGE ANALYSIS - secure_128                        │");
    println!("└────────────────────────────────────────────────────────────┘");
    
    let config = SecureConfig::secure_128().into_config();
    let mut ctx = bench_ctx(config);
    let mut rng = ShadowHarvester::new();
    let keys = ctx.generate_keys(&mut rng);

    // Measure memory before operations
    let start_memory = get_current_rss_kb().unwrap_or(0);
    
    // Perform a series of operations
    let mut cts = Vec::new();
    for i in 0..10 {
        let ct = ctx.encrypt(i, &keys.public_key, &mut rng);
        cts.push(ct);
    }
    
    // Perform operations
    for i in 0..9 {
        let result = ctx.add(&cts[i], &cts[i+1]);
        cts[i+1] = result;
    }
    
    let end_memory = get_current_rss_kb().unwrap_or(0);
    let memory_used = end_memory - start_memory;
    
    println!("Memory used for 10 encrypt + 9 add operations: {} KB", memory_used);
    println!("Estimated memory per ciphertext: {} KB", memory_used / 10);
}

/// Benchmark noise growth over multiple operations
#[test]
fn benchmark_noise_growth() {
    println!("\n┌────────────────────────────────────────────────────────────┐");
    println!("│  NOISE GROWTH ANALYSIS - secure_128                        │");
    println!("└────────────────────────────────────────────────────────────┘");
    
    let config = SecureConfig::secure_128().into_config();
    let mut ctx = bench_ctx(config);
    let mut rng = ShadowHarvester::new();
    let keys = ctx.generate_keys(&mut rng);

    let mut ct = ctx.encrypt(2, &keys.public_key, &mut rng);
    
    println!("Operation | Noise Level | Collapses | Time (ms)");
    println!("----------|-------------|-----------|----------");
    
    for i in 1..=20 {
        let start = Instant::now();
        let ct_clone = ct.clone();
        ct = ctx.mul(&ct, &ct_clone, &keys.eval_key);
        let elapsed = start.elapsed().as_millis() as f64 / 1_000.0;
        
        let stats = ctx.noise_stats(&ct);
        println!("{:9} | {:11.2} | {:9} | {:7.2}", 
                 format!("Mul #{}", i), 
                 stats.noise_level, 
                 stats.collapses, 
                 elapsed);
    }
}

/// Benchmark parallel operations to identify scalability issues
#[test]
fn benchmark_parallel_operations() {
    println!("\n┌────────────────────────────────────────────────────────────┐");
    println!("│  PARALLEL OPERATION SCALABILITY - secure_128               │");
    println!("└────────────────────────────────────────────────────────────┘");
    
    use rayon::prelude::*;
    
    let config = SecureConfig::secure_128().into_config();
    let ctx_template = bench_ctx(config);
    let keys_template = {
        let mut ctx = ctx_template.clone();
        let mut rng = ShadowHarvester::new();
        ctx.generate_keys(&mut rng)
    };
    
    let start = Instant::now();
    
    // Perform 10 encrypt operations in parallel
    let results: Vec<_> = (0..10)
        .into_par_iter()
        .map(|i| {
            let mut ctx = ctx_template.clone();
            let mut rng = ShadowHarvester::new();
            let keys = keys_template.clone(); // Use cloned keys for each operation
            let ct = ctx.encrypt(i, &keys.public_key, &mut rng);
            (i, ctx.noise_stats(&ct))
        })
        .collect();
    
    let elapsed = start.elapsed().as_millis() as f64 / 1_000.0;
    
    println!("10 parallel encrypt operations took {:.2}s", elapsed);
    println!("Average per operation: {:.2}s", elapsed / 10.0);
    println!("Operations per second: {:.2}", 10.0 / elapsed);
}

/// Benchmark session creation and destruction to identify resource leaks
#[test]
fn benchmark_session_lifecycle() {
    println!("\n┌────────────────────────────────────────────────────────────┐");
    println!("│  SESSION LIFECYCLE ANALYSIS - secure_128                   │");
    println!("└────────────────────────────────────────────────────────────┘");
    
    let config_name = "secure_128";
    
    let start_memory = get_current_rss_kb().unwrap_or(0);
    let start = Instant::now();
    
    // Create and destroy 100 sessions
    for i in 0..100 {
        let session_result = crate::session::Session::new(config_name);
        if let Ok(_session) = session_result {
            // Session is automatically dropped here
        }
    }
    
    let elapsed = start.elapsed().as_millis() as f64 / 1_000.0;
    let end_memory = get_current_rss_kb().unwrap_or(0);
    let memory_delta = end_memory as i64 - start_memory as i64;
    
    println!("100 session creations/destructions took {:.2}s", elapsed);
    println!("Memory change: {} KB (positive = increase)", memory_delta);
    println!("Average per session: {:.2}ms", (elapsed * 1000.0) / 100.0);
}

/// Helper function to get current RSS (Resident Set Size) in KB
fn get_current_rss_kb() -> Option<u64> {
    #[cfg(target_os = "linux")]
    {
        std::fs::read_to_string("/proc/self/status")
            .ok()
            .and_then(|s| {
                s.lines()
                    .find(|line| line.starts_with("VmRSS:"))
                    .and_then(|line| line.split_whitespace().nth(1)?.parse::<u64>().ok())
            })
    }
    #[cfg(not(target_os = "linux"))]
    {
        // On non-Linux systems, return None
        None
    }
}

/// Benchmark batch operations to identify potential DoS vectors
#[test]
fn benchmark_batch_operations() {
    println!("\n┌────────────────────────────────────────────────────────────┐");
    println!("│  BATCH OPERATION ANALYSIS - secure_128                     │");
    println!("└────────────────────────────────────────────────────────────┘");
    
    let config = SecureConfig::secure_128().into_config();
    let mut ctx = bench_ctx(config);
    let mut rng = ShadowHarvester::new();
    let keys = ctx.generate_keys(&mut rng);

    // Test batch encryption performance
    let start = Instant::now();
    let mut cts = Vec::new();
    for i in 0..100 {
        let ct = ctx.encrypt(i, &keys.public_key, &mut rng);
        cts.push(ct);
    }
    let encrypt_time = start.elapsed().as_millis() as f64 / 1_000.0;
    
    // Test batch addition performance
    let start = Instant::now();
    for i in 0..99 {
        let _result = ctx.add(&cts[i], &cts[i+1]);
    }
    let add_time = start.elapsed().as_millis() as f64 / 1_000.0;
    
    println!("Batch of 100 encrypt operations: {:.2}s ({:.2}ms/op)", 
             encrypt_time, (encrypt_time * 1000.0) / 100.0);
    println!("Batch of 99 add operations: {:.2}s ({:.2}ms/op)", 
             add_time, (add_time * 1000.0) / 99.0);
}

/// Benchmark depth-specific operations to identify degradation patterns
#[test]
fn benchmark_depth_specific_operations() {
    println!("\n┌────────────────────────────────────────────────────────────┐");
    println!("│  DEPTH-SPECIFIC ANALYSIS - secure_128                      │");
    println!("└────────────────────────────────────────────────────────────┘");
    
    let config = SecureConfig::secure_128().into_config();
    let mut ctx = bench_ctx(config);
    let mut rng = ShadowHarvester::new();
    let keys = ctx.generate_keys(&mut rng);

    let mut ct = ctx.encrypt(2, &keys.public_key, &mut rng);
    
    println!("Depth | Operation Type | Time (ms) | Noise Level | Collapses");
    println!("------|---------------|-----------|-------------|----------");
    
    for depth in 1..=50 {
        let start = Instant::now();
        
        // Alternate between add and mul to simulate realistic usage
        if depth % 2 == 0 {
            let ct_clone = ct.clone();
            ct = ctx.add(&ct, &ct_clone);
            let elapsed = start.elapsed().as_millis() as f64 / 1_000.0;
            let stats = ctx.noise_stats(&ct);
            println!("{:5} | {:13} | {:9.2} | {:11.2} | {:9}", 
                     depth, "ADD", elapsed * 1000.0, stats.noise_level, stats.collapses);
        } else {
            let ct_clone = ct.clone();
            ct = ctx.mul(&ct, &ct_clone, &keys.eval_key);
            let elapsed = start.elapsed().as_millis() as f64 / 1_000.0;
            let stats = ctx.noise_stats(&ct);
            println!("{:5} | {:13} | {:9.2} | {:11.2} | {:9}", 
                     depth, "MUL", elapsed * 1000.0, stats.noise_level, stats.collapses);
        }
    }
}