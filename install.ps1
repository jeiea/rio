$ErrorActionPreference = 'Stop'

if (-not $env:RIO_INSTALL_PATH) {
    throw 'Set RIO_INSTALL_PATH in mise.local.toml to the installed rio.exe.'
}
$installed = Get-Item -LiteralPath $env:RIO_INSTALL_PATH
$running = Get-Process -Name rio -ErrorAction SilentlyContinue |
    Where-Object Path -eq $installed.FullName
if ($running) {
    throw "Close Rio before replacing $($installed.FullName)."
}

$release = Get-Item -LiteralPath (Join-Path $PSScriptRoot 'target/release/rio.exe')
$backup = "$($installed.FullName).bak"
$staged = Join-Path $installed.DirectoryName ([IO.Path]::GetRandomFileName())
try {
    [IO.File]::Copy($release.FullName, $staged)
    [IO.File]::Replace($staged, $installed.FullName, $backup)
} finally {
    [IO.File]::Delete($staged)
}

Write-Output "Backup: $backup"
$version = (Get-Item -LiteralPath $installed.FullName).VersionInfo.ProductVersion
Write-Output "Installed Rio $version -> $($installed.FullName)"
