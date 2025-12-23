# pkmgr Windows Installer
# PowerShell installer script for Windows
# Copyright (c) 2025 CasjaysDev
# License: MIT

$ErrorActionPreference = "Stop"

# Configuration
$BinaryName = "pkmgr"
$GithubRepo = "pkmgr/pkmgr"
$InstallDir = "$env:ProgramFiles\pkmgr"
$UserInstallDir = "$env:LOCALAPPDATA\pkmgr"
$Version = "latest"

# Functions
function Write-Info {
    param([string]$Message)
    Write-Host "ℹ️  $Message" -ForegroundColor Blue
}

function Write-Success {
    param([string]$Message)
    Write-Host "✅ $Message" -ForegroundColor Green
}

function Write-Error {
    param([string]$Message)
    Write-Host "❌ $Message" -ForegroundColor Red
}

function Write-Warn {
    param([string]$Message)
    Write-Host "⚠️  $Message" -ForegroundColor Yellow
}

function Test-Admin {
    $currentUser = [Security.Principal.WindowsIdentity]::GetCurrent()
    $principal = New-Object Security.Principal.WindowsPrincipal($currentUser)
    return $principal.IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)
}

function Get-Architecture {
    if ([Environment]::Is64BitOperatingSystem) {
        return "x86_64"
    } else {
        return "i686"
    }
}

function Download-Binary {
    param(
        [string]$Architecture
    )
    
    $url = "https://github.com/$GithubRepo/releases/$Version/download/$BinaryName-windows-$Architecture.exe"
    $tempFile = "$env:TEMP\$BinaryName.exe"
    
    Write-Info "Downloading 📦 $BinaryName for Windows-$Architecture..."
    
    try {
        Invoke-WebRequest -Uri $url -OutFile $tempFile -UseBasicParsing
        Write-Success "Downloaded successfully"
        return $tempFile
    } catch {
        Write-Error "Failed to download: $_"
        exit 1
    }
}

function Install-System {
    param([string]$BinaryPath)
    
    Write-Info "Installing to $InstallDir..."
    
    try {
        if (-not (Test-Path $InstallDir)) {
            New-Item -ItemType Directory -Path $InstallDir -Force | Out-Null
        }
        
        Copy-Item -Path $BinaryPath -Destination "$InstallDir\$BinaryName.exe" -Force
        
        # Add to PATH if not already there
        $currentPath = [Environment]::GetEnvironmentVariable("Path", "Machine")
        if ($currentPath -notlike "*$InstallDir*") {
            [Environment]::SetEnvironmentVariable(
                "Path",
                "$currentPath;$InstallDir",
                "Machine"
            )
            Write-Info "Added $InstallDir to system PATH"
        }
        
        Write-Success "Installed to $InstallDir\$BinaryName.exe"
    } catch {
        Write-Error "Installation failed: $_"
        exit 1
    }
}

function Install-User {
    param([string]$BinaryPath)
    
    Write-Info "Installing to $UserInstallDir..."
    
    try {
        if (-not (Test-Path $UserInstallDir)) {
            New-Item -ItemType Directory -Path $UserInstallDir -Force | Out-Null
        }
        
        Copy-Item -Path $BinaryPath -Destination "$UserInstallDir\$BinaryName.exe" -Force
        
        # Add to user PATH if not already there
        $currentPath = [Environment]::GetEnvironmentVariable("Path", "User")
        if ($currentPath -notlike "*$UserInstallDir*") {
            [Environment]::SetEnvironmentVariable(
                "Path",
                "$currentPath;$UserInstallDir",
                "User"
            )
            Write-Info "Added $UserInstallDir to user PATH"
        }
        
        Write-Success "Installed to $UserInstallDir\$BinaryName.exe"
        Write-Warn "Please restart your terminal for PATH changes to take effect"
    } catch {
        Write-Error "Installation failed: $_"
        exit 1
    }
}

function Test-Installation {
    # Refresh PATH for current session
    $env:Path = [Environment]::GetEnvironmentVariable("Path", "Machine") + ";" + 
                [Environment]::GetEnvironmentVariable("Path", "User")
    
    try {
        $version = & $BinaryName --version 2>&1
        Write-Success "$BinaryName installed successfully!"
        Write-Info "Version: $version"
        return $true
    } catch {
        Write-Error "Installation verification failed"
        return $false
    }
}

# Main
function Main {
    param([bool]$UserInstall = $false)
    
    Write-Host ""
    Write-Host "📦 pkmgr Windows Installer" -ForegroundColor Cyan
    Write-Host "===========================" -ForegroundColor Cyan
    Write-Host ""
    
    # Detect architecture
    $arch = Get-Architecture
    Write-Info "Detected architecture: $arch"
    
    # Download
    $binaryPath = Download-Binary -Architecture $arch
    
    # Install
    if ($UserInstall) {
        Install-User -BinaryPath $binaryPath
    } else {
        if (Test-Admin) {
            Install-System -BinaryPath $binaryPath
        } else {
            Write-Warn "Not running as Administrator. Installing to user directory..."
            Install-User -BinaryPath $binaryPath
        }
    }
    
    # Clean up
    Remove-Item -Path $binaryPath -Force -ErrorAction SilentlyContinue
    
    # Verify
    if (Test-Installation) {
        Write-Host ""
        Write-Success "Installation complete!"
        Write-Host ""
        Write-Info "Try it out:"
        Write-Host "    $BinaryName --help"
        Write-Host "    $BinaryName search vim"
        Write-Host ""
    } else {
        exit 1
    }
}

# Parse arguments
$UserInstall = $false
foreach ($arg in $args) {
    switch ($arg) {
        "--user" {
            $UserInstall = $true
        }
        "--version" {
            $Version = $args[$args.IndexOf($arg) + 1]
        }
        "--help" {
            Write-Host "Usage: .\windows.ps1 [OPTIONS]"
            Write-Host ""
            Write-Host "Options:"
            Write-Host "  --user           Install to user directory"
            Write-Host "  --version VER    Install specific version (default: latest)"
            Write-Host "  --help           Show this help message"
            exit 0
        }
    }
}

Main -UserInstall $UserInstall
