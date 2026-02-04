---
title: "Qmnf Performance Report 20251014 201049"
description: "Placeholder description — please update."
authors:
  - "maintainer <maintainer@example.org>"
maintainers:
  - "See AGENTS.md"
tags:
status: "published"
canonical_path: "/docs/QMNF_Performance_Report_20251014_201049.md"
last_reviewed: "2025-11-07"
version: "1.0"
references:
---

# QMNF System Performance Analysis Report
**Generated:** 2025-10-14 20:10:49
**Duration:** 2.51 seconds

## Executive Summary
- **Total Tests:** 21
- **Success Rate:** 90.5%
- **Total Operations:** 44,320
- **Average Throughput:** 37551.36842105263 operations/second

## Performance Metrics

### Execution Time Analysis
- **Average:** 121.00ms
- **Median:** 53.93ms
- **Range:** 0.27ms - 748.65ms

### Memory Usage Analysis
- **Average per Operation:** 0.00MB
- **Peak Usage:** 1.00MB
- **Total Memory Impact:** 2.00MB

### Throughput Analysis
- **Average:** 37551.36842105263 ops/sec
- **Peak:** 211394 ops/sec

### System Resource Utilization
- **Average CPU:** 28.4%
- **Peak CPU:** 34.0%
- **Average Memory:** 74.8%
- **Peak Memory:** 74.8%

## Performance by Component

### QMNFRational
- **Tests:** 3
- **Avg Time:** 164.71ms
- **Throughput:** 36395.0 ops/sec
- **Total Ops:** 16,000

### GeometricPoint
- **Tests:** 1
- **Avg Time:** 152.89ms
- **Throughput:** 32703.0 ops/sec
- **Total Ops:** 5,000

### GeometricLine
- **Tests:** 1
- **Avg Time:** 91.95ms
- **Throughput:** 21749.0 ops/sec
- **Total Ops:** 2,000

### GeometricCircle
- **Tests:** 1
- **Avg Time:** 235.55ms
- **Throughput:** 4245.0 ops/sec
- **Total Ops:** 1,000

### TheoremProving
- **Tests:** 2
- **Avg Time:** 66.13ms
- **Throughput:** 12496.0 ops/sec
- **Total Ops:** 1,500

### Multiplication
- **Tests:** 2
- **Avg Time:** 4.85ms
- **Throughput:** 54410.0 ops/sec
- **Total Ops:** 600

### Memory
- **Tests:** 2
- **Avg Time:** 397.98ms
- **Throughput:** 106364.0 ops/sec
- **Total Ops:** 11,000

### Scaling
- **Tests:** 7
- **Avg Time:** 55.22ms
- **Throughput:** 28435.0 ops/sec
- **Total Ops:** 7,220

## Identified Bottlenecks

### 🔴 Test Failures
**Severity:** High
**Description:** 2 tests failed
**Recommendation:** Investigate and fix failing components before production

## Optimization Recommendations

1. Implement adaptive algorithms that scale better with input size
2. Consider spatial indexing for geometric operations with large point sets
3. Implement memory pressure monitoring and adaptive garbage collection
4. Consider implementing operation caching for frequently used computations
5. Optimize GCD computation with faster algorithms (binary GCD, extended Euclidean)
6. [test_failures] Investigate and fix failing components before production

## Failed Tests

### Multiplication_Heavy
**Error:** name 'QMNFRational' is not defined

### Multiplication_Heavy_Large
**Error:** name 'QMNFRational' is not defined

## Detailed Test Results

### ✅ QMNFRational_Basic_Arithmetic
- **Status:** Success
- **Time:** 297.94ms
- **Memory:** 0.00MB
- **Iterations:** 10,000
- **Throughput:** 33563 ops/sec

### ✅ QMNFRational_Large_Numbers
- **Status:** Success
- **Time:** 122.04ms
- **Memory:** 0.00MB
- **Iterations:** 1,000
- **Throughput:** 8193 ops/sec

### ✅ QMNFRational_GCD_Intensive
- **Status:** Success
- **Time:** 74.15ms
- **Memory:** 0.00MB
- **Iterations:** 5,000
- **Throughput:** 67431 ops/sec

### ✅ GeometricPoint_Operations
- **Status:** Success
- **Time:** 152.89ms
- **Memory:** 1.00MB
- **Iterations:** 5,000
- **Throughput:** 32703 ops/sec

### ✅ GeometricLine_Operations
- **Status:** Success
- **Time:** 91.95ms
- **Memory:** 0.00MB
- **Iterations:** 2,000
- **Throughput:** 21749 ops/sec

### ✅ GeometricCircle_Operations
- **Status:** Success
- **Time:** 235.55ms
- **Memory:** 0.00MB
- **Iterations:** 1,000
- **Throughput:** 4245 ops/sec

### ✅ TheoremProving_Collinearity
- **Status:** Success
- **Time:** 53.67ms
- **Memory:** 0.00MB
- **Iterations:** 1,000
- **Throughput:** 18630 ops/sec

### ✅ TheoremProving_Concurrency
- **Status:** Success
- **Time:** 78.59ms
- **Memory:** 0.00MB
- **Iterations:** 500
- **Throughput:** 6362 ops/sec

### ✅ Multiplication_Standard
- **Status:** Success
- **Time:** 6.27ms
- **Memory:** 0.00MB
- **Iterations:** 500
- **Throughput:** 79771 ops/sec

### ❌ Multiplication_Heavy
- **Status:** Failed
- **Time:** 3.83ms
- **Memory:** 0.00MB
- **Iterations:** 500
- **Throughput:** 130670 ops/sec

**Error:** name 'QMNFRational' is not defined

### ✅ Multiplication_Standard_Large
- **Status:** Success
- **Time:** 3.44ms
- **Memory:** 0.00MB
- **Iterations:** 100
- **Throughput:** 29049 ops/sec

### ❌ Multiplication_Heavy_Large
- **Status:** Failed
- **Time:** 2.52ms
- **Memory:** 0.00MB
- **Iterations:** 100
- **Throughput:** 39599 ops/sec

**Error:** name 'QMNFRational' is not defined

### ✅ Memory_ObjectCreation
- **Status:** Success
- **Time:** 47.30ms
- **Memory:** 0.00MB
- **Iterations:** 10,000
- **Throughput:** 211394 ops/sec

### ✅ Memory_LargeComputations
- **Status:** Success
- **Time:** 748.65ms
- **Memory:** 0.00MB
- **Iterations:** 1,000
- **Throughput:** 1335 ops/sec

### ✅ Scaling_Rational_10
- **Status:** Success
- **Time:** 0.42ms
- **Memory:** 0.00MB
- **Iterations:** 10
- **Throughput:** 23900 ops/sec

### ✅ Scaling_Geometric_10
- **Status:** Success
- **Time:** 0.27ms
- **Memory:** 0.00MB
- **Iterations:** 10
- **Throughput:** 36935 ops/sec

### ✅ Scaling_Rational_100
- **Status:** Success
- **Time:** 4.49ms
- **Memory:** 0.00MB
- **Iterations:** 100
- **Throughput:** 22287 ops/sec

### ✅ Scaling_Geometric_100
- **Status:** Success
- **Time:** 2.45ms
- **Memory:** 0.00MB
- **Iterations:** 100
- **Throughput:** 40754 ops/sec

### ✅ Scaling_Rational_1000
- **Status:** Success
- **Time:** 53.93ms
- **Memory:** 0.00MB
- **Iterations:** 1,000
- **Throughput:** 18541 ops/sec

### ✅ Scaling_Geometric_1000
- **Status:** Success
- **Time:** 25.02ms
- **Memory:** 0.00MB
- **Iterations:** 1,000
- **Throughput:** 39965 ops/sec

### ✅ Scaling_Rational_5000
- **Status:** Success
- **Time:** 299.95ms
- **Memory:** 0.00MB
- **Iterations:** 5,000
- **Throughput:** 16669 ops/sec
