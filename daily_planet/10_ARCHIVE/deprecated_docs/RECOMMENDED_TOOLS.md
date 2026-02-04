# Recommended Development Tools for QMNF System

Comprehensive guide to GitHub tools and online services you should be using for maximum productivity.

---

## Table of Contents

1. [GitHub Native Tools](#github-native-tools)
2. [CI/CD & Automation](#cicd--automation)
3. [Code Quality & Security](#code-quality--security)
4. [Documentation & Knowledge Management](#documentation--knowledge-management)
5. [Collaboration & Project Management](#collaboration--project-management)
6. [Monitoring & Observability](#monitoring--observability)
7. [Development Environments](#development-environments)
8. [Rust-Specific Tools](#rust-specific-tools)
9. [Python-Specific Tools](#python-specific-tools)
10. [Performance & Profiling](#performance--profiling)

---

## GitHub Native Tools

### 1. **GitHub Actions** ⭐ **PRIORITY**
**What:** Automated CI/CD workflows
**Why:** Automate testing, building, and deployment
**Cost:** 2000 free minutes/month

**Setup for QMNF:**
```yaml
# .github/workflows/ci.yml
name: QMNF CI

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - name: Build Rust
        run: cd hcvlang && cargo build --release
      - name: Test Rust
        run: cd hcvlang && cargo test --release
      - name: Setup Python
        uses: actions/setup-python@v4
        with:
          python-version: '3.11'
      - name: Test Python
        run: |
          pip install pytest
          python3 -m pytest tests/python/ -v
      - name: Check Float Contamination
        run: python3 tools/check_no_floats.py
```

**Recommended workflows:**
- ✅ Run tests on every PR
- ✅ Check for float contamination
- ✅ Build documentation
- ✅ Run benchmarks on release
- ✅ Deploy docs to GitHub Pages

### 2. **GitHub Projects** ⭐ **PRIORITY**
**What:** Built-in project management (Kanban boards)
**Why:** Track issues, PRs, and roadmap in one place
**Cost:** Free

**How to set up:**
1. Go to repository → Projects tab
2. Click "New project"
3. Choose "Board" template
4. Create columns: Backlog, In Progress, Review, Done
5. Add automation:
   - Auto-move to "In Progress" when PR opened
   - Auto-move to "Done" when PR merged

**Views to create:**
- 📋 By milestone (FFI modernization, Phase 2, etc.)
- 📋 By priority (P0, P1, P2)
- 📋 By module (Rust, Python, Docs)
- 📋 By assignee

### 3. **GitHub Issues** ⭐ **PRIORITY**
**What:** Bug tracking and feature requests
**Why:** Organize development work
**Cost:** Free

**Templates to create:**

Create `.github/ISSUE_TEMPLATE/bug_report.md`:
```markdown
---
name: Bug Report
about: Report a bug in QMNF System
labels: bug
---

**Description**
Clear description of the bug

**To Reproduce**
Steps to reproduce:
1. ...
2. ...

**Expected Behavior**
What should happen

**Actual Behavior**
What actually happens

**Environment**
- OS: [e.g., Ubuntu 22.04]
- Rust version: [e.g., 1.75]
- Python version: [e.g., 3.11]

**Float Contamination Check**
Did you run `python3 tools/check_no_floats.py`? Yes/No
```

Create `.github/ISSUE_TEMPLATE/feature_request.md`:
```markdown
---
name: Feature Request
about: Suggest a feature for QMNF System
labels: enhancement
---

**Feature Description**
What feature do you want?

**Use Case**
Why is this needed?

**Proposed Implementation**
High-level approach (Rust/Python/both?)

**Performance Considerations**
Expected impact on performance

**Integer-Only Compliance**
How will this maintain integer-only architecture?
```

**Labels to create:**
- `P0-critical` (red)
- `P1-high` (orange)
- `P2-medium` (yellow)
- `P3-low` (green)
- `rust` (orange)
- `python` (blue)
- `ffi` (purple)
- `performance` (red)
- `documentation` (cyan)
- `float-contamination` (red)

### 4. **GitHub Discussions**
**What:** Community forum for Q&A
**Why:** Separate discussions from issues
**Cost:** Free

**Categories to create:**
- 💬 General
- 💡 Ideas
- 🙋 Q&A
- 📢 Announcements
- 🐛 Bug Reports (before creating issue)
- 🚀 Show and Tell

### 5. **GitHub Wiki**
**What:** Built-in documentation system
**Why:** Separate docs from code
**Cost:** Free

**Pages to create:**
- Home (project overview)
- Architecture Overview
- Module Development Guide
- FFI Bridge Patterns
- Performance Optimization
- Troubleshooting
- FAQ

**Or use GitHub Pages instead** (see below)

### 6. **GitHub Pages** ⭐ **RECOMMENDED**
**What:** Free static site hosting
**Why:** Beautiful documentation website
**Cost:** Free

**Setup with mdBook (Rust documentation standard):**
```bash
# Install mdBook
cargo install mdbook

# Create book structure
mdbook init docs-site

# Build and preview
cd docs-site
mdbook serve
# Visit http://localhost:3000

# Deploy to GitHub Pages
# In .github/workflows/docs.yml
```

**Alternative: Use MkDocs (Python):**
```bash
pip install mkdocs-material
mkdocs new docs-site
cd docs-site
mkdocs serve
# Visit http://localhost:8000
```

### 7. **Dependabot** ⭐ **PRIORITY**
**What:** Automatic dependency updates
**Why:** Stay secure and up-to-date
**Cost:** Free

**Setup:**
Create `.github/dependabot.yml`:
```yaml
version: 2
updates:
  # Rust dependencies
  - package-ecosystem: "cargo"
    directory: "/hcvlang"
    schedule:
      interval: "weekly"
    open-pull-requests-limit: 5

  # Python dependencies
  - package-ecosystem: "pip"
    directory: "/"
    schedule:
      interval: "weekly"

  # GitHub Actions
  - package-ecosystem: "github-actions"
    directory: "/"
    schedule:
      interval: "weekly"
```

### 8. **Code Scanning (CodeQL)** ⭐ **RECOMMENDED**
**What:** Automated security scanning
**Why:** Find vulnerabilities automatically
**Cost:** Free for public repos

**Setup:**
1. Go to Security tab → Code scanning → Set up
2. Choose CodeQL
3. Commit `.github/workflows/codeql.yml`

### 9. **Secret Scanning**
**What:** Detect committed secrets
**Why:** Prevent credential leaks
**Cost:** Free for public repos

**Enable:**
1. Go to Settings → Code security and analysis
2. Enable "Secret scanning"

### 10. **GitHub Copilot** 💰
**What:** AI code completion
**Why:** Write code faster
**Cost:** $10/month or $100/year (free for students)

**Usage:**
- Install in VS Code/Codespaces (auto-included in devcontainer)
- Press Tab to accept suggestions
- Press Ctrl+Enter for multiple suggestions
- Comment what you want, Copilot writes it

---

## CI/CD & Automation

### 1. **GitHub Actions** (covered above) ⭐

### 2. **Codecov** ⭐ **RECOMMENDED**
**What:** Code coverage reports
**Why:** Track test coverage over time
**Cost:** Free for public repos

**Setup:**
```yaml
# In .github/workflows/coverage.yml
- name: Generate coverage
  run: |
    cargo install cargo-tarpaulin
    cargo tarpaulin --out Xml

- name: Upload to Codecov
  uses: codecov/codecov-action@v3
  with:
    files: ./cobertura.xml
```

**Get badge for README:**
```markdown
[![codecov](https://codecov.io/gh/Skyelabz210/QMNF_System/branch/main/graph/badge.svg)](https://codecov.io/gh/Skyelabz210/QMNF_System)
```

### 3. **Pre-commit Hooks** ⭐ **RECOMMENDED**
**What:** Run checks before commits
**Why:** Catch issues early
**Cost:** Free

**Setup:**
Create `.pre-commit-config.yaml`:
```yaml
repos:
  - repo: https://github.com/pre-commit/pre-commit-hooks
    rev: v4.5.0
    hooks:
      - id: trailing-whitespace
      - id: end-of-file-fixer
      - id: check-yaml
      - id: check-added-large-files

  - repo: https://github.com/psf/black
    rev: 23.12.0
    hooks:
      - id: black

  - repo: https://github.com/charliermarsh/ruff-pre-commit
    rev: v0.1.8
    hooks:
      - id: ruff

  - repo: local
    hooks:
      - id: check-floats
        name: Check for float contamination
        entry: python3 tools/check_no_floats.py
        language: system
        pass_filenames: false
```

**Install:**
```bash
pip install pre-commit
pre-commit install
```

---

## Code Quality & Security

### 1. **Clippy** (Rust) ⭐ **PRIORITY**
**Already in use!** Continue using in CI.

### 2. **Ruff** (Python) ⭐ **PRIORITY**
**Already recommended!** Fast Python linter.

### 3. **cargo-audit** ⭐ **RECOMMENDED**
**What:** Check for security vulnerabilities in Rust deps
**Why:** Stay secure
**Cost:** Free

**Setup:**
```bash
cargo install cargo-audit

# In CI
cargo audit
```

### 4. **Safety** (Python)
**What:** Check Python dependencies for vulnerabilities
**Why:** Security
**Cost:** Free for basic use

```bash
pip install safety
safety check
```

### 5. **SonarCloud** 💰
**What:** Code quality and security analysis
**Why:** Deep code analysis
**Cost:** Free for public repos, paid for private

**Features:**
- Code smells detection
- Security hotspots
- Code duplication
- Technical debt tracking

---

## Documentation & Knowledge Management

### 1. **docs.rs** ⭐ **RECOMMENDED** (Rust)
**What:** Automatic Rust documentation hosting
**Why:** Free docs for published crates
**Cost:** Free

**Publish your docs:**
```bash
# Build docs locally
cargo doc --no-deps --open

# Publish to crates.io (publishes to docs.rs automatically)
cargo publish
```

### 2. **Read the Docs** ⭐ **RECOMMENDED**
**What:** Documentation hosting (Python-focused)
**Why:** Beautiful docs, version support
**Cost:** Free for public projects

**Setup:**
1. Sign up at readthedocs.org
2. Import your GitHub repository
3. Add `.readthedocs.yml`:
```yaml
version: 2

build:
  os: ubuntu-22.04
  tools:
    python: "3.11"

sphinx:
  configuration: docs/conf.py

python:
  install:
    - requirements: docs/requirements.txt
```

### 3. **GitBook**
**What:** Modern documentation platform
**Why:** Beautiful UI, great for user-facing docs
**Cost:** Free for public projects

### 4. **Notion** (Personal use)
**What:** All-in-one workspace
**Why:** Internal notes, planning, wikis
**Cost:** Free for personal use

**Use for:**
- Architecture diagrams
- Meeting notes
- Research notes
- Roadmap planning

---

## Collaboration & Project Management

### 1. **GitHub Projects** (covered above) ⭐

### 2. **Linear** 💰
**What:** Modern issue tracker
**Why:** Faster than GitHub Issues, better UX
**Cost:** $8/user/month (free trial)

**Features:**
- Keyboard shortcuts
- Cycles (sprints)
- Roadmaps
- GitHub integration

### 3. **Excalidraw**
**What:** Quick diagramming tool
**Why:** Fast, collaborative diagrams
**Cost:** Free

**Use for:**
- Architecture diagrams
- FFI flow diagrams
- Explaining complex systems
- Quick sketches

**Link:** https://excalidraw.com

### 4. **Miro**
**What:** Online whiteboard
**Why:** Team brainstorming
**Cost:** Free for basic use

### 5. **Figma** (If building UI)
**What:** Design tool
**Why:** UI/UX design
**Cost:** Free for personal use

---

## Monitoring & Observability

### 1. **Sentry** 💰
**What:** Error tracking
**Why:** Know when things break in production
**Cost:** Free tier (5k events/month)

**Integration:**
```python
import sentry_sdk

sentry_sdk.init(
    dsn="your-dsn-here",
    traces_sample_rate=1.0
)
```

### 2. **LogRocket** 💰
**What:** Session replay for web apps
**Why:** See what users actually did
**Cost:** Free tier (1k sessions/month)

### 3. **Datadog** 💰
**What:** Full observability platform
**Why:** Metrics, logs, traces
**Cost:** Free trial, then paid

### 4. **Prometheus + Grafana** (Self-hosted)
**What:** Metrics and dashboards
**Why:** Free, powerful, self-hosted
**Cost:** Free (infrastructure costs)

---

## Development Environments

### 1. **GitHub Codespaces** ⭐ **IMPLEMENTED**
You already have this set up!

### 2. **GitPod**
**What:** Alternative to Codespaces
**Why:** Different features, sometimes faster
**Cost:** 50 hours/month free

**Setup:**
Create `.gitpod.yml`:
```yaml
image: rust:latest

tasks:
  - init: |
      cd hcvlang
      cargo build --release
  - command: |
      echo "Ready to code!"

vscode:
  extensions:
    - rust-lang.rust-analyzer
    - ms-python.python
```

### 3. **Replit**
**What:** Quick online IDE
**Why:** Fast prototyping
**Cost:** Free tier available

### 4. **StackBlitz** (Web projects)
**What:** Instant dev environment
**Why:** Fast for web projects
**Cost:** Free

---

## Rust-Specific Tools

### 1. **cargo-make** ⭐ **RECOMMENDED**
**What:** Task runner for Rust
**Why:** Complex build tasks
**Cost:** Free

**Install:**
```bash
cargo install cargo-make
```

**Create `Makefile.toml`:**
```toml
[tasks.build-all]
script = '''
cargo build --release
cargo test --release
cargo bench --no-run
'''

[tasks.check-all]
script = '''
cargo clippy --all-targets
cargo fmt -- --check
cargo audit
'''
```

**Run:**
```bash
cargo make build-all
cargo make check-all
```

### 2. **cargo-watch** ⭐ **RECOMMENDED**
**What:** Auto-rebuild on file changes
**Why:** Faster iteration
**Cost:** Free

```bash
cargo install cargo-watch
cargo watch -x "build --release"
```

### 3. **cargo-expand**
**What:** Expand macros
**Why:** Debug macro issues
**Cost:** Free

```bash
cargo install cargo-expand
cargo expand module_name
```

### 4. **cargo-flamegraph** ⭐ **RECOMMENDED**
**What:** Profile Rust code
**Why:** Find performance bottlenecks
**Cost:** Free

```bash
cargo install flamegraph
cargo flamegraph --bench my_benchmark
```

### 5. **cargo-edit**
**What:** Add/remove dependencies from CLI
**Why:** No manual TOML editing
**Cost:** Free

```bash
cargo install cargo-edit
cargo add serde
cargo rm old_dep
```

### 6. **rust-analyzer** ⭐ **PRIORITY**
**Already in Codespaces!** Best Rust IDE support.

### 7. **Criterion.rs** ⭐ **IN USE**
**Already using!** Continue benchmarking with it.

---

## Python-Specific Tools

### 1. **Poetry** ⭐ **RECOMMENDED**
**What:** Modern Python dependency management
**Why:** Better than pip + requirements.txt
**Cost:** Free

**Setup:**
```bash
curl -sSL https://install.python-poetry.org | python3 -
poetry init
poetry add pytest pytest-cov
```

### 2. **Pyenv**
**What:** Manage multiple Python versions
**Why:** Test on different Python versions
**Cost:** Free

```bash
# Install
curl https://pyenv.run | bash

# Use
pyenv install 3.11.0
pyenv local 3.11.0
```

### 3. **IPython** ⭐ **RECOMMENDED**
**Already in Codespaces!** Better Python REPL.

### 4. **Jupyter** ⭐ **RECOMMENDED**
**Already in Codespaces!** Interactive notebooks.

### 5. **pydantic**
**What:** Data validation
**Why:** Type-safe configs and APIs
**Cost:** Free

```bash
pip install pydantic
```

---

## Performance & Profiling

### 1. **perf** (Linux)
**What:** System profiler
**Why:** Deep performance analysis
**Cost:** Free

```bash
# Install
sudo apt-get install linux-tools-common

# Profile
perf record --call-graph dwarf ./your_binary
perf report
```

### 2. **Valgrind**
**What:** Memory profiler
**Why:** Find memory leaks
**Cost:** Free

```bash
sudo apt-get install valgrind
valgrind --leak-check=full ./your_binary
```

### 3. **py-spy** ⭐ **RECOMMENDED**
**What:** Python profiler
**Why:** Zero-overhead profiling
**Cost:** Free

```bash
pip install py-spy
py-spy record -o profile.svg -- python your_script.py
```

### 4. **hyperfine**
**What:** Command-line benchmarking
**Why:** Accurate timing
**Cost:** Free

```bash
cargo install hyperfine
hyperfine 'python3 benchmark.py'
```

---

## Quick Setup Checklist

### Must-Have (Priority 0)
- [ ] GitHub Actions CI/CD
- [ ] GitHub Issues with templates
- [ ] GitHub Projects for tracking
- [ ] Dependabot for security
- [ ] Pre-commit hooks
- [ ] Codecov for coverage

### Should-Have (Priority 1)
- [ ] GitHub Pages or Read the Docs
- [ ] Code scanning (CodeQL)
- [ ] cargo-make for task automation
- [ ] cargo-watch for development
- [ ] Poetry for Python deps
- [ ] Excalidraw for diagrams

### Nice-to-Have (Priority 2)
- [ ] GitHub Discussions
- [ ] GitHub Wiki
- [ ] Sentry for error tracking
- [ ] Linear for issue tracking
- [ ] GitPod as backup to Codespaces
- [ ] Notion for internal docs

---

## Action Plan

### Week 1: GitHub Foundation
1. Set up GitHub Actions (1 hour)
2. Create issue templates (30 min)
3. Set up GitHub Projects (30 min)
4. Enable Dependabot (15 min)
5. Configure CodeQL (15 min)

### Week 2: Documentation
1. Choose doc platform (GitHub Pages or Read the Docs)
2. Set up documentation site (2 hours)
3. Migrate key docs (2 hours)
4. Add badges to README

### Week 3: Developer Experience
1. Set up pre-commit hooks (1 hour)
2. Configure cargo-make (1 hour)
3. Set up Codecov (30 min)
4. Add more GitHub Actions workflows

### Week 4: Optimization
1. Set up performance monitoring
2. Create benchmarking dashboard
3. Add profiling tools
4. Document everything

---

## Resources

### GitHub
- https://docs.github.com
- https://github.com/features/actions
- https://github.com/features/codespaces

### Rust
- https://doc.rust-lang.org/cargo/
- https://github.com/rust-lang/rust-analyzer
- https://bheisler.github.io/criterion.rs/

### Python
- https://python-poetry.org/
- https://docs.pytest.org/
- https://pydantic-docs.helpmanual.io/

### DevOps
- https://pre-commit.com/
- https://docs.codecov.com/
- https://sentry.io/welcome/

---

**Next Steps:**
1. Start with Priority 0 tools
2. Set up GitHub Actions first (biggest impact)
3. Add tools incrementally
4. Document your setup

**Questions?** Check the links above or open a GitHub Discussion!
