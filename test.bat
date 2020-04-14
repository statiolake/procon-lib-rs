@echo off

echo ::: test for Rust 2020
del Cargo.lock
cargo +1.42.0 test --no-default-features --features "rust2020"
if %ERRORLEVEL% neq 0 (
    echo test failed for Rust 2020
    exit /b
)

echo ::: test for Rust 2016
del Cargo.lock
cargo +1.15.0 test --no-default-features --features "rust2016"
if %ERRORLEVEL% neq 0 (
    echo test failed for Rust 2016
    exit /b
)
