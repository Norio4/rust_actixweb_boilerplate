#!/bin/bash

echo "Actions Start"
echo "===== cargo check ======"
cargo check
if [ $? -ne 0 ]; then
  echo "FAILED ACTION: cargo check failed"
  exit 1
fi
echo "===== cargo fmt --all -- --check ====="
cargo fmt --all -- --check
if [ $? -ne 0 ]; then
  echo "FAILED ACTION: cargo  fmt --all -- --check failed"
  exit 1
fi
echo "===== cargo clippy ======"
cargo clippy -- -D warnings
if [ $? -ne 0 ]; then
  echo "FAILED ACTION: cargo clippy failed"
  exit 1
fi
echo "===== cargo test ======"
cargo test
if [ $? -ne 0 ]; then
  echo "FAILED ACTION: cargo test failed"
  exit 1
fi
echo "Actions Completed"
