# GitHub Codespaces - Quick Setup Guide

**⚡ Get started coding in 3 clicks and 5 minutes!**

## What You Get

A fully configured development environment with:
- ✅ Rust 1.70+ with all tools
- ✅ Python 3.11 with all dependencies
- ✅ VS Code with extensions installed
- ✅ QMNF System pre-built and ready
- ✅ All tests passing
- ✅ Zero local setup required

## Step-by-Step Instructions (First Time)

### 1. Go to GitHub Repository

Navigate to: `https://github.com/Skyelabz210/QMNF_System`

### 2. Click the Green "Code" Button

You'll find it near the top right, above the file list.

### 3. Click "Codespaces" Tab

Switch from "Local" to "Codespaces" tab in the dropdown.

### 4. Click "Create codespace on main"

Or select your branch from the dropdown if working on a feature.

### 5. Wait for Setup (3-5 minutes)

You'll see:
```
Setting up your codespace...
Running postCreateCommand...
Building Rust library...
Installing dependencies...
```

### 6. You're Ready!

When you see the welcome message, try these commands:

```bash
# Build the project
qmnf-build

# Run tests
qmnf-test-python
qmnf-test-rust

# Check system
qmnf-check
```

## Using Your Codespace

### Built-in Commands

```bash
qmnf-build         # Build Rust library (release mode)
qmnf-test-rust     # Run Rust tests
qmnf-test-python   # Run Python tests
qmnf-bench         # Run quick benchmark
qmnf-check         # Check for float contamination
qmnf-validate      # Validate boundary protection
```

### File Locations

- **Python code**: `qmnf/` directory
- **Rust code**: `hcvlang/src/` directory
- **Tests**: `tests/python/` and `hcvlang/tests/`
- **Documentation**: `*.md` files in root

### Opening Files

- Press `Ctrl+P` (or `Cmd+P` on Mac) to quick-open files
- Type filename to search
- Press `Ctrl+Shift+F` to search in all files

### Running Tests

**Quick test:**
```bash
python3 -m pytest tests/python/test_suite.py -v
```

**Specific test:**
```bash
python3 -m pytest tests/python/test_suite.py::TestCRTBigInt -v
```

**All tests with coverage:**
```bash
python3 -m pytest tests/ --cov=qmnf --cov-report=html
```

### Debugging

**Set a breakpoint:**
1. Click left of line number (red dot appears)
2. Press `F5` to start debugging
3. Use debug controls at top

**Debug console:**
- View variables
- Execute code
- Inspect state

## Daily Workflow

### Starting Work

1. Go to https://github.com/codespaces
2. Click your codespace name
3. Wait for it to start (~30 seconds)
4. Pull latest changes: `git pull`

### Making Changes

1. Create a branch: `git checkout -b feature-name`
2. Make your changes
3. Test: `qmnf-test-python && qmnf-test-rust`
4. Check: `qmnf-check`
5. Commit: `git add . && git commit -m "description"`
6. Push: `git push -u origin feature-name`

### Ending Work

**Important:** Stop your codespace to save hours!

**Option 1: Auto-stop**
- Settings (gear icon) → Timeout: 30 minutes
- Codespace stops automatically when idle

**Option 2: Manual stop**
1. Click "Codespaces" in bottom-left corner
2. Select "Stop Current Codespace"

**Option 3: From GitHub**
1. Go to https://github.com/codespaces
2. Click "..." next to your codespace
3. Click "Stop codespace"

## Tips & Tricks

### Keyboard Shortcuts

- `Ctrl+`` - Toggle terminal
- `Ctrl+Shift+P` - Command palette
- `Ctrl+P` - Quick open file
- `Ctrl+Shift+F` - Search all files
- `Ctrl+/` - Toggle comment
- `F5` - Start debugging
- `Ctrl+B` - Toggle sidebar

### Extensions Included

- **Rust Analyzer** - Rust IDE features
- **Python** - Python IDE features
- **Pylance** - Fast Python IntelliSense
- **Ruff** - Fast Python linting
- **GitHub Copilot** - AI code assistant (if enabled)

### Performance

**Faster builds:**
- Cargo cache is preserved between sessions
- Build artifacts are cached
- First build: ~5 min, subsequent: ~30 sec

**Machine sizes:**
- 2-core (default) - Good for most work
- 4-core - Faster builds, tests
- 8-core - Maximum performance

Change machine: Click codespace menu → "Change Machine Type"

### Accessing Running Services

If you start a web server:

```bash
# Example: Run a dev server on port 8000
python3 -m http.server 8000
```

Codespaces automatically forwards the port and gives you a URL to access it.

## Troubleshooting

### "Module not found" Error

```bash
# Rebuild Rust library
qmnf-build

# Check Python path
echo $PYTHONPATH

# Try importing
python3 -c "import hcvlang; print('OK')"
```

### Build Errors

```bash
# Clean everything
cd hcvlang
cargo clean
cargo build --release
cd ..
```

### Extension Not Loading

1. Press `F1`
2. Type "Reload Window"
3. Press Enter

### Can't Push to GitHub

```bash
# Check authentication
gh auth status

# Re-authenticate if needed
gh auth login
```

### Codespace Won't Start

1. Go to https://github.com/codespaces
2. Find your codespace
3. Click "..." → Delete
4. Create new codespace

## Cost & Limits

### Free Tier (Personal Accounts)
- **60 core-hours/month** on 2-core machine
- **15 GB storage** included
- Perfect for QMNF development!

### Usage Tips
- **Stop when not using** - Saves hours
- **Delete old codespaces** - Frees storage
- **Use 2-core default** - Sufficient for most work

### Check Usage
1. Go to https://github.com/settings/billing
2. Click "Plans and usage"
3. See Codespaces section

## Documentation

**Inside Codespace, read:**
- `00_START_HERE.md` - Project overview
- `DEVELOPER_QUICK_START.md` - Development guide
- `CLAUDE.md` - Complete development standards
- `.devcontainer/README.md` - Advanced configuration

## Need Help?

**Common Issues:**
- Check `.devcontainer/README.md` for detailed troubleshooting
- Read `CLAUDE.md` for development guidelines
- Review https://docs.github.com/codespaces

**Still Stuck?**
- Open an issue on GitHub
- Check existing issues for solutions
- Ask in discussions

## Advanced Features

### Multiple Codespaces

You can have multiple codespaces for different branches:
- One for main (stable)
- One for feature branch (experimentation)
- One for hotfix (urgent fixes)

### Connecting from VS Code Desktop

1. Install VS Code Desktop
2. Install "GitHub Codespaces" extension
3. Press `F1` → "Codespaces: Connect to Codespace"
4. Select your codespace
5. Work in desktop VS Code instead of browser

### Sharing Your Codespace

Share a running service with team members:
1. Start a service (e.g., web server)
2. Right-click forwarded port in Ports panel
3. Select "Port Visibility: Public"
4. Share the URL

### Dotfiles

Customize your shell environment:
1. Create a dotfiles repo on GitHub
2. Add your `.bashrc`, `.vimrc`, etc.
3. Go to GitHub Codespaces settings
4. Link your dotfiles repo
5. New codespaces will use your settings

## Next Steps

**After your codespace is running:**

1. **Read the getting started guide**
   ```bash
   cat 00_START_HERE.md
   ```

2. **Verify the build**
   ```bash
   qmnf-build
   ```

3. **Run the test suite**
   ```bash
   qmnf-test-python
   ```

4. **Try a benchmark**
   ```bash
   qmnf-bench
   ```

5. **Read the development guide**
   ```bash
   cat CLAUDE.md
   ```

6. **Start coding!**

## Success Checklist

After setup, you should be able to:

- [ ] Open files in VS Code
- [ ] Run `qmnf-build` successfully
- [ ] Run `qmnf-test-python` with passing tests
- [ ] Run `qmnf-test-rust` with passing tests
- [ ] Import `hcvlang` in Python
- [ ] Create and use `QMNFRational` objects
- [ ] Run benchmarks with `qmnf-bench`
- [ ] Commit and push changes

If any fail, see Troubleshooting section above.

---

**🎉 You're all set! Happy coding!**

For detailed information, see `.devcontainer/README.md`
