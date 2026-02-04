# CI/CD Integration Guide for QMNF System

**Version**: 1.0
**Date**: 2025-11-06
**Purpose**: Integrate QMNF validation tools into continuous integration/deployment pipelines

---

## Overview

This guide shows how to integrate the QMNF validation tools into your CI/CD pipeline to ensure:
- ✅ Float-free compliance (integer-only architecture)
- ✅ Phase 1 boundary architecture adherence
- ✅ Performance regression detection
- ✅ Code quality standards

---

## Available Validation Tools

### 1. Float Contamination Scanner
**Tool**: `tools/check_no_floats.py`
**Purpose**: Detect floating-point violations in core modules
**Exit Codes**: 0 (clean), 1 (violations found), 2 (error)

### 2. Boundary Compliance Validator
**Tool**: `tools/boundary_validator.py`
**Purpose**: Validate Phase 1 conversion boundary architecture
**Exit Codes**: 0 (compliant), 1 (violations), 2 (error)

### 3. Benchmark Comparator
**Tool**: `tools/compare_benchmarks.py`
**Purpose**: Detect performance regressions
**Exit Codes**: 0 (no regressions), 1 (regressions found), 2 (error)

---

## GitHub Actions Integration

### Basic Workflow

Create `.github/workflows/qmnf-validation.yml`:

```yaml
name: QMNF Validation

on:
  push:
    branches: [ main, develop ]
  pull_request:
    branches: [ main, develop ]

jobs:
  validate:
    runs-on: ubuntu-latest

    steps:
    - uses: actions/checkout@v3

    - name: Set up Python
      uses: actions/setup-python@v4
      with:
        python-version: '3.11'

    - name: Set up Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
        override: true

    - name: Install Python dependencies
      run: |
        pip install numpy scipy mpmath pytest psutil setuptools-rust

    - name: Build Rust components
      run: |
        cd hcvlang
        cargo build --release --features python

    - name: Install QMNF package
      run: |
        pip install -e .

    - name: Check for float contamination
      run: |
        python3 tools/check_no_floats.py --path qmnf/

    - name: Validate boundary compliance
      run: |
        python3 tools/boundary_validator.py

    - name: Run tests
      run: |
        python3 -m pytest tests/python/ -v

    - name: Run benchmarks
      run: |
        python3 -c "
        import time, json
        from qmnf_boundary_fixed import QMNFRational

        # Quick benchmark
        start = time.perf_counter()
        for i in range(10000):
            r = QMNFRational(i, i+1)
            _ = r + r
        duration = time.perf_counter() - start
        ops_per_sec = 10000 / duration

        # Save result
        result = {'ops_per_sec': int(ops_per_sec), 'target': 30000}
        with open('benchmark_result.json', 'w') as f:
            json.dump(result, f)

        # Check target
        if ops_per_sec < 30000:
            print(f'Performance regression: {ops_per_sec:.0f} < 30,000')
            exit(1)
        print(f'Performance OK: {ops_per_sec:.0f} ops/sec')
        "
```

### Advanced Workflow with Regression Detection

```yaml
name: QMNF Advanced Validation

on:
  pull_request:
    branches: [ main ]

jobs:
  validate-and-benchmark:
    runs-on: ubuntu-latest

    steps:
    - uses: actions/checkout@v3
      with:
        fetch-depth: 0  # Full history for comparison

    - name: Set up environment
      run: |
        python -m pip install --upgrade pip
        pip install numpy scipy mpmath pytest psutil setuptools-rust

    - name: Build and install
      run: |
        pip install -e .

    # Phase 1: Validation
    - name: Float contamination scan
      run: |
        python3 tools/check_no_floats.py --path qmnf/ --report violations.json
      continue-on-error: false

    - name: Boundary validation
      run: |
        python3 tools/boundary_validator.py --strict
      continue-on-error: false

    # Phase 2: Testing
    - name: Run test suite
      run: |
        python3 -m pytest tests/python/ -v --junitxml=test-results.xml

    - name: Upload test results
      if: always()
      uses: actions/upload-artifact@v3
      with:
        name: test-results
        path: test-results.xml

    # Phase 3: Benchmarking
    - name: Run current benchmarks
      run: |
        python3 << 'EOF'
        import time, json
        from qmnf_boundary_fixed import QMNFRational

        tests = {
            "creation": lambda: QMNFRational(22, 7),
            "addition": lambda: QMNFRational(1, 2) + QMNFRational(1, 3),
            "multiplication": lambda: QMNFRational(22, 7) * QMNFRational(1, 3),
        }

        results = {}
        for name, func in tests.items():
            start = time.perf_counter()
            for _ in range(10000):
                func()
            duration = time.perf_counter() - start
            results[name] = int(10000 / duration)

        with open('current_benchmark.json', 'w') as f:
            json.dump(results, f, indent=2)
        EOF

    - name: Download baseline benchmark
      run: |
        # Download from artifacts or use committed baseline
        cp benchmarks/baseline_2025-11-06.json baseline_benchmark.json || echo '{"benchmarks":{"creation":300000,"addition":70000,"multiplication":70000}}' > baseline_benchmark.json

    - name: Compare benchmarks
      run: |
        python3 tools/compare_benchmarks.py baseline_benchmark.json current_benchmark.json --threshold 10.0
      continue-on-error: true

    - name: Upload benchmark results
      uses: actions/upload-artifact@v3
      with:
        name: benchmark-results
        path: |
          current_benchmark.json
          baseline_benchmark.json
```

---

## GitLab CI Integration

Create `.gitlab-ci.yml`:

```yaml
image: python:3.11

variables:
  PIP_CACHE_DIR: "$CI_PROJECT_DIR/.cache/pip"

cache:
  paths:
    - .cache/pip
    - hcvlang/target

stages:
  - setup
  - validate
  - test
  - benchmark

before_script:
  - apt-get update && apt-get install -y cargo rustc
  - pip install numpy scipy mpmath pytest psutil setuptools-rust

setup:build:
  stage: setup
  script:
    - pip install -e .
  artifacts:
    paths:
      - hcvlang_pyo3*.so
    expire_in: 1 hour

validate:floats:
  stage: validate
  script:
    - python3 tools/check_no_floats.py --path qmnf/ --report violations.json
  artifacts:
    when: on_failure
    paths:
      - violations.json
    expire_in: 1 week

validate:boundary:
  stage: validate
  script:
    - python3 tools/boundary_validator.py

test:unit:
  stage: test
  script:
    - python3 -m pytest tests/python/ -v --junitxml=report.xml
  artifacts:
    when: always
    reports:
      junit: report.xml

benchmark:performance:
  stage: benchmark
  script:
    - python3 -c "
      import time, json
      from qmnf_boundary_fixed import QMNFRational

      start = time.perf_counter()
      for i in range(10000):
          r = QMNFRational(i, i+1)
          _ = r + r
      duration = time.perf_counter() - start
      ops_per_sec = 10000 / duration

      result = {'ops_per_sec': int(ops_per_sec), 'target': 30000, 'status': 'PASS' if ops_per_sec >= 30000 else 'FAIL'}
      with open('benchmark.json', 'w') as f:
          json.dump(result, f, indent=2)

      print(f'Performance: {ops_per_sec:.0f} ops/sec')
      if ops_per_sec < 30000:
          exit(1)
      "
  artifacts:
    paths:
      - benchmark.json
    expire_in: 1 month
```

---

## Pre-commit Hook Integration

Create `.git/hooks/pre-commit`:

```bash
#!/bin/bash
# QMNF Pre-commit Validation Hook

echo "🔍 QMNF Pre-commit Validation"
echo "=============================="

# Check for float contamination in staged files
echo "Checking for float contamination..."
STAGED_PY_FILES=$(git diff --cached --name-only --diff-filter=ACM | grep "\.py$" | grep -E "^qmnf/")

if [ -n "$STAGED_PY_FILES" ]; then
    for file in $STAGED_PY_FILES; do
        python3 tools/check_no_floats.py --path "$file" --quiet
        if [ $? -ne 0 ]; then
            echo "❌ Float violations detected in $file"
            echo "Run: python3 tools/check_no_floats.py --path $file"
            exit 1
        fi
    done
    echo "✅ No float contamination detected"
else
    echo "ℹ️  No QMNF Python files staged"
fi

# Boundary validation
echo ""
echo "Validating boundary compliance..."
python3 tools/boundary_validator.py --quiet
if [ $? -ne 0 ]; then
    echo "❌ Boundary validation failed"
    echo "Run: python3 tools/boundary_validator.py"
    exit 1
fi
echo "✅ Boundary validation passed"

echo ""
echo "✅ All pre-commit checks passed!"
exit 0
```

Make it executable:
```bash
chmod +x .git/hooks/pre-commit
```

---

## Jenkins Integration

Create `Jenkinsfile`:

```groovy
pipeline {
    agent any

    stages {
        stage('Setup') {
            steps {
                sh '''
                    python3 -m venv venv
                    . venv/bin/activate
                    pip install numpy scipy mpmath pytest psutil setuptools-rust
                '''
            }
        }

        stage('Build') {
            steps {
                sh '''
                    . venv/bin/activate
                    pip install -e .
                '''
            }
        }

        stage('Validate') {
            parallel {
                stage('Float Check') {
                    steps {
                        sh '''
                            . venv/bin/activate
                            python3 tools/check_no_floats.py --path qmnf/ --report violations.json
                        '''
                    }
                }

                stage('Boundary Check') {
                    steps {
                        sh '''
                            . venv/bin/activate
                            python3 tools/boundary_validator.py
                        '''
                    }
                }
            }
        }

        stage('Test') {
            steps {
                sh '''
                    . venv/bin/activate
                    python3 -m pytest tests/python/ -v --junitxml=test-results.xml
                '''
            }
            post {
                always {
                    junit 'test-results.xml'
                }
            }
        }

        stage('Benchmark') {
            steps {
                sh '''
                    . venv/bin/activate
                    python3 << 'EOF'
import time, json
from qmnf_boundary_fixed import QMNFRational

start = time.perf_counter()
for i in range(10000):
    r = QMNFRational(i, i+1)
    _ = r + r
duration = time.perf_counter() - start
ops_per_sec = 10000 / duration

result = {'ops_per_sec': int(ops_per_sec), 'target': 30000}
with open('benchmark.json', 'w') as f:
    json.dump(result, f, indent=2)

print(f'Performance: {ops_per_sec:.0f} ops/sec')
exit(0 if ops_per_sec >= 30000 else 1)
EOF
                '''
            }
            post {
                always {
                    archiveArtifacts artifacts: 'benchmark.json', fingerprint: true
                }
            }
        }
    }

    post {
        failure {
            echo 'QMNF validation failed!'
        }
        success {
            echo 'QMNF validation passed!'
        }
    }
}
```

---

## Docker Integration

Create `Dockerfile.ci`:

```dockerfile
FROM python:3.11-slim

# Install Rust
RUN apt-get update && apt-get install -y \
    cargo \
    rustc \
    build-essential \
    && rm -rf /var/lib/apt/lists/*

# Install Python dependencies
RUN pip install --no-cache-dir \
    numpy \
    scipy \
    mpmath \
    pytest \
    psutil \
    setuptools-rust

# Set working directory
WORKDIR /app

# Copy project
COPY . .

# Build and install
RUN pip install -e .

# Default command runs all validations
CMD ["sh", "-c", " \
    python3 tools/check_no_floats.py --path qmnf/ && \
    python3 tools/boundary_validator.py && \
    python3 -m pytest tests/python/ -v \
"]
```

Usage:
```bash
docker build -f Dockerfile.ci -t qmnf-ci .
docker run --rm qmnf-ci
```

---

## Validation Checklist

Use this checklist for manual or automated validation:

- [ ] **Float Contamination Check**
  ```bash
  python3 tools/check_no_floats.py --path qmnf/
  ```

- [ ] **Boundary Compliance Check**
  ```bash
  python3 tools/boundary_validator.py
  ```

- [ ] **Test Suite**
  ```bash
  python3 -m pytest tests/python/ -v
  ```

- [ ] **Performance Benchmark**
  ```bash
  # Should be >30,000 ops/sec
  python3 -c "import time; from qmnf_boundary_fixed import QMNFRational; start = time.perf_counter(); [QMNFRational(i, i+1) + QMNFRational(i, i+1) for i in range(10000)]; print(f'{10000/(time.perf_counter()-start):.0f} ops/sec')"
  ```

- [ ] **Build Verification**
  ```bash
  python3 -c "import hcvlang_pyo3, qmnf; print('✓ Imports OK')"
  ```

---

## Continuous Monitoring

### Metrics to Track

1. **Float Violations**: Should be 0 in core modules
2. **Test Pass Rate**: Should be 100%
3. **Performance**: Should exceed 30,000 ops/sec
4. **Build Time**: Track Rust compilation time
5. **Code Coverage**: Target >80% for core modules

### Alerting Thresholds

```yaml
alerts:
  - name: float_contamination
    condition: violations > 0
    severity: critical
    action: block_merge

  - name: performance_regression
    condition: ops_per_sec < 30000
    severity: high
    action: notify_team

  - name: test_failure
    condition: pass_rate < 100%
    severity: high
    action: block_merge

  - name: boundary_violation
    condition: boundary_errors > 0
    severity: critical
    action: block_merge
```

---

## Troubleshooting CI/CD Issues

### Common Problems

**Problem**: Float checker reports false positives
**Solution**: Check `ALLOWED_FLOAT_MODULES` in `tools/check_no_floats.py`

**Problem**: Benchmarks fail intermittently
**Solution**: Use multiple runs and average results; add warmup iterations

**Problem**: Rust build times out
**Solution**: Cache `hcvlang/target` directory; use incremental builds

**Problem**: Tests pass locally but fail in CI
**Solution**: Ensure same Python/Rust versions; check for environment-specific code

---

## Best Practices

1. **Run validations on every commit** - Catch issues early
2. **Cache build artifacts** - Speed up CI/CD pipeline
3. **Parallel execution** - Run validations concurrently
4. **Fail fast** - Stop on critical violations
5. **Store baselines** - Track performance over time
6. **Regular updates** - Keep baseline benchmarks current

---

## Support

- **Documentation**: `SYSTEM_RECOVERY_GUIDE.md` for build issues
- **Validation Tools**: `tools/` directory
- **Test Suite**: `tests/python/` directory

---

**Maintained by**: QMNF Development Team
**Last Updated**: 2025-11-06
**Version**: 1.0
