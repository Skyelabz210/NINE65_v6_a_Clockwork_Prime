---
title: "Qmnf Performance Report 20251014 201029"
description: "Placeholder description — please update."
authors:
  - "maintainer <maintainer@example.org>"
maintainers:
  - "See AGENTS.md"
tags:
status: "published"
canonical_path: "/docs/QMNF_Performance_Report_20251014_201029.md"
last_reviewed: "2025-11-07"
version: "1.0"
references:
---

# QMNF System Performance Analysis Report
**Generated:** 2025-10-14 20:10:29
**Duration:** 2.50 seconds

## Executive Summary
- **Total Tests:** 21
- **Success Rate:** 90.5%
- **Total Operations:** 44,320
- **Average Throughput:** 38360.05263157895 operations/second

## Performance Metrics

### Execution Time Analysis
- **Average:** 118.15ms
- **Median:** 55.10ms
- **Range:** 0.27ms - 732.47ms

### Memory Usage Analysis
- **Average per Operation:** 0.00MB
- **Peak Usage:** 1.00MB
- **Total Memory Impact:** 2.00MB

### Throughput Analysis
- **Average:** 38360.05263157895 ops/sec
- **Peak:** 208405 ops/sec

### System Resource Utilization
- **Average CPU:** 26.6%
- **Peak CPU:** 34.3%
- **Average Memory:** 74.8%
- **Peak Memory:** 74.9%

## Performance by Component

### QMNFRational
- **Tests:** 3
- **Avg Time:** 152.40ms
- **Throughput:** 42309.0 ops/sec
- **Total Ops:** 16,000

### GeometricPoint
- **Tests:** 1
- **Avg Time:** 137.13ms
- **Throughput:** 36460.0 ops/sec
- **Total Ops:** 5,000

### GeometricLine
- **Tests:** 1
- **Avg Time:** 99.15ms
- **Throughput:** 20170.0 ops/sec
- **Total Ops:** 2,000

### GeometricCircle
- **Tests:** 1
- **Avg Time:** 246.46ms
- **Throughput:** 4057.0 ops/sec
- **Total Ops:** 1,000

### TheoremProving
- **Tests:** 2
- **Avg Time:** 66.79ms
- **Throughput:** 12290.0 ops/sec
- **Total Ops:** 1,500

### Multiplication
- **Tests:** 2
- **Avg Time:** 4.86ms
- **Throughput:** 54239.0 ops/sec
- **Total Ops:** 600

### Memory
- **Tests:** 2
- **Avg Time:** 390.23ms
- **Throughput:** 104885.0 ops/sec
- **Total Ops:** 11,000

### Scaling
- **Tests:** 7
- **Avg Time:** 54.45ms
- **Throughput:** 28342.0 ops/sec
- **Total Ops:** 7,220

## Identified Bottlenecks

### 🔴 Test Failures
**Severity:** High
**Description:** 2 tests failed
**Recommendation:** Investigate and fix failing components before production

## Optimization Recommendations

1. Optimize GCD computation with faster algorithms (binary GCD, extended Euclidean)
2. Consider spatial indexing for geometric operations with large point sets
3. Consider implementing operation caching for frequently used computations
4. Implement memory pressure monitoring and adaptive garbage collection
5. [test_failures] Investigate and fix failing components before production
6. Implement adaptive algorithms that scale better with input size

## Failed Tests

### Multiplication_Heavy
**Error:** name 'gcd' is not defined

### Multiplication_Heavy_Large
**Error:** name 'gcd' is not defined

## Detailed Test Results

### ✅ QMNFRational_Basic_Arithmetic
- **Status:** Success
- **Time:** 275.38ms
- **Memory:** 0.00MB
- **Iterations:** 10,000
- **Throughput:** 36313 ops/sec

### ✅ QMNFRational_Large_Numbers
- **Status:** Success
- **Time:** 121.11ms
- **Memory:** 0.00MB
- **Iterations:** 1,000
- **Throughput:** 8256 ops/sec

### ✅ QMNFRational_GCD_Intensive
- **Status:** Success
- **Time:** 60.71ms
- **Memory:** 0.00MB
- **Iterations:** 5,000
- **Throughput:** 82360 ops/sec

### ✅ GeometricPoint_Operations
- **Status:** Success
- **Time:** 137.13ms
- **Memory:** 1.00MB
- **Iterations:** 5,000
- **Throughput:** 36460 ops/sec

### ✅ GeometricLine_Operations
- **Status:** Success
- **Time:** 99.15ms
- **Memory:** 0.00MB
- **Iterations:** 2,000
- **Throughput:** 20170 ops/sec

### ✅ GeometricCircle_Operations
- **Status:** Success
- **Time:** 246.46ms
- **Memory:** 0.00MB
- **Iterations:** 1,000
- **Throughput:** 4057 ops/sec

### ✅ TheoremProving_Collinearity
- **Status:** Success
- **Time:** 54.85ms
- **Memory:** 0.00MB
- **Iterations:** 1,000
- **Throughput:** 18230 ops/sec

### ✅ TheoremProving_Concurrency
- **Status:** Success
- **Time:** 78.74ms
- **Memory:** 0.00MB
- **Iterations:** 500
- **Throughput:** 6350 ops/sec

### ✅ Multiplication_Standard
- **Status:** Success
- **Time:** 6.32ms
- **Memory:** 0.00MB
- **Iterations:** 500
- **Throughput:** 79175 ops/sec

### ❌ Multiplication_Heavy
- **Status:** Failed
- **Time:** 3.61ms
- **Memory:** 0.00MB
- **Iterations:** 500
- **Throughput:** 138507 ops/sec

**Error:** name 'gcd' is not defined

### ✅ Multiplication_Standard_Large
- **Status:** Success
- **Time:** 3.41ms
- **Memory:** 0.00MB
- **Iterations:** 100
- **Throughput:** 29303 ops/sec

### ❌ Multiplication_Heavy_Large
- **Status:** Failed
- **Time:** 2.15ms
- **Memory:** 0.00MB
- **Iterations:** 100
- **Throughput:** 46411 ops/sec

**Error:** name 'gcd' is not defined

### ✅ Memory_ObjectCreation
- **Status:** Success
- **Time:** 47.98ms
- **Memory:** 0.00MB
- **Iterations:** 10,000
- **Throughput:** 208405 ops/sec

### ✅ Memory_LargeComputations
- **Status:** Success
- **Time:** 732.47ms
- **Memory:** 0.00MB
- **Iterations:** 1,000
- **Throughput:** 1365 ops/sec

### ✅ Scaling_Rational_10
- **Status:** Success
- **Time:** 0.41ms
- **Memory:** 0.00MB
- **Iterations:** 10
- **Throughput:** 24425 ops/sec

### ✅ Scaling_Geometric_10
- **Status:** Success
- **Time:** 0.27ms
- **Memory:** 0.00MB
- **Iterations:** 10
- **Throughput:** 36489 ops/sec

### ✅ Scaling_Rational_100
- **Status:** Success
- **Time:** 4.37ms
- **Memory:** 0.00MB
- **Iterations:** 100
- **Throughput:** 22904 ops/sec

### ✅ Scaling_Geometric_100
- **Status:** Success
- **Time:** 2.58ms
- **Memory:** 0.00MB
- **Iterations:** 100
- **Throughput:** 38790 ops/sec

### ✅ Scaling_Rational_1000
- **Status:** Success
- **Time:** 55.10ms
- **Memory:** 0.00MB
- **Iterations:** 1,000
- **Throughput:** 18148 ops/sec

### ✅ Scaling_Geometric_1000
- **Status:** Success
- **Time:** 24.61ms
- **Memory:** 0.00MB
- **Iterations:** 1,000
- **Throughput:** 40625 ops/sec

### ✅ Scaling_Rational_5000
- **Status:** Success
- **Time:** 293.84ms
- **Memory:** 0.00MB
- **Iterations:** 5,000
- **Throughput:** 17016 ops/sec
