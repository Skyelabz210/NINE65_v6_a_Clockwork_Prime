---
title: "Qmnf Performance Report 20251014 201116"
description: "Placeholder description — please update."
authors:
  - "maintainer <maintainer@example.org>"
maintainers:
  - "See AGENTS.md"
tags:
status: "published"
canonical_path: "/docs/QMNF_Performance_Report_20251014_201116.md"
last_reviewed: "2025-11-07"
version: "1.0"
references:
---

# QMNF System Performance Analysis Report
**Generated:** 2025-10-14 20:11:16
**Duration:** 2.48 seconds

## Executive Summary
- **Total Tests:** 21
- **Success Rate:** 100.0%
- **Total Operations:** 44,920
- **Average Throughput:** 42096.23809523809 operations/second

## Performance Metrics

### Execution Time Analysis
- **Average:** 106.56ms
- **Median:** 52.18ms
- **Range:** 0.26ms - 739.58ms

### Memory Usage Analysis
- **Average per Operation:** 0.00MB
- **Peak Usage:** 1.00MB
- **Total Memory Impact:** 2.00MB

### Throughput Analysis
- **Average:** 42096.23809523809 ops/sec
- **Peak:** 209618 ops/sec

### System Resource Utilization
- **Average CPU:** 27.5%
- **Peak CPU:** 52.9%
- **Average Memory:** 74.8%
- **Peak Memory:** 74.8%

## Performance by Component

### QMNFRational
- **Tests:** 3
- **Avg Time:** 157.36ms
- **Throughput:** 41490.0 ops/sec
- **Total Ops:** 16,000

### GeometricPoint
- **Tests:** 1
- **Avg Time:** 125.37ms
- **Throughput:** 39880.0 ops/sec
- **Total Ops:** 5,000

### GeometricLine
- **Tests:** 1
- **Avg Time:** 94.65ms
- **Throughput:** 21129.0 ops/sec
- **Total Ops:** 2,000

### GeometricCircle
- **Tests:** 1
- **Avg Time:** 232.33ms
- **Throughput:** 4304.0 ops/sec
- **Total Ops:** 1,000

### TheoremProving
- **Tests:** 2
- **Avg Time:** 65.83ms
- **Throughput:** 12515.0 ops/sec
- **Total Ops:** 1,500

### Multiplication
- **Tests:** 4
- **Avg Time:** 4.18ms
- **Throughput:** 64490.0 ops/sec
- **Total Ops:** 1,200

### Memory
- **Tests:** 2
- **Avg Time:** 393.64ms
- **Throughput:** 105485.0 ops/sec
- **Total Ops:** 11,000

### Scaling
- **Tests:** 7
- **Avg Time:** 53.95ms
- **Throughput:** 28610.0 ops/sec
- **Total Ops:** 7,220

## Optimization Recommendations

1. Implement adaptive algorithms that scale better with input size
2. Optimize GCD computation with faster algorithms (binary GCD, extended Euclidean)
3. Implement memory pressure monitoring and adaptive garbage collection
4. Consider implementing operation caching for frequently used computations
5. Consider spatial indexing for geometric operations with large point sets

## Detailed Test Results

### ✅ QMNFRational_Basic_Arithmetic
- **Status:** Success
- **Time:** 290.84ms
- **Memory:** 0.00MB
- **Iterations:** 10,000
- **Throughput:** 34383 ops/sec

### ✅ QMNFRational_Large_Numbers
- **Status:** Success
- **Time:** 120.08ms
- **Memory:** 0.00MB
- **Iterations:** 1,000
- **Throughput:** 8327 ops/sec

### ✅ QMNFRational_GCD_Intensive
- **Status:** Success
- **Time:** 61.15ms
- **Memory:** 0.00MB
- **Iterations:** 5,000
- **Throughput:** 81760 ops/sec

### ✅ GeometricPoint_Operations
- **Status:** Success
- **Time:** 125.37ms
- **Memory:** 1.00MB
- **Iterations:** 5,000
- **Throughput:** 39880 ops/sec

### ✅ GeometricLine_Operations
- **Status:** Success
- **Time:** 94.65ms
- **Memory:** 0.00MB
- **Iterations:** 2,000
- **Throughput:** 21129 ops/sec

### ✅ GeometricCircle_Operations
- **Status:** Success
- **Time:** 232.33ms
- **Memory:** 0.00MB
- **Iterations:** 1,000
- **Throughput:** 4304 ops/sec

### ✅ TheoremProving_Collinearity
- **Status:** Success
- **Time:** 53.72ms
- **Memory:** 0.00MB
- **Iterations:** 1,000
- **Throughput:** 18615 ops/sec

### ✅ TheoremProving_Concurrency
- **Status:** Success
- **Time:** 77.94ms
- **Memory:** 0.00MB
- **Iterations:** 500
- **Throughput:** 6415 ops/sec

### ✅ Multiplication_Standard
- **Status:** Success
- **Time:** 6.32ms
- **Memory:** 0.00MB
- **Iterations:** 500
- **Throughput:** 79096 ops/sec

### ✅ Multiplication_Heavy
- **Status:** Success
- **Time:** 4.61ms
- **Memory:** 0.00MB
- **Iterations:** 500
- **Throughput:** 108573 ops/sec

### ✅ Multiplication_Standard_Large
- **Status:** Success
- **Time:** 3.29ms
- **Memory:** 0.00MB
- **Iterations:** 100
- **Throughput:** 30432 ops/sec

### ✅ Multiplication_Heavy_Large
- **Status:** Success
- **Time:** 2.51ms
- **Memory:** 0.00MB
- **Iterations:** 100
- **Throughput:** 39862 ops/sec

### ✅ Memory_ObjectCreation
- **Status:** Success
- **Time:** 47.70ms
- **Memory:** 0.00MB
- **Iterations:** 10,000
- **Throughput:** 209618 ops/sec

### ✅ Memory_LargeComputations
- **Status:** Success
- **Time:** 739.58ms
- **Memory:** 0.00MB
- **Iterations:** 1,000
- **Throughput:** 1352 ops/sec

### ✅ Scaling_Rational_10
- **Status:** Success
- **Time:** 0.44ms
- **Memory:** 0.00MB
- **Iterations:** 10
- **Throughput:** 22848 ops/sec

### ✅ Scaling_Geometric_10
- **Status:** Success
- **Time:** 0.26ms
- **Memory:** 0.00MB
- **Iterations:** 10
- **Throughput:** 37800 ops/sec

### ✅ Scaling_Rational_100
- **Status:** Success
- **Time:** 4.32ms
- **Memory:** 0.00MB
- **Iterations:** 100
- **Throughput:** 23144 ops/sec

### ✅ Scaling_Geometric_100
- **Status:** Success
- **Time:** 2.50ms
- **Memory:** 0.00MB
- **Iterations:** 100
- **Throughput:** 40007 ops/sec

### ✅ Scaling_Rational_1000
- **Status:** Success
- **Time:** 52.18ms
- **Memory:** 0.00MB
- **Iterations:** 1,000
- **Throughput:** 19165 ops/sec

### ✅ Scaling_Geometric_1000
- **Status:** Success
- **Time:** 24.84ms
- **Memory:** 0.00MB
- **Iterations:** 1,000
- **Throughput:** 40251 ops/sec

### ✅ Scaling_Rational_5000
- **Status:** Success
- **Time:** 293.08ms
- **Memory:** 0.00MB
- **Iterations:** 5,000
- **Throughput:** 17060 ops/sec
