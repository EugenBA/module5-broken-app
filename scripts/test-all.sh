#!/usr/bin/env bash

echo "Test ALL"
cargo test --all
echo "Test valgrind"
valgrind --leak-check=full cargo test --tests
rustup default nightly
echo "Test MIRI"
cargo +nightly miri test
echo "Test -Zsanitizer=address"
RUSTFLAGS="-Zsanitizer=address" cargo test --target x86_64-unknown-linux-gnu
echo "Test -Zsanitizer=thread"
TSAN_OPTIONS="ignore_noninstrumented_modules=1"  RUSTFLAGS="-Zsanitizer=thread -Cunsafe-allow-abi-mismatch=sanitizer" cargo test --target x86_64-unknown-linux-gnu
rustup default stable

