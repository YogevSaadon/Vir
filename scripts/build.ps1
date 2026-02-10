$env:PATH = "$env:USERPROFILE\.cargo\bin;$env:PATH"
Set-Location "c:\Projects\Vir"
cargo build 2>&1
