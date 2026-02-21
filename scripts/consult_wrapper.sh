#!/bin/bash
#
# Wrapper script for /consult command
# Thin shim that calls Python orchestrator (all logic in Python)
#

set -euo pipefail

# Check if we're in ~/Projects
PROJECTS_DIR="$HOME/Projects"
CWD="$(pwd)"

if [[ ! "$CWD" == "$PROJECTS_DIR"* ]]; then
    echo "Error: /consult must be run from within ~/Projects/<project>/" >&2
    echo "Current directory: $CWD" >&2
    exit 1
fi

# Pass all arguments to orchestrator (handles everything: project detection, context check, consultant launch)
# Orchestrator will exit with code 2 if context needs to be created
exec python3 "$HOME/Projects/scripts/consult_orchestrator.py" "$@"
