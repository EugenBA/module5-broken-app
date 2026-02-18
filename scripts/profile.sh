#!/usr/bin/env bash
set -euo pipefail

# Пример профилирования (Linux, perf). Настройте под свою систему.
cargo build --release
perf record -g -a ./target/release/demo -- sleep 60 || true
#perf report
perf script > perf.script
/home/eugen/RustroverProjects/module5-broken-app/stackcollapse-perf.pl perf.script | /home/eugen/RustroverProjects/module5-broken-app/flamegraph.pl > flame.svg