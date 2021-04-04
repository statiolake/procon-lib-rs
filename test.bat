@echo off

echo ::: test for AtCoder (2020)
del Cargo.lock
cargo +1.42.0 test --no-default-features --features "crates-atc-2020,rust-142"
if %ERRORLEVEL% neq 0 (
    echo test failed for AtCoder 2020
    exit /b
)

echo ::: test for AtCoder (2016)
del Cargo.lock
cargo +1.15.0 test --no-default-features --features "rust-115"
if %ERRORLEVEL% neq 0 (
    echo test failed for AtCoder 2016
    exit /b
)
