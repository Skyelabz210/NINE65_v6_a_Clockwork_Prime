#!/bin/bash
# Setup script for local model delegation
# Installs Ollama and pulls recommended models

set -e

echo "========================================"
echo "Local Model Delegation Setup"
echo "========================================"
echo ""

# Check if Ollama is installed
if command -v ollama &> /dev/null; then
    echo "✓ Ollama is already installed"
    OLLAMA_VERSION=$(ollama --version | head -1)
    echo "  Version: $OLLAMA_VERSION"
else
    echo "Installing Ollama..."
    curl -fsSL https://ollama.com/install.sh | sh
    echo "✓ Ollama installed successfully"
fi

echo ""
echo "========================================"
echo "Pulling Recommended Models"
echo "========================================"
echo ""
echo "This will download ~20GB of models. Continue? (y/n)"
read -r response

if [[ "$response" != "y" ]]; then
    echo "Skipping model downloads. You can pull them later with:"
    echo "  ollama pull <model-name>"
    exit 0
fi

# Pull models
MODELS=(
    "deepseek-coder:6.7b"      # Fast code generation (5GB)
    "qwen2.5:14b"              # Mathematical reasoning (9GB)
    "gemma2:9b"                # Code analysis (5.5GB)
    "llama3.1:8b"              # General purpose (4.7GB)
)

for model in "${MODELS[@]}"; do
    echo ""
    echo "Pulling $model..."
    ollama pull "$model"
    echo "✓ $model ready"
done

echo ""
echo "========================================"
echo "Setup Complete!"
echo "========================================"
echo ""
echo "Available slash commands in Claude Code:"
echo "  /codex <prompt>         - Code generation"
echo "  /qwen <prompt>          - Mathematical reasoning"
echo "  /gemini-local <prompt>  - Code analysis"
echo "  /delegate <prompt>      - Smart routing"
echo ""
echo "Test it:"
echo "  /codex implement fibonacci in rust"
echo "  /qwen prove the chinese remainder theorem"
echo ""
echo "All models run locally - no API keys needed!"
