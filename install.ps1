#!/usr/bin/env pwsh

$ErrorActionPreference = 'Stop'

if ($v) {
  $Version = "v${v}"
}
if ($args.Length -eq 1) {
  $Version = $args.Get(0)
}

$DtInstall = $env:DT_INSTALL
$BinDir = if ($DtInstall) {
  "$DtInstall\bin"
} else {
  "$Home\.dt\bin"
}

$DtZip = "$BinDir\dt.zip"
$DtExe = "$BinDir\dt.exe"
$Target = 'windows-2022'

# GitHub requires TLS 1.2
[Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12

$DtUri = if (!$Version) {
  "https://github.com/dtemplate/dt/releases/latest/download/dt-${Target}.zip"
} else {
  "https://github.com/dtemplate/dt/releases/download/${Version}/dt-${Target}.zip"
}

if (!(Test-Path $BinDir)) {
  New-Item $BinDir -ItemType Directory | Out-Null
}

curl -o $DtZip $DtUri

7z a $DtZip $BinDir

Remove-Item $DtZip

$User = [EnvironmentVariableTarget]::User
$Path = [Environment]::GetEnvironmentVariable('Path', $User)
if (!(";$Path;".ToLower() -like "*;$BinDir;*".ToLower())) {
  [Environment]::SetEnvironmentVariable('Path', "$Path;$BinDir", $User)
  $Env:Path += ";$BinDir"
}

Write-Output "Dt was installed successfully to $DtExe"
Write-Output "Run 'dt --help' to get started"
