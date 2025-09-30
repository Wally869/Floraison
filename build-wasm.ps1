# PowerShell build script for WASM module
# Builds the Rust WASM crate and outputs to the UI directory

Write-Host "🔨 Building Floraison WASM module..." -ForegroundColor Cyan

# Navigate to WASM crate directory
Push-Location floraison-wasm

try {
    # Build with wasm-pack for web target
    wasm-pack build `
        --target web `
        --out-dir ../floraison-ui/src/lib/wasm `
        --out-name floraison `
        --no-typescript

    Write-Host "✅ WASM build complete! Output: floraison-ui/src/lib/wasm/" -ForegroundColor Green
    Write-Host ""
    Write-Host "Generated files:" -ForegroundColor Yellow
    Get-ChildItem ../floraison-ui/src/lib/wasm/ | Format-Table Name, Length, LastWriteTime
}
finally {
    # Go back to root
    Pop-Location
}
