#!/usr/bin/env bash
set -euo pipefail

# Пример профилирования (Linux, perf). Настройте под свою систему.
cargo build --release
perf record -g --max-size=50M ./target/release/demo || true
perf report
