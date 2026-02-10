$env:PATH = "$env:USERPROFILE\.cargo\bin;$env:PATH"
Set-Location "c:\Projects\Vir"
cargo test 2>&1
