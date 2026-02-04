# Enable Raptor mini (Preview) - Admin Guide

This file contains recommended steps for administrators to enable the Raptor mini (Preview) AI model for all agent clients used in the QMNF System.

## Overview
The repository has added `ai_agent_models.yml` to recommend `raptor-mini-preview` as the default model for all clients. To make this effective across your organization, you (or the provider admin) must enable the model and ensure clients have permission.

## Provider Steps
1. Sign in to your AI provider admin console.
2. Navigate to the Models or Model Registry section.
3. Find `raptor-mini-preview` in the list of available models.
4. Enable the model for the tenant or organization, if it is marked as "Preview" or "Restricted".
5. Grant access to service accounts or API keys used by:
   - Maintainers
   - CI/automation agents
   - Contributor tools
6. Update any deployment policies or whitelists to allow usage of `raptor-mini-preview`.

## Client Steps (Local)
1. Update your client configuration to point to `ai_agent_models.yml` in the repository or use environment variables to override default model.
2. Optionally set environment variable:

```bash
export AI_DEFAULT_MODEL=raptor-mini-preview
```

3. For automation CI or cloud builds, set the environment variable in the CI platform settings (GitHub Actions, GitLab CI, etc.)

## Example: GitHub Actions (Using environment variable)
```
name: CI
on: [push, pull_request]

jobs:
  build:
    runs-on: ubuntu-latest
    env:
      AI_DEFAULT_MODEL: raptor-mini-preview
    steps:
      - uses: actions/checkout@v4
      - name: Example: show model env
        run: echo "Default AI Model: $AI_DEFAULT_MODEL"
```

## Confirmation
After enabling, confirm by:
1. Running the helper:
```bash
./scripts/show_ai_model_defaults.sh
```
2. Ensuring the provider returns the `raptor-mini-preview` model in a test API call

## Notes
- This document does not enable the model automatically; provider admin action is required.
- If your provider requires additional steps (billing, token provisioning), follow their documented process.
- Keep security in mind: limit model access to intended service accounts and CI systems.

## Contact
If you need help enabling the model in your provider, contact `support@hackfate.us` or your provider's support channel.
