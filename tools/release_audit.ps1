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
        [switch]$AllowKnownTopLevelPackageBlocker
    )

    Write-Host ""
    Write-Host "==> $Name"
    Write-Host "    cargo $($Arguments -join ' ')"

    $output = & cargo @Arguments 2>&1
    $exitCode = $LASTEXITCODE
    $text = ($output | Out-String).TrimEnd()

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

Invoke-Check -Name "Format" -Arguments @("fmt", "--all", "--", "--check")
Invoke-Check -Name "Clippy" -Arguments @("clippy", "--workspace", "--all-targets", "--no-default-features", "--", "-D", "warnings")
Invoke-Check -Name "Tests (stub)" -Arguments @("test", "--workspace", "--no-default-features")
Invoke-Check -Name "Examples (stub)" -Arguments @("test", "--examples", "--no-default-features")
Invoke-Check -Name "Docs (stub)" -Arguments @("doc", "--workspace", "--no-deps", "--no-default-features")

$ocioSysPackageArgs = @("package", "-p", "ocio-sys", "--allow-dirty")
if ($Offline) {
    $ocioSysPackageArgs += "--offline"
}
Invoke-Check -Name "Package ocio-sys" -Arguments $ocioSysPackageArgs

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
