# Build Vassnian for WASM and package for web deployment
Write-Host "Building Vassnian for WASM..." -ForegroundColor Cyan

# Build WASM release
cargo build --target wasm32-unknown-unknown --release
if ($LASTEXITCODE -ne 0) {
    Write-Host "WASM build failed!" -ForegroundColor Red
    exit 1
}

# Create output directory
$outDir = "web_build"
if (Test-Path $outDir) {
    Remove-Item $outDir -Recurse -Force
}
New-Item -ItemType Directory -Path $outDir | Out-Null

# Copy WASM binary
Copy-Item "target\wasm32-unknown-unknown\release\vassnian_game.wasm" "$outDir\"

# Copy HTML
Copy-Item "web\index.html" "$outDir\"

Write-Host ""
Write-Host "WASM build complete!" -ForegroundColor Green
Write-Host "Output: $outDir\" -ForegroundColor Yellow
Write-Host ""
Write-Host "To test locally, run a web server in the $outDir directory:" -ForegroundColor Cyan
Write-Host "  python -m http.server 8080 --directory $outDir" -ForegroundColor White
Write-Host "  Then open http://localhost:8080" -ForegroundColor White
Write-Host ""
Write-Host "For itch.io: zip the contents of $outDir and upload." -ForegroundColor Cyan
