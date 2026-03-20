#!/bin/bash
#
# Sets up the local development environment.
# Run this once after cloning the repository.

set -e

echo "Setting up git hooks..."
git config core.hooksPath .githooks
echo "Git hooks configured."

echo ""
echo "Done. Your development environment is ready."
