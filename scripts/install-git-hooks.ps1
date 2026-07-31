# Optional: strip automated co-author trailers from commit messages
Copy-Item -Force (Join-Path $PSScriptRoot "git-hooks\prepare-commit-msg") (Join-Path (git rev-parse --git-dir) "hooks\prepare-commit-msg")
Write-Host "Installed prepare-commit-msg hook."
