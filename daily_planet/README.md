# Daily Planet - QMNF Documentation & Artifacts Repository

**Purpose**: Organized repository for all QMNF System non-source-code artifacts.

**Created**: December 16, 2025

---

## Quick Navigation

| Folder | Contents | Audience |
|--------|----------|----------|
| [00_NAVIGATION](00_NAVIGATION/) | Entry points, indexes, MOC files | Everyone |
| [01_GUIDES](01_GUIDES/) | Master guides (System, User, Developer, Researcher) | All |
| [02_ARCHITECTURE](02_ARCHITECTURE/) | System design, blueprints, diagrams | Developers, Researchers |
| [03_API_REFERENCE](03_API_REFERENCE/) | Rust, Python, FFI documentation | Developers |
| [04_MATHEMATICAL](04_MATHEMATICAL/) | Proofs, theorems, algorithms | Researchers |
| [05_BENCHMARKS](05_BENCHMARKS/) | Performance data, reports, comparisons | Developers, Users |
| [06_ANALYSIS](06_ANALYSIS/) | Gap analysis, audits, technical debt | Developers |
| [07_PROGRESS](07_PROGRESS/) | Sessions, milestones, work requests | Team |
| [08_TUTORIALS](08_TUTORIALS/) | Learning resources, demos | Users, Developers |
| [09_REFERENCE](09_REFERENCE/) | External papers, standards | Researchers |
| [10_ARCHIVE](10_ARCHIVE/) | Historical/superseded content | Reference |
| [11_MEDIA](11_MEDIA/) | Images, diagrams, screenshots | All |
| [12_EXPORTS](12_EXPORTS/) | PDF, DOCX, presentations | Distribution |

---

## Design Philosophy

### Numbered Prefixes (00-12)
- Ensures consistent sort order across all file managers
- Navigation (00) always first, Archive/Exports (10-12) at end
- Guides (01) are primary output - most frequently accessed

### Separation from Source Code
- QMNF_System contains **only** source code and essential build files
- All documentation, reports, and artifacts live here in daily_planet
- Clear boundary enables focused development and clean repository

### Guide-Centric Organization
The `01_GUIDES/` folder is the primary output destination:
- **SYSTEM_GUIDE**: Complete architecture understanding
- **USER_GUIDE**: Getting started, tutorials, examples
- **DEVELOPER_GUIDE**: API, coding standards, integration
- **RESEARCHER_GUIDE**: Mathematical foundations, proofs

Guides **reference** content from other folders (02-12) rather than duplicate it.

---

## What Stays in QMNF_System

Only these files remain in the main codebase:
- `README.md` - Project overview
- `CLAUDE.md` - AI assistant instructions
- `LICENSE` - Legal terms
- `Cargo.toml`, `pyproject.toml` - Build configuration
- `setup_*.sh` - Setup scripts
- Source directories (`hcvlang/`, `qmnf/`, `tools/`, `tests/`)

---

## Related Locations

- **Source Code**: `/home/acid/Projects/QMNF_System/`
- **Business Materials** (Private): `~/Desktop/QMNF_Business_Private/`
  - Pitch decks, investment memos, IP submissions
  - Kept separate for confidentiality

---

## Getting Started

1. Start at [00_NAVIGATION/INDEX.md](00_NAVIGATION/INDEX.md) for full navigation
2. Choose your guide based on your role in [01_GUIDES/](01_GUIDES/)
3. Explore specific topics in the numbered folders

---

*This repository supports the QMNF System - Quantum-Modular Numerical Framework*
*Integer-only computation with zero floating-point contamination*
