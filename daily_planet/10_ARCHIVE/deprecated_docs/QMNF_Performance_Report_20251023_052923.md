# QMNF System Performance Analysis Report
**Generated:** 2025-10-23 05:29:23
**Duration:** 1002.70 seconds

## Executive Summary
- **Total Tests:** 23
- **Success Rate:** 39.1%
- **Total Operations:** 27,750
- **Average Throughput:** 54476.333333333336 operations/second

## Performance Metrics

### Execution Time Analysis
- **Average:** 111364.71ms
- **Median:** 113.19ms
- **Range:** 2.17ms - 500509.78ms

### Memory Usage Analysis
- **Average per Operation:** 0.00MB
- **Peak Usage:** 2.00MB
- **Total Memory Impact:** 4.00MB

### Throughput Analysis
- **Average:** 54476.333333333336 ops/sec
- **Peak:** 220845 ops/sec

### System Resource Utilization
- **Average CPU:** 9.1%
- **Peak CPU:** 100.0%
- **Average Memory:** 52.8%
- **Peak Memory:** 61.8%

## Performance by Component

### QMNFRational
- **Tests:** 3
- **Avg Time:** 207.97ms
- **Throughput:** 33069.0 ops/sec
- **Total Ops:** 16,000

### AgentCoordination
- **Tests:** 2
- **Avg Time:** 500399.08ms
- **Throughput:** 0.0 ops/sec
- **Total Ops:** 150

### Multiplication
- **Tests:** 2
- **Avg Time:** 3.12ms
- **Throughput:** 84498.0 ops/sec
- **Total Ops:** 600

### Memory
- **Tests:** 2
- **Avg Time:** 427.04ms
- **Throughput:** 111040.0 ops/sec
- **Total Ops:** 11,000

## Identified Bottlenecks

### 🔴 Slow Operations
**Severity:** High
**Description:** Some operations taking over 1 second: 500509ms
**Recommendation:** Investigate algorithmic complexity and consider optimization

### 🟡 Cpu Usage
**Severity:** Medium
**Description:** High CPU usage: 100.00%
**Recommendation:** Consider load balancing or algorithm optimization

### 🔴 Test Failures
**Severity:** High
**Description:** 14 tests failed
**Recommendation:** Investigate and fix failing components before production

### 🟢 Low Throughput
**Severity:** Low
**Description:** Low throughput in AgentCoordination: 0 ops/sec
**Recommendation:** Optimize AgentCoordination algorithms or consider parallel processing

## Optimization Recommendations

1. [low_throughput] Optimize AgentCoordination algorithms or consider parallel processing
2. [cpu_usage] Consider load balancing or algorithm optimization
3. [test_failures] Investigate and fix failing components before production
4. [slow_operations] Investigate algorithmic complexity and consider optimization
5. Optimize GCD computation with faster algorithms (binary GCD, extended Euclidean)
6. Consider implementing operation caching for frequently used computations
7. Implement task batching and prioritization for improved agent efficiency

## Failed Tests

### GeometricPoint_Operations
**Error:** Rational.__new__() missing 1 required positional argument: 'den'

### GeometricLine_Operations
**Error:** Rational.__new__() missing 1 required positional argument: 'den'

### GeometricCircle_Operations
**Error:** Rational.__new__() missing 1 required positional argument: 'den'

### TheoremProving_Collinearity
**Error:** Rational.__new__() missing 1 required positional argument: 'den'

### TheoremProving_Concurrency
**Error:** Rational.__new__() missing 1 required positional argument: 'den'

### Multiplication_Heavy
**Error:** 'builtins.Rational' object has no attribute 'numerator'

### Multiplication_Heavy_Large
**Error:** 'builtins.Rational' object has no attribute 'numerator'

### Scaling_Rational_10
**Error:** Rational.__new__() missing 1 required positional argument: 'den'

### Scaling_Geometric_10
**Error:** Rational.__new__() missing 1 required positional argument: 'den'

### Scaling_Rational_100
**Error:** Rational.__new__() missing 1 required positional argument: 'den'

### Scaling_Geometric_100
**Error:** Rational.__new__() missing 1 required positional argument: 'den'

### Scaling_Rational_1000
**Error:** Rational.__new__() missing 1 required positional argument: 'den'

### Scaling_Geometric_1000
**Error:** Rational.__new__() missing 1 required positional argument: 'den'

### Scaling_Rational_5000
**Error:** Rational.__new__() missing 1 required positional argument: 'den'

## Detailed Test Results

### ✅ QMNFRational_Basic_Arithmetic
- **Status:** Success
- **Time:** 436.61ms
- **Memory:** 0.00MB
- **Iterations:** 10,000
- **Throughput:** 22903 ops/sec

### ✅ QMNFRational_Large_Numbers
- **Status:** Success
- **Time:** 113.19ms
- **Memory:** 0.00MB
- **Iterations:** 1,000
- **Throughput:** 8834 ops/sec

### ✅ QMNFRational_GCD_Intensive
- **Status:** Success
- **Time:** 74.10ms
- **Memory:** 0.00MB
- **Iterations:** 5,000
- **Throughput:** 67472 ops/sec

### ❌ GeometricPoint_Operations
- **Status:** Failed
- **Time:** 0.04ms
- **Memory:** 0.00MB
- **Iterations:** 5,000
- **Throughput:** 118956987 ops/sec

**Error:** Rational.__new__() missing 1 required positional argument: 'den'

### ❌ GeometricLine_Operations
- **Status:** Failed
- **Time:** 0.06ms
- **Memory:** 0.00MB
- **Iterations:** 2,000
- **Throughput:** 34529190 ops/sec

**Error:** Rational.__new__() missing 1 required positional argument: 'den'

### ❌ GeometricCircle_Operations
- **Status:** Failed
- **Time:** 0.04ms
- **Memory:** 0.00MB
- **Iterations:** 1,000
- **Throughput:** 21854100 ops/sec

**Error:** Rational.__new__() missing 1 required positional argument: 'den'

### ❌ TheoremProving_Collinearity
- **Status:** Failed
- **Time:** 0.04ms
- **Memory:** 0.00MB
- **Iterations:** 1,000
- **Throughput:** 26297794 ops/sec

**Error:** Rational.__new__() missing 1 required positional argument: 'den'

### ❌ TheoremProving_Concurrency
- **Status:** Failed
- **Time:** 0.03ms
- **Memory:** 0.00MB
- **Iterations:** 500
- **Throughput:** 15355792 ops/sec

**Error:** Rational.__new__() missing 1 required positional argument: 'den'

### ✅ AgentCoordination_SingleTask
- **Status:** Success
- **Time:** 500509.78ms
- **Memory:** 2.00MB
- **Iterations:** 100
- **Throughput:** 0 ops/sec

### ✅ AgentCoordination_ParallelTasks
- **Status:** Success
- **Time:** 500288.38ms
- **Memory:** 1.00MB
- **Iterations:** 50
- **Throughput:** 0 ops/sec

### ✅ Multiplication_Standard
- **Status:** Success
- **Time:** 4.07ms
- **Memory:** 0.00MB
- **Iterations:** 500
- **Throughput:** 122842 ops/sec

### ❌ Multiplication_Heavy
- **Status:** Failed
- **Time:** 3.21ms
- **Memory:** 0.00MB
- **Iterations:** 500
- **Throughput:** 155761 ops/sec

**Error:** 'builtins.Rational' object has no attribute 'numerator'

### ✅ Multiplication_Standard_Large
- **Status:** Success
- **Time:** 2.17ms
- **Memory:** 0.00MB
- **Iterations:** 100
- **Throughput:** 46155 ops/sec

### ❌ Multiplication_Heavy_Large
- **Status:** Failed
- **Time:** 2.26ms
- **Memory:** 0.00MB
- **Iterations:** 100
- **Throughput:** 44295 ops/sec

**Error:** 'builtins.Rational' object has no attribute 'numerator'

### ✅ Memory_ObjectCreation
- **Status:** Success
- **Time:** 45.28ms
- **Memory:** 0.00MB
- **Iterations:** 10,000
- **Throughput:** 220845 ops/sec

### ✅ Memory_LargeComputations
- **Status:** Success
- **Time:** 808.81ms
- **Memory:** 0.00MB
- **Iterations:** 1,000
- **Throughput:** 1236 ops/sec

### ❌ Scaling_Rational_10
- **Status:** Failed
- **Time:** 0.06ms
- **Memory:** 0.00MB
- **Iterations:** 10
- **Throughput:** 179391 ops/sec

**Error:** Rational.__new__() missing 1 required positional argument: 'den'

### ❌ Scaling_Geometric_10
- **Status:** Failed
- **Time:** 0.03ms
- **Memory:** 0.00MB
- **Iterations:** 10
- **Throughput:** 319478 ops/sec

**Error:** Rational.__new__() missing 1 required positional argument: 'den'

### ❌ Scaling_Rational_100
- **Status:** Failed
- **Time:** 0.05ms
- **Memory:** 0.00MB
- **Iterations:** 100
- **Throughput:** 1840569 ops/sec

**Error:** Rational.__new__() missing 1 required positional argument: 'den'

### ❌ Scaling_Geometric_100
- **Status:** Failed
- **Time:** 0.03ms
- **Memory:** 0.00MB
- **Iterations:** 100
- **Throughput:** 2925602 ops/sec

**Error:** Rational.__new__() missing 1 required positional argument: 'den'

### ❌ Scaling_Rational_1000
- **Status:** Failed
- **Time:** 0.05ms
- **Memory:** 0.00MB
- **Iterations:** 1,000
- **Throughput:** 19427285 ops/sec

**Error:** Rational.__new__() missing 1 required positional argument: 'den'

### ❌ Scaling_Geometric_1000
- **Status:** Failed
- **Time:** 0.03ms
- **Memory:** 0.00MB
- **Iterations:** 1,000
- **Throughput:** 32745016 ops/sec

**Error:** Rational.__new__() missing 1 required positional argument: 'den'

### ❌ Scaling_Rational_5000
- **Status:** Failed
- **Time:** 0.05ms
- **Memory:** 0.00MB
- **Iterations:** 5,000
- **Throughput:** 100908175 ops/sec

**Error:** Rational.__new__() missing 1 required positional argument: 'den'
