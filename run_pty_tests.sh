#!/bin/bash
# Run PTY tests with timeout to prevent hangs

echo "Running PTY module tests..."
echo "=============================="
echo

# Run each test with a timeout
timeout 5s cargo test --lib -- --test-threads=1 --nocapture pty::tests::test_create_terminal
timeout 5s cargo test --lib -- --test-threads=1 --nocapture pty::tests::test_create_terminal_with_custom_buffer
timeout 5s cargo test --lib -- --test-threads=1 --nocapture pty::tests::test_invalid_dimensions
timeout 5s cargo test --lib -- --test-threads=1 --nocapture pty::tests::test_spawn_process
timeout 8s cargo test --lib -- --test-threads=1 --nocapture pty::tests::test_spawn_with_timeout
timeout 8s cargo test --lib -- --test-threads=1 --nocapture pty::tests::test_spawn_already_running
timeout 8s cargo test --lib -- --test-threads=1 --nocapture pty::tests::test_read_write
timeout 8s cargo test --lib -- --test-threads=1 --nocapture pty::tests::test_no_process_running_errors

echo
echo "=============================="
echo "Basic tests completed!"
echo
echo "Total tests implemented: 19"
echo "Tests run in this script: 8"
