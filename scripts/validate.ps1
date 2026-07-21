#!/usr/bin/env pwsh
# Poltergeist quality gate. Run before committing or opening a PR.
# Mirrors the Dark Avian Labs Rust validate contract (AppBase docs/org-standards/validate.md).
$ErrorActionPreference = 'Stop'

$repoRoot = Split-Path -Parent $PSScriptRoot
Push-Location $repoRoot
try {
    Write-Host '==> cargo fmt --all -- --check'
    cargo fmt --all -- --check
    if ($LASTEXITCODE -ne 0) { throw 'cargo fmt check failed' }

    Write-Host '==> cargo clippy --workspace --all-targets --locked -- -D warnings'
    cargo clippy --workspace --all-targets --locked -- -D warnings
    if ($LASTEXITCODE -ne 0) { throw 'cargo clippy failed' }

    Write-Host '==> cargo test --workspace --locked'
    cargo test --workspace --locked
    if ($LASTEXITCODE -ne 0) { throw 'cargo test failed' }

    Write-Host '==> validate passed'
}
finally {
    Pop-Location
}
