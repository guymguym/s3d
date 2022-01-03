#!/bin/bash
set -e -x -o pipefail

# with --remote it updates the commit pointer from a remote branch name
git submodule update --init --remote --recursive
