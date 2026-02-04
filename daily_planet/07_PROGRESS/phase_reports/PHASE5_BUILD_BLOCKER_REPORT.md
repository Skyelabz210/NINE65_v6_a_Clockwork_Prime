# Phase 5 Crypto Build - Dependency Installation Required

**Date**: 2025-10-23
**Status**: ⏸️ **BLOCKED - Missing Build Tools**
**Resolution**: User must install CMake, g++, and Python development headers

---

## Issue Summary

Attempted to build the QMNF CryptoEngine C++ module but encountered missing system dependencies. The build system requires CMake, a C++ compiler (g++), and Python development headers that are not currently installed.

## Missing Dependencies

| Package | Status | Purpose |
|---------|--------|---------|
| cmake | ❌ Not installed | Build system generator |
| g++ | ❌ Not installed | C++ compiler for PQClean + qmnf_crypto.cpp |
| python3-devel | ❌ Not installed | Python.h headers for PyBind11 |

## Available Tools

| Package | Status | Location |
|---------|--------|----------|
| gcc | ✅ Installed | /usr/bin/gcc |
| make | ✅ Installed | /usr/bin/make |
| pybind11 | ✅ Installed | Python package (pip3) |

---

## Installation Commands

### For Fedora/RHEL-based Systems

```bash
sudo dnf install -y cmake gcc-c++ python3-devel
```

**Explanation**:
- `cmake`: Build system that generates Makefiles from CMakeLists.txt
- `gcc-c++`: C++ compiler (provides g++)
- `python3-devel`: Python development headers (Python.h, pyconfig.h)

### Verification

After installation, verify with:

```bash
which cmake g++ python3-config
```

Expected output:
```
/usr/bin/cmake
/usr/bin/g++
/usr/bin/python3-config
```

---

## Build Process (After Installation)

Once dependencies are installed, run:

```bash
cd /home/acid/QMNF_System/qmnf/crypto/cpp
./build.sh test
```

**Expected Output**:
1. CMake configures build (detects PQClean, pybind11, Python)
2. Compilation proceeds with `-j$(nproc)` parallel jobs
3. Float detection check passes (0 violations)
4. Module installed to `qmnf/crypto/qmnf_crypto.so`
5. Basic functionality test passes (SHA-3 hash computation)

---

## Build Time Estimate

- **Configuration (CMake)**: 5-10 seconds
- **Compilation**: 15-30 seconds (depends on CPU cores)
- **Float detection check**: <1 second
- **Installation**: <1 second
- **Testing**: <1 second

**Total**: ~20-40 seconds

---

## Why These Dependencies Are Required

### CMake (Version ≥ 3.15)

CMake reads `CMakeLists.txt` and generates platform-specific Makefiles. It:
- Detects Python installation and headers
- Finds pybind11 library
- Configures include paths for PQClean source
- Sets compiler flags (-O3, -march=native, -Werror=float-conversion)
- Creates custom targets (check_floats, test_crypto)

**Cannot be substituted**: The build system is specifically designed for CMake.

### g++ (C++17 compiler)

The C++ compiler compiles:
- `qmnf_crypto.cpp` (540 lines of PyBind11 bindings)
- PQClean ML-KEM-1024 C source (~3,000 lines)
- PQClean ML-DSA-87 C source (~4,000 lines)
- PQClean common/fips202.c (SHA-3, ~500 lines)

**gcc (C compiler) is insufficient**: While gcc can compile C code, it cannot compile C++ code or link C++ standard library. g++ is required.

**C++17 features used**:
- `constexpr` variables
- Structured bindings (potentially)
- `std::vector`, `std::string`
- PyBind11 template metaprogramming

### python3-devel (Development Headers)

PyBind11 requires Python headers to:
- Access `Python.h` (CPython API definitions)
- Get `pyconfig.h` (platform-specific configuration)
- Link against `libpython3.so` (shared library)

**Without these headers**: Compilation fails with errors like:
```
fatal error: Python.h: No such file or directory
```

---

## Alternative: Docker Build (Advanced)

If unable to install system packages, a containerized build is possible:

```bash
# Create Dockerfile
cat > /tmp/Dockerfile.qmnf <<'EOF'
FROM fedora:42
RUN dnf install -y cmake gcc-c++ python3-devel make
WORKDIR /build
CMD ["bash"]
EOF

# Build container and compile
docker build -t qmnf-builder -f /tmp/Dockerfile.qmnf /tmp
docker run -v /home/acid/QMNF_System:/build qmnf-builder \
    bash -c "cd /build/qmnf/crypto/cpp && ./build.sh test"
```

**Note**: This approach requires Docker installed and configured.

---

## Next Steps After Installation

1. **Install dependencies** (run the dnf command above)
2. **Verify installation** (`which cmake g++ python3-config`)
3. **Run build script** (`./build.sh test`)
4. **Verify module import**:
   ```bash
   python3 -c "import qmnf.crypto.qmnf_crypto as qc; print('✓ Module loaded')"
   ```
5. **Run Known-Answer Tests** (verify cryptographic correctness)
6. **Benchmark performance** (compare vs. native PQClean)

---

## Current Status Summary

| Component | Status | Notes |
|-----------|--------|-------|
| C++ source code | ✅ Complete | 540 lines, float-free |
| Python float guards | ✅ Complete | 430 lines, recursive validation |
| CMake build system | ✅ Complete | 120 lines, automated |
| Build script | ✅ Complete | 60 lines, dependency checking |
| PQClean source | ✅ Verified | 0 float violations |
| Build tools | ❌ Missing | Requires: cmake, g++, python3-devel |
| Compilation | ⏳ Pending | Awaiting dependency installation |
| Testing | ⏳ Pending | Awaiting successful build |

**Completion**: 85% (implementation complete, build pending)

---

## Estimated Time to Resolution

- **Dependency installation**: 2-5 minutes (download + install)
- **Build and test**: 1 minute
- **Total**: ~3-6 minutes of user time

---

## Contact

If issues persist after installing dependencies, check:
1. CMake version: `cmake --version` (must be ≥ 3.15)
2. g++ version: `g++ --version` (must support C++17)
3. Python version: `python3 --version` (must be 3.x)
4. Build log: Check error messages in `qmnf/crypto/cpp/build/` directory

---

**Next Action**: Run the dependency installation command, then retry the build.

**Command**: `sudo dnf install -y cmake gcc-c++ python3-devel`
