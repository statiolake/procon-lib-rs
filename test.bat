@echo off

echo ::: test for AtCoder (2020)
del Cargo.lock
cargo +1.42.0 test --no-default-features --features "atc-2020"
if %ERRORLEVEL% neq 0 (
    echo test failed for AtCoder 2020
    exit /b
)
