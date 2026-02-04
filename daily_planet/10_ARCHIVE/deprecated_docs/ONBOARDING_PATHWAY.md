# QMNF System - Comprehensive Onboarding Pathway

**Mission**: Transform new users into productive power users in 2 hours, not 2 days.

**Last Updated**: 2025-11-16
**Status**: Production Ready

---

## What is QMNF?

**In One Sentence**: QMNF is a high-performance computational arithmetic library providing 100% integer-only mathematics with exact rational arithmetic—no floating-point approximations.

**Why It Matters**: Achieve exact computation at arbitrary scale with competitive performance (~120ns operations), enabling applications in cryptography, neural networks, scientific computing, and storage systems.

**What You Can Do With It**:
- Perform exact rational arithmetic without floating-point errors
- Build cryptographic systems with homomorphic encryption
- Train neural networks using integer-only mathematics
- Store and retrieve data with holographic encoding
- Implement custom numerical algorithms with precision guarantees

---

## Choose Your Path (4 User Personas)

### 🔬 **Researcher** - Understand Algorithms and Math
**Goal**: Understand the mathematical foundations and algorithmic innovations

**Start Here**: [GETTING_STARTED_RESEARCHER.md](GETTING_STARTED_RESEARCHER.md)

**You Care About**:
- Mathematical proofs and formal verification
- Algorithm complexity and correctness
- Novel computational techniques (CRT, deferred reconstruction, adaptive precision)
- Research applications and publications

**Time Investment**: 3-5 hours for deep understanding

---

### 🛠️ **Engineer** - Build Features and Integrate Systems
**Goal**: Build production features and integrate QMNF into existing systems

**Start Here**: [GETTING_STARTED_ENGINEER.md](GETTING_STARTED_ENGINEER.md)

**You Care About**:
- API reference and usage patterns
- Integration with existing codebases
- Performance optimization techniques
- Testing and validation

**Time Investment**: 1-2 hours to productive coding

---

### 📊 **Data Scientist** - Train Models and Run Inference
**Goal**: Train neural networks and run inference using integer-only mathematics

**Start Here**: [GETTING_STARTED_DATA_SCIENTIST.md](GETTING_STARTED_DATA_SCIENTIST.md)

**You Care About**:
- Model training workflows
- Dataset handling with exact arithmetic
- Performance benchmarking
- Inference optimization

**Time Investment**: 2-3 hours to first model

---

### 🚀 **DevOps** - Deploy, Monitor, and Optimize
**Goal**: Deploy QMNF to production, monitor performance, and optimize infrastructure

**Start Here**: [GETTING_STARTED_DEVOPS.md](GETTING_STARTED_DEVOPS.md)

**You Care About**:
- Installation and dependencies
- Environment configuration
- Performance monitoring
- Production deployment patterns

**Time Investment**: 1 hour to production deployment

---

## Progressive Complexity Pathway

### 📍 **Level 0: "Hello QMNF"** (5 minutes)

**Objective**: Understand what QMNF is and verify it works

**Read**: [QUICK_START.md](QUICK_START.md)

**Do**:
```bash
# Install and verify
cd QMNF_System
python3 setup.py build_rust --release --inplace
python3 -c "from qmnf import QMNFRational; print(QMNFRational(22, 7))"
```

**Success Criteria**: ✅ You can import QMNF and create a rational number

---

### 📍 **Level 1: "First Operation"** (15 minutes)

**Objective**: Run a simple computation and verify correctness

**Read**: [QUICK_START.md](QUICK_START.md) - "First Steps" section

**Do**:
```python
from qmnf import QMNFRational

# Create exact rationals
pi = QMNFRational(22, 7)
phi = QMNFRational(1618, 1000)

# Perform exact arithmetic
result = pi * phi
print(f"π × φ = {result}")  # Exact: 35596/7000

# Verify no precision loss
print(float(result))  # Approximation: 5.085142857...
```

**Run Validation**:
```bash
python3 tools/verify_installation.py
```

**Success Criteria**: ✅ You can perform arithmetic operations with exact results

---

### 📍 **Level 2: "Building Blocks"** (1 hour)

**Objective**: Understand core components and performance characteristics

**Read**:
- [QUICK_START.md](QUICK_START.md) - Complete guide
- [COMMON_TASKS_COOKBOOK.md](COMMON_TASKS_COOKBOOK.md) - Sections 1-3

**Understand**:
1. **QMNFRational**: Python wrapper for exact rational arithmetic
2. **CRTBigInt**: Fast bounded integers (±2^126, ~120ns ops)
3. **HCVLangBigInt**: Arbitrary precision integers (unlimited scale)
4. **Boundary Layer**: Safe conversion between float/int and rational

**Do**:
```python
# Test basic operations
python3 tools/test_basic_operations.py

# Understand performance
python3 milestone_benchmark.py

# Read results
cat benchmarks/results/latest.json
```

**Success Criteria**:
✅ You understand the three-tier architecture (QMNFRational → CRTBigInt → HCVLangBigInt)
✅ You can explain why QMNF avoids floats
✅ You can measure operation performance

---

### 📍 **Level 3: "Real Use Case"** (3 hours)

**Objective**: Build something meaningful in your domain

**Choose Your Domain**:

#### **Cryptography** (FHE)
**Read**: [FHE_DELIVERABLES_INDEX.md](FHE_DELIVERABLES_INDEX.md)

**Do**:
```python
from hcvlang import FHEContext, SecurityLevel

ctx = FHEContext(SecurityLevel(1))  # 128-bit security
sk, pk = ctx.generate_keypair()

# Encrypt
ct1 = ctx.encrypt(ctx.encode(10), pk)
ct2 = ctx.encrypt(ctx.encode(32), pk)

# Homomorphic addition
ct_sum = ctx.add(ct1, ct2)
result = ctx.decode(ctx.decrypt(ct_sum, sk))
print(f"10 + 32 = {result}")  # 42 (computed on encrypted data!)
```

#### **Neural Networks**
**Read**: [GETTING_STARTED_DATA_SCIENTIST.md](GETTING_STARTED_DATA_SCIENTIST.md)

**Do**:
```python
from qmnf.neural.helix_compiler import HelixNeuralNet

model = HelixNeuralNet(input_dim=784, hidden_dim=128, output_dim=10)
# Train with integer gradients (see guide for full example)
```

#### **Storage Systems**
**Read**: [INTEGRATION_QUICK_REFERENCE.md](INTEGRATION_QUICK_REFERENCE.md) - COSMOS section

**Do**:
```python
from qmnf.storage.cosmos import COSMOSBackend

cosmos = COSMOSBackend(capacity=1024**2)  # 1MB
cosmos.store(key="data", value=tensor)
retrieved = cosmos.retrieve(key="data")
```

**Success Criteria**: ✅ You built a working prototype in your domain

---

### 📍 **Level 4: "Production Ready"** (1 day)

**Objective**: Deploy to production with monitoring and optimization

**Read**:
- [BEST_PRACTICES.md](BEST_PRACTICES.md)
- [TROUBLESHOOTING_PLAYBOOK.md](TROUBLESHOOTING_PLAYBOOK.md)
- [GETTING_STARTED_DEVOPS.md](GETTING_STARTED_DEVOPS.md)

**Do**:

1. **Performance Tuning**:
```bash
# Run comprehensive benchmarks
python3 tools/qmnf_benchmark_suite.py

# Profile your application
python3 -m cProfile -o profile.stats your_app.py
python3 -c "import pstats; p=pstats.Stats('profile.stats'); p.sort_stats('cumulative').print_stats(20)"
```

2. **Error Handling**:
```python
from qmnf.conversion_boundary import DataBoundary
from qmnf import QMNFRational

try:
    # Safe boundary conversion
    r = DataBoundary.float_to_rational(user_input, precision=10)
    result = process(QMNFRational(r.numerator, r.denominator))
except ValueError as e:
    logger.error(f"Invalid input: {e}")
    # Handle gracefully
```

3. **Integration Testing**:
```bash
python3 tools/test_integration.py
```

4. **Deployment**:
```bash
# Build optimized Rust library
cd hcvlang
cargo build --release

# Set production environment
export LD_LIBRARY_PATH=/opt/qmnf:$LD_LIBRARY_PATH
export PYTHONPATH=/opt/qmnf:$PYTHONPATH

# Run production tests
pytest tests/ --cov=qmnf --cov-report=html
```

**Success Criteria**:
✅ Application handles errors gracefully
✅ Performance meets targets
✅ Tests pass in production environment
✅ Monitoring and logging in place

---

## Documentation Structure

### **Quick Access** (5-15 minutes)
- [QUICK_START.md](QUICK_START.md) - Get started in 5 minutes
- [READING_ORDER.md](READING_ORDER.md) - Navigate all documentation
- [COMMON_TASKS_COOKBOOK.md](COMMON_TASKS_COOKBOOK.md) - "How do I...?" reference

### **Persona-Specific Guides** (1-2 hours)
- [GETTING_STARTED_RESEARCHER.md](GETTING_STARTED_RESEARCHER.md)
- [GETTING_STARTED_ENGINEER.md](GETTING_STARTED_ENGINEER.md)
- [GETTING_STARTED_DATA_SCIENTIST.md](GETTING_STARTED_DATA_SCIENTIST.md)
- [GETTING_STARTED_DEVOPS.md](GETTING_STARTED_DEVOPS.md)

### **Reference Documentation** (as needed)
- [INTEGRATION_QUICK_REFERENCE.md](INTEGRATION_QUICK_REFERENCE.md) - API reference
- [SYSTEM_DEVELOPER_GUIDE.md](SYSTEM_DEVELOPER_GUIDE.md) - Complete architecture
- [FFI_BRIDGE_ANALYSIS.md](FFI_BRIDGE_ANALYSIS.md) - Performance optimization
- [CLAUDE.md](CLAUDE.md) - Development guidelines

### **Best Practices** (30 minutes)
- [BEST_PRACTICES.md](BEST_PRACTICES.md) - Production patterns
- [TROUBLESHOOTING_PLAYBOOK.md](TROUBLESHOOTING_PLAYBOOK.md) - Problem solving

### **Mathematical Foundations** (3-5 hours for deep understanding)
- `docs/mathematical/` - Proofs and formal verification
- [QMNF_MATHEMATICAL_REFERENCE_MANUAL.md](QMNF_MATHEMATICAL_REFERENCE_MANUAL.md)

---

## Validation Checkpoints

QMNF provides automated validation scripts to verify your setup at each level:

### **Checkpoint 1: Installation**
```bash
python3 tools/verify_installation.py
```
Confirms:
- ✅ Rust library built successfully
- ✅ Python imports working
- ✅ Environment variables set

### **Checkpoint 2: Basic Operations**
```bash
python3 tools/test_basic_operations.py
```
Confirms:
- ✅ QMNFRational creation
- ✅ Arithmetic operations
- ✅ Boundary conversions
- ✅ Performance baseline

### **Checkpoint 3: Integration**
```bash
python3 tools/test_integration.py
```
Confirms:
- ✅ FFI bindings working
- ✅ Subsystems integrated
- ✅ No float contamination
- ✅ Tests passing

### **Checkpoint 4: Production Readiness**
```bash
python3 tools/production_readiness_check.py
```
Confirms:
- ✅ All tests pass
- ✅ Performance meets targets
- ✅ Error handling implemented
- ✅ Logging configured

---

## Common Tasks Guide

### "How do I...?"

Quick links to [COMMON_TASKS_COOKBOOK.md](COMMON_TASKS_COOKBOOK.md):

- **Add integers without floating points** → Section 1.1
- **Convert floats safely** → Section 1.2
- **Perform batch operations** → Section 2.1
- **Encrypt data homomorphically** → Section 3.1
- **Train a neural network** → Section 4.1
- **Store large data efficiently** → Section 5.1
- **Optimize performance** → Section 6.1
- **Deploy to production** → Section 7.1
- **Debug issues** → [TROUBLESHOOTING_PLAYBOOK.md](TROUBLESHOOTING_PLAYBOOK.md)

---

## Troubleshooting Quick Reference

Common issues and solutions:

| Problem | Solution | Details |
|---------|----------|---------|
| Import errors | Set environment variables | [Troubleshooting §1.1](TROUBLESHOOTING_PLAYBOOK.md#11-import-errors) |
| Build failures | Check dependencies | [Troubleshooting §1.2](TROUBLESHOOTING_PLAYBOOK.md#12-build-failures) |
| Slow performance | Use batch operations | [Troubleshooting §2.1](TROUBLESHOOTING_PLAYBOOK.md#21-slow-performance) |
| Type errors | Use DataBoundary | [Troubleshooting §3.1](TROUBLESHOOTING_PLAYBOOK.md#31-type-errors) |
| Test failures | Check float violations | [Troubleshooting §4.1](TROUBLESHOOTING_PLAYBOOK.md#41-test-failures) |

**Full Playbook**: [TROUBLESHOOTING_PLAYBOOK.md](TROUBLESHOOTING_PLAYBOOK.md)

---

## Success Milestones

Track your progress through the onboarding pathway:

- [ ] **Level 0 Complete**: Installed QMNF and ran "Hello World"
- [ ] **Level 1 Complete**: Performed first computation
- [ ] **Level 2 Complete**: Understand core architecture
- [ ] **Level 3 Complete**: Built domain-specific prototype
- [ ] **Level 4 Complete**: Production deployment ready

**Target**: Complete Levels 0-3 in 2 hours

---

## Next Steps After Onboarding

### **Continue Learning**
- Read [SYSTEM_DEVELOPER_GUIDE.md](SYSTEM_DEVELOPER_GUIDE.md) for deep architecture understanding
- Explore [FFI_BRIDGE_ANALYSIS.md](FFI_BRIDGE_ANALYSIS.md) for performance optimization
- Study mathematical foundations in `docs/mathematical/`

### **Contribute**
- Report issues on GitHub
- Submit pull requests for improvements
- Share your use cases and applications

### **Get Support**
- 📧 Email: founder@hackfate.us
- 🌐 Website: www.hackfate.us
- 📝 GitHub Issues: Submit detailed bug reports

---

## Recommended Reading Order

See [READING_ORDER.md](READING_ORDER.md) for complete documentation navigation guide.

**Quick Path** (2 hours):
1. QUICK_START.md (5 min)
2. Your persona guide (30 min)
3. COMMON_TASKS_COOKBOOK.md (20 min)
4. Build your first prototype (60 min)

**Deep Path** (1 day):
1. QUICK_START.md (5 min)
2. Your persona guide (1 hour)
3. SYSTEM_DEVELOPER_GUIDE.md (2 hours)
4. FFI_BRIDGE_ANALYSIS.md (1 hour)
5. Mathematical foundations (2 hours)
6. Build production application (remaining time)

---

## Summary

**You Are Here**: Start of your QMNF journey

**Next Action**: Choose your persona and open the corresponding guide:
- 🔬 [GETTING_STARTED_RESEARCHER.md](GETTING_STARTED_RESEARCHER.md)
- 🛠️ [GETTING_STARTED_ENGINEER.md](GETTING_STARTED_ENGINEER.md)
- 📊 [GETTING_STARTED_DATA_SCIENTIST.md](GETTING_STARTED_DATA_SCIENTIST.md)
- 🚀 [GETTING_STARTED_DEVOPS.md](GETTING_STARTED_DEVOPS.md)

**Or**: If you just want to get started immediately → [QUICK_START.md](QUICK_START.md)

---

**Last Updated**: 2025-11-16
**Maintainer**: Anthony Diaz (founder@hackfate.us)
**License**: Proprietary - See LICENSE file
