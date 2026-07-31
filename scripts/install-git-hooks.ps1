# Optional: keep cursoragent off GitHub Contributors
Copy-Item -Force (Join-Path $PSScriptRoot "git-hooks\prepare-commit-msg") (Join-Path (git rev-parse --git-dir) "hooks\prepare-commit-msg")
Write-Host "Installed prepare-commit-msg hook (strips Cursor co-author trailers)."
