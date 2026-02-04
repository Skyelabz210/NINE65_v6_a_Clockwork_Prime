# Merge Conflict Quick Reference Guide

A quick reference for handling merge conflicts in the QMNF System repository.

---

## Quick Start

### When You Encounter a Merge Conflict

```bash
# 1. See what's conflicted
git status

# 2. Open conflicted files and look for markers

# 3. Resolve each conflict manually
# - Understand both changes
# - Keep what's needed
# - Remove conflict markers
# - Test the result

# 4. Stage resolved files
git add <resolved-file>

# 5. Continue merge
git merge --continue
# or
git rebase --continue
```

---

## Common Conflict Types

### 1. Code Conflicts

**Duplicate Functions/Structs:**
```rust
pub struct MyStruct {
    field_a: i32,
}
```

**Resolution:** Keep the version with more fields (feature-branch), or merge both if truly different:
```rust
pub struct MyStruct {
    field_a: i32,
    field_b: String,  // Keep new field from feature
}
```

### 2. Import Conflicts

**Conflicting Imports:**
```rust
use crate::module_a::Function;
```

**Resolution:** Understand which module is correct, or alias if both needed:
```rust
use crate::module_a::Function as FunctionA;
use crate::module_b::Function as FunctionB;
```

### 3. Documentation Conflicts

**Different Docstrings:**
```python
def process(data):
    """Process data."""
```

**Resolution:** Keep more descriptive version or merge information:
```python
def process(data):
    """
    Process data with advanced algorithm.
    
    Uses optimized processing for large datasets.
    """
```

---

## Resolution Strategies

### Strategy Decision Matrix

| Situation | Action |
|-----------|--------|
| Identical code in both | Keep one, delete duplicate |
| Complementary changes | Merge both changes |
| Conflicting logic | Choose best, test thoroughly |
| Unclear which is correct | Consult original authors |

---

## Testing After Resolution

### Minimum Required

```bash
# For Rust changes
cargo build --release
cargo test

# For Python changes
pytest tests/ -v
python -m mypy qmnf

# For both
make quality-check
```

### Comprehensive Testing

```bash
# Full test suite
cargo test --all-features
pytest tests/ -v --cov=qmnf

# Integration tests
python -c "import qmnf; print('OK')"

# Benchmarks (if performance-critical)
cargo bench
```

---

## Don'ts

❌ **Never do this:**

1. **Don't blindly keep both:**
   ```bash
   # This often creates duplicates
   git checkout --ours .
   git checkout --theirs .
   ```

2. **Don't commit without testing:**
   ```bash
   git add .
   git commit -m "fixed conflicts"  # ❌ No testing!
   ```

3. **Don't ignore conflict markers:**
   ```rust
   // ❌ This will not compile!
   let x = 1;
   ```

4. **Don't resolve without understanding:**
   - Always read both versions
   - Understand the intent
   - Test the result

---

## Do's

✅ **Always do this:**

1. **Understand both sides:**
   ```bash
   git log HEAD..feature-branch --oneline
   git diff HEAD...feature-branch
   ```

2. **Test after each file:**
   ```bash
   # Resolve one file
   git add file.rs
   cargo build  # Verify it compiles
   ```

3. **Document complex resolutions:**
   ```bash
   git commit -m "Merge feature-branch: resolve conflicts
   
   - file.rs: kept feature-branch version (more complete)
   - module.py: merged both changes (complementary)
   - test: all tests pass
   "
   ```

4. **Ask for help if unsure:**
   - Contact the original authors
   - Review PR discussions
   - Consult team members

---

## Common Patterns in QMNF

### FFI Layer Conflicts

**Problem:** Duplicate PyO3 bindings

**Solution:**
```rust
// Keep ONE definition
#[pyclass]
pub struct PyStructName {
    inner: StructName,
}

// Remove duplicate definitions
// Verify FFI tests pass
```

### Module Structure Conflicts

**Problem:** Different import paths

**Solution:**
```rust
// Use consistent structure
use crate::module::submodule::Item;  // ✅ Correct path

// Not
use crate::tests::module::Item;      // ❌ Wrong location
```

### Configuration Conflicts

**Problem:** Different constant values

**Solution:**
```python
# Choose one source of truth
from qmnf.unified_config import MODULUS

# Not scattered definitions
# MODULUS = 2147483647  # ❌ Don't redefine
```

---

## Abort and Restart

If resolution becomes too complex:

```bash
# Abort merge
git merge --abort
# or
git rebase --abort

# Start over with better strategy
git merge --strategy=recursive -X theirs feature-branch
# Review and test carefully
```

---

## Prevention

### Before Merging

```bash
# 1. Update your branch
git checkout feature-branch
git rebase master

# 2. Resolve conflicts in feature branch
# (easier than resolving in master)

# 3. Test thoroughly
cargo test --all
pytest tests/ -v

# 4. Then merge to master
git checkout master
git merge feature-branch  # Should be clean now
```

### During Development

```bash
# Keep your branch updated
git fetch origin
git rebase origin/master

# Resolve small conflicts incrementally
# Easier than one big conflict at the end
```

---

## Tools

### VS Code

- Built-in merge conflict resolver
- Shows both sides side-by-side
- "Accept Current" / "Accept Incoming" / "Accept Both" buttons
- ⚠️ Use "Accept Both" carefully!

### Command Line

```bash
# See conflict markers
git diff --check

# Visual diff tool
git mergetool

# See what changed in each branch
git log --merge --oneline
```

### Git Config

```bash
# Better conflict markers
git config merge.conflictstyle diff3

# Now markers show:
# (your changes)
# ||||||| base
# (original)
```

---

## Get Help

### Documentation

- Full guide: `MERGE_CONFLICT_RESOLUTION_REPORT.md`
- Contributing: `CONTRIBUTING.md` (merge section)
- Git docs: https://git-scm.com/book/en/v2/Git-Branching-Basic-Branching-and-Merging

### Contact

- Technical help: support@hackfate.us
- Complex merges: Consult maintainers
- Emergencies: founder@hackfate.us

## Local Pre-commit Hook Setup (Recommended)

To ensure the repository validation runs locally before each commit, enable the `.githooks` pre-commit hook described in the repository:

```bash
# Configure git to use bundled hooks
git config core.hooksPath .githooks
chmod +x .githooks/pre-commit
# Optional: test the checks now
./scripts/run_all_checks.sh
```

This will run the `run_all_checks.sh` script and prevent commits with unresolved merge markers or duplicate definitions.

### Using pre-commit framework (Recommended)
If you prefer the `pre-commit` toolchain (Python), the repository includes a `.pre-commit-config.yaml`. To install and enable it:

```bash
pip install pre-commit
pre-commit install
pre-commit run --all-files
```

This integrates checks into commits automatically.

### AI Agent Model Defaults
The repository contains `ai_agent_models.yml` which documents recommended model defaults. To align your tooling with repo recommendations, set your agent's default model to `raptor-mini-preview` and ensure your provider enables it for the agent.

---

## Example Walkthrough

### Scenario: Merging feature branch with conflicts

```bash
# 1. Start merge
$ git merge feature-neural-network
Auto-merging hcvlang/src/ffi.rs
CONFLICT (content): Merge conflict in hcvlang/src/ffi.rs
Automatic merge failed; fix conflicts and then commit the result.

# 2. Check status
$ git status
You have unmerged paths.
  (fix conflicts and run "git commit")

Unmerged paths:
  (use "git add <file>..." to mark resolution)
        both modified:   hcvlang/src/ffi.rs

# 3. Open file and see:
pub struct PyNeuralNet {
    inner: NeuralNet,
}

# 4. Resolve (keep enhanced version):
pub struct PyNeuralNet {
    inner: NeuralNet,
    training_mode: bool,  // Keep new field
}

# 5. Test
$ cargo build --release
   Compiling hcvlang v0.1.0
    Finished release [optimized] target(s) in 2.3s

$ cargo test
   running 47 tests
   test result: ok. 47 passed; 0 failed

# 6. Stage and commit
$ git add hcvlang/src/ffi.rs
$ git commit -m "Merge feature-neural-network: add training_mode field

- Resolved conflict in ffi.rs by keeping enhanced PyNeuralNet
- Added training_mode field from feature branch
- All tests pass, build successful"

# 7. Done!
$ git log --oneline -3
abc1234 Merge feature-neural-network: add training_mode field
def5678 Add training mode support
ghi9012 Initial neural network implementation
```

---

**Last Updated:** November 26, 2025

**Quick Tip:** When in doubt, test frequently and document your decisions!
