#!/usr/bin/env bash
set -euo pipefail
mode="${1:-standards}"
case "$mode" in
  standards|compliance)
    bash scripts/ci-doctor.sh
    bash ops/ci/jankurai.sh
    ;;
  *)
    echo "unknown PR policy mode: $mode" >&2
    exit 2
    ;;
esac
