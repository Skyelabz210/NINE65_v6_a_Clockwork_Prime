# QMNF System Performance Analysis Report
**Generated:** 2025-11-06 14:57:55
**Duration:** 32.27 seconds

## Executive Summary
- **Total Tests:** 21
- **Success Rate:** 71.4%
- **Total Operations:** 39,820
- **Average Throughput:** 4135.266666666666 operations/second

## Performance Metrics

### Execution Time Analysis
- **Average:** 2125.59ms
- **Median:** 527.38ms
- **Range:** 2.35ms - 15242.37ms

### Memory Usage Analysis
- **Average per Operation:** 0.00MB
- **Peak Usage:** 2.00MB
- **Total Memory Impact:** 2.00MB

### Throughput Analysis
- **Average:** 4135.266666666666 ops/sec
- **Peak:** 18961 ops/sec

### System Resource Utilization
- **Average CPU:** 87.5%
- **Peak CPU:** 100.0%
- **Average Memory:** 60.0%
- **Peak Memory:** 62.5%

## Performance by Component

### QMNFRational
- **Tests:** 3
- **Avg Time:** 2920.27ms
- **Throughput:** 2414.0 ops/sec
- **Total Ops:** 16,000

### GeometricPoint
- **Tests:** 1
- **Avg Time:** 1119.24ms
- **Throughput:** 4467.0 ops/sec
- **Total Ops:** 5,000

### Multiplication
- **Tests:** 2
- **Avg Time:** 43.59ms
- **Throughput:** 6021.0 ops/sec
- **Total Ops:** 600

### Memory
- **Tests:** 2
- **Avg Time:** 7884.88ms
- **Throughput:** 9513.0 ops/sec
- **Total Ops:** 11,000

### Scaling
- **Tests:** 7
- **Avg Time:** 878.13ms
- **Throughput:** 2750.0 ops/sec
- **Total Ops:** 7,220

## Identified Bottlenecks

### 🔴 Slow Operations
**Severity:** High
**Description:** Some operations taking over 1 second: 15242ms
**Recommendation:** Investigate algorithmic complexity and consider optimization

### 🟡 Cpu Usage
**Severity:** Medium
**Description:** High CPU usage: 100.00%
**Recommendation:** Consider load balancing or algorithm optimization

### 🔴 Test Failures
**Severity:** High
**Description:** 6 tests failed
**Recommendation:** Investigate and fix failing components before production

## Optimization Recommendations

1. Consider implementing operation caching for frequently used computations
2. [slow_operations] Investigate algorithmic complexity and consider optimization
3. [cpu_usage] Consider load balancing or algorithm optimization
4. [test_failures] Investigate and fix failing components before production
5. Consider implementing CPU-aware load balancing
6. Optimize GCD computation with faster algorithms (binary GCD, extended Euclidean)
7. Implement adaptive algorithms that scale better with input size
8. Consider spatial indexing for geometric operations with large point sets

## Failed Tests

### GeometricLine_Operations
**Error:** 'builtins.Rational' object has no attribute 'is_zero'

### GeometricCircle_Operations
**Error:** 'builtins.Rational' object has no attribute 'is_zero'

### TheoremProving_Collinearity
**Error:** 'builtins.Rational' object has no attribute 'is_zero'

### TheoremProving_Concurrency
**Error:** 'builtins.Rational' object has no attribute 'is_zero'

### Multiplication_Heavy
**Error:** bad operand type for abs(): 'builtin_function_or_method'

### Multiplication_Heavy_Large
**Error:** bad operand type for abs(): 'builtin_function_or_method'

## Detailed Test Results

### ✅ QMNFRational_Basic_Arithmetic
- **Status:** Success
- **Time:** 5977.52ms
- **Memory:** 0.00MB
- **Iterations:** 10,000
- **Throughput:** 1672 ops/sec

### ✅ QMNFRational_Large_Numbers
- **Status:** Success
- **Time:** 1785.65ms
- **Memory:** 0.00MB
- **Iterations:** 1,000
- **Throughput:** 560 ops/sec

### ✅ QMNFRational_GCD_Intensive
- **Status:** Success
- **Time:** 997.63ms
- **Memory:** 0.00MB
- **Iterations:** 5,000
- **Throughput:** 5011 ops/sec

### ✅ GeometricPoint_Operations
- **Status:** Success
- **Time:** 1119.24ms
- **Memory:** 2.00MB
- **Iterations:** 5,000
- **Throughput:** 4467 ops/sec

### ❌ GeometricLine_Operations
- **Status:** Failed
- **Time:** 0.34ms
- **Memory:** 0.00MB
- **Iterations:** 2,000
- **Throughput:** 5872731 ops/sec

**Error:** 'builtins.Rational' object has no attribute 'is_zero'

### ❌ GeometricCircle_Operations
- **Status:** Failed
- **Time:** 0.56ms
- **Memory:** 0.00MB
- **Iterations:** 1,000
- **Throughput:** 1769604 ops/sec

**Error:** 'builtins.Rational' object has no attribute 'is_zero'

### ❌ TheoremProving_Collinearity
- **Status:** Failed
- **Time:** 0.43ms
- **Memory:** 0.00MB
- **Iterations:** 1,000
- **Throughput:** 2314429 ops/sec

**Error:** 'builtins.Rational' object has no attribute 'is_zero'

### ❌ TheoremProving_Concurrency
- **Status:** Failed
- **Time:** 0.30ms
- **Memory:** 0.00MB
- **Iterations:** 500
- **Throughput:** 1640339 ops/sec

**Error:** 'builtins.Rational' object has no attribute 'is_zero'

### ✅ Multiplication_Standard
- **Status:** Success
- **Time:** 58.17ms
- **Memory:** 0.00MB
- **Iterations:** 500
- **Throughput:** 8595 ops/sec

### ❌ Multiplication_Heavy
- **Status:** Failed
- **Time:** 3.49ms
- **Memory:** 0.00MB
- **Iterations:** 500
- **Throughput:** 143209 ops/sec

**Error:** bad operand type for abs(): 'builtin_function_or_method'

### ✅ Multiplication_Standard_Large
- **Status:** Success
- **Time:** 29.01ms
- **Memory:** 0.00MB
- **Iterations:** 100
- **Throughput:** 3447 ops/sec

### ❌ Multiplication_Heavy_Large
- **Status:** Failed
- **Time:** 2.35ms
- **Memory:** 0.00MB
- **Iterations:** 100
- **Throughput:** 42604 ops/sec

**Error:** bad operand type for abs(): 'builtin_function_or_method'

### ✅ Memory_ObjectCreation
- **Status:** Success
- **Time:** 527.38ms
- **Memory:** 0.00MB
- **Iterations:** 10,000
- **Throughput:** 18961 ops/sec

### ✅ Memory_LargeComputations
- **Status:** Success
- **Time:** 15242.37ms
- **Memory:** 0.00MB
- **Iterations:** 1,000
- **Throughput:** 65 ops/sec

### ✅ Scaling_Rational_10
- **Status:** Success
- **Time:** 6.94ms
- **Memory:** 0.00MB
- **Iterations:** 10
- **Throughput:** 1440 ops/sec

### ✅ Scaling_Geometric_10
- **Status:** Success
- **Time:** 2.35ms
- **Memory:** 0.00MB
- **Iterations:** 10
- **Throughput:** 4257 ops/sec

### ✅ Scaling_Rational_100
- **Status:** Success
- **Time:** 104.92ms
- **Memory:** 0.00MB
- **Iterations:** 100
- **Throughput:** 953 ops/sec

### ✅ Scaling_Geometric_100
- **Status:** Success
- **Time:** 18.05ms
- **Memory:** 0.00MB
- **Iterations:** 100
- **Throughput:** 5540 ops/sec

### ✅ Scaling_Rational_1000
- **Status:** Success
- **Time:** 924.90ms
- **Memory:** 0.00MB
- **Iterations:** 1,000
- **Throughput:** 1081 ops/sec

### ✅ Scaling_Geometric_1000
- **Status:** Success
- **Time:** 201.68ms
- **Memory:** 0.00MB
- **Iterations:** 1,000
- **Throughput:** 4958 ops/sec

### ✅ Scaling_Rational_5000
- **Status:** Success
- **Time:** 4888.04ms
- **Memory:** 0.00MB
- **Iterations:** 5,000
- **Throughput:** 1022 ops/sec
