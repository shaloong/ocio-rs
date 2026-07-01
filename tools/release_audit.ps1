param(
    [switch]$IncludeBundled,
    [switch]$IncludeTopLevelPackage,
    [switch]$Offline
)

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest

$repoRoot = Split-Path -Parent $PSScriptRoot
Set-Location $repoRoot

$script:Failures = @()
$script:Warnings = @()

function Invoke-Check {
    param(
        [Parameter(Mandatory = $true)]
        [string]$Name,
        [Parameter(Mandatory = $true)]
        [string[]]$Arguments,
        [string]$WorkingDirectory = $repoRoot,
        [switch]$AllowKnownTopLevelPackageBlocker
    )

    Write-Host ""
    Write-Host "==> $Name"
    Write-Host "    cargo $($Arguments -join ' ')"

    $stdoutPath = [System.IO.Path]::GetTempFileName()
    $stderrPath = [System.IO.Path]::GetTempFileName()
    try {
        $process = Start-Process `
            -FilePath "cargo" `
            -ArgumentList $Arguments `
            -WorkingDirectory $WorkingDirectory `
            -NoNewWindow `
            -Wait `
            -PassThru `
            -RedirectStandardOutput $stdoutPath `
            -RedirectStandardError $stderrPath
        $exitCode = $process.ExitCode
        $stdout = if (Test-Path $stdoutPath) { Get-Content $stdoutPath -Raw } else { "" }
        $stderr = if (Test-Path $stderrPath) { Get-Content $stderrPath -Raw } else { "" }
        $text = @($stdout, $stderr) -join ""
    }
    finally {
        Remove-Item -LiteralPath $stdoutPath -ErrorAction SilentlyContinue
        Remove-Item -LiteralPath $stderrPath -ErrorAction SilentlyContinue
    }
    $text = $text.TrimEnd()

    if ($text) {
        Write-Host $text
    }

    if ($exitCode -eq 0) {
        Write-Host "    PASS" -ForegroundColor Green
        return
    }

    if ($AllowKnownTopLevelPackageBlocker) {
        $knownBlocker =
            $text -match 'failed to select a version for the requirement `ocio-sys = "\^0\.2\.0"`' -and
            $text -match 'candidate versions found which didn''t match: 0\.1\.1'
        if ($knownBlocker) {
            $script:Warnings += "$Name blocked by registry state: publish ocio-sys 0.2.0 before packaging ocio-rs."
            Write-Host "    WARN: blocked by registry state, not by repository contents" -ForegroundColor Yellow
            return
        }
    }

    $script:Failures += "$Name failed."
    Write-Host "    FAIL" -ForegroundColor Red
}

function Test-OcioSysBundledPayload {
    Write-Host ""
    Write-Host "==> Package payload (ocio-sys bundled source)"
    Write-Host "    cargo package -p ocio-sys --allow-dirty --list"

    $stdoutPath = [System.IO.Path]::GetTempFileName()
    $stderrPath = [System.IO.Path]::GetTempFileName()
    try {
        $process = Start-Process `
            -FilePath "cargo" `
            -ArgumentList @("package", "-p", "ocio-sys", "--allow-dirty", "--list") `
            -NoNewWindow `
            -Wait `
            -PassThru `
            -RedirectStandardOutput $stdoutPath `
            -RedirectStandardError $stderrPath
        $exitCode = $process.ExitCode
        $stdout = if (Test-Path $stdoutPath) { Get-Content $stdoutPath -Raw } else { "" }
        $stderr = if (Test-Path $stderrPath) { Get-Content $stderrPath -Raw } else { "" }
        $text = @($stdout, $stderr) -join ""
    }
    finally {
        Remove-Item -LiteralPath $stdoutPath -ErrorAction SilentlyContinue
        Remove-Item -LiteralPath $stderrPath -ErrorAction SilentlyContinue
    }

    if ($text.Trim()) {
        Write-Host $text.TrimEnd()
    }

    if ($exitCode -ne 0) {
        $script:Failures += "Package payload (ocio-sys bundled source) failed."
        Write-Host "    FAIL" -ForegroundColor Red
        return
    }

    $hasBundledSource = $text -match 'OpenColorIO[/\\]CMakeLists\.txt' -or $text -match 'third_party[/\\]OpenColorIO'
    if (-not $hasBundledSource) {
        $script:Warnings += "Published ocio-sys package does not currently vendor the OpenColorIO source tree."
        Write-Host "    WARN: bundled source tree not present in packaged ocio-sys payload" -ForegroundColor Yellow
        return
    }

    Write-Host "    PASS" -ForegroundColor Green
}

function Get-OcioSysPackageDir {
    $packageRoot = Join-Path $repoRoot "target/package"
    if (-not (Test-Path -LiteralPath $packageRoot)) {
        return $null
    }

    Get-ChildItem -LiteralPath $packageRoot -Directory |
        Where-Object { $_.Name -like "ocio-sys-*" } |
        Sort-Object LastWriteTime -Descending |
        Select-Object -First 1 -ExpandProperty FullName
}

Invoke-Check -Name "Format" -Arguments @("fmt", "--all", "--", "--check")
Invoke-Check -Name "Clippy" -Arguments @("clippy", "--workspace", "--all-targets", "--no-default-features", "--", "-D", "warnings")
Invoke-Check -Name "Tests (stub)" -Arguments @("test", "--workspace", "--no-default-features")
Invoke-Check -Name "Examples (stub)" -Arguments @("test", "--examples", "--no-default-features")
Invoke-Check -Name "Docs (stub)" -Arguments @("doc", "--workspace", "--no-deps", "--no-default-features")
Invoke-Check -Name "Parity" -Arguments @("run", "--bin", "check_parity", "--quiet")

$ocioSysPackageArgs = @("package", "-p", "ocio-sys", "--allow-dirty")
if ($Offline) {
    $ocioSysPackageArgs += "--offline"
}
Invoke-Check -Name "Package ocio-sys" -Arguments $ocioSysPackageArgs
Test-OcioSysBundledPayload

$ocioSysPackageDir = Get-OcioSysPackageDir
if ($ocioSysPackageDir) {
    $ocioSysBundledBuildArgs = @("build", "--features", "bundled")
    if ($Offline) {
        $ocioSysBundledBuildArgs += "--offline"
    }
    Invoke-Check `
        -Name "Packaged bundled build (ocio-sys)" `
        -Arguments $ocioSysBundledBuildArgs `
        -WorkingDirectory $ocioSysPackageDir
} else {
    $script:Failures += "Packaged bundled build (ocio-sys) could not locate extracted package directory."
    Write-Host ""
    Write-Host "==> Packaged bundled build (ocio-sys)"
    Write-Host "    FAIL" -ForegroundColor Red
}

if ($IncludeBundled) {
    Invoke-Check -Name "Tests (bundled)" -Arguments @("test", "--workspace", "--features", "bundled")
}

if ($IncludeTopLevelPackage) {
    $topLevelPackageArgs = @("package", "--allow-dirty")
    if ($Offline) {
        $topLevelPackageArgs += "--offline"
    }
    Invoke-Check `
        -Name "Package ocio-rs" `
        -Arguments $topLevelPackageArgs `
        -AllowKnownTopLevelPackageBlocker
}

Write-Host ""
Write-Host "==> Release audit summary"

if ($script:Warnings.Count -gt 0) {
    Write-Host "Warnings:" -ForegroundColor Yellow
    foreach ($warning in $script:Warnings) {
        Write-Host "  - $warning"
    }
}

if ($script:Failures.Count -gt 0) {
    Write-Host "Failures:" -ForegroundColor Red
    foreach ($failure in $script:Failures) {
        Write-Host "  - $failure"
    }
    exit 1
}

if ($script:Warnings.Count -gt 0) {
    Write-Host "Result: repository checks passed; external publish-order state still pending." -ForegroundColor Yellow
    exit 0
}

Write-Host "Result: all selected release checks passed." -ForegroundColor Green
