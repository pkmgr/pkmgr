# CasjaysDev Package Manager (pkmgr) - Complete Project Specification

## ⚠️ CRITICAL RULES - READ FIRST ⚠️

**This is a STRICT SPECIFICATION - not guidelines.**

### Build & Test Rules (NON-NEGOTIABLE)

| Rule | Description |
|------|-------------|
| **ALWAYS use Docker** | All builds MUST use Docker containers |
| **ALWAYS use Incus** | All testing MUST use Incus containers (Debian, Ubuntu, Fedora, AlmaLinux) |
| **NEVER run on host** | Never execute binary or cargo directly on host system |
| **Static binary** | Single static binary with zero runtime dependencies |
| **Cross-platform** | Build for 8 platforms: Linux, macOS, Windows, BSD × amd64, arm64 |

### Docker Build Workflow

```bash
# Build in Docker (correct)
docker run --rm -v $(pwd):/build -w /build rust:alpine cargo build --release

# Run tests in Incus (correct)  
incus launch images:debian/12 pkmgr-test
incus exec pkmgr-test -- /path/to/pkmgr install git

# WRONG - Never on host
cargo build                    # ❌
./target/release/pkmgr        # ❌
```

### CI/CD Rules (NON-NEGOTIABLE)

| Rule | Description |
|------|-------------|
| **VERSION from tag** | Strip `v` prefix: `v1.2.3` → `1.2.3` |
| **Docker on EVERY push** | Any branch push triggers Docker image build |
| **Docker tags** | Push → `devel`, `{commit}`; Beta → adds `beta`; Tag → `{version}`, `latest`, `YYMM`, `{commit}` |
| **8 platform builds** | Linux, macOS, Windows, BSD × amd64, arm64 |
| **GitHub/Gitea/Jenkins must match** | Same platforms, same logic |

### Directory Structure Rules (NON-NEGOTIABLE)

| Directory | Contents | Notes |
|-----------|----------|-------|
| `src/` | ALL Rust source code | No exceptions |
| `scripts/` | Production installer scripts | install.sh, linux.sh, bsd.sh, windows.ps1 |
| `tests/` | Test/development scripts | Test scripts, dev helpers |
| `docker/` | Docker files | Dockerfile, compose files |
| `docs/` | Additional documentation | Optional |

### Documentation Rules (NON-NEGOTIABLE)

| File | Purpose | Required |
|------|---------|----------|
| `README.md` | User documentation | YES |
| `CLAUDE.md` | Project specification (this file) | YES |
| `TODO.md` | Task tracking | YES |
| `LICENSE.md` | MIT License | YES |

**No other markdown files allowed in project root.**

### Commit Message Rules

Format: `{emoji} Title (max 64 chars) {emoji}`

| Emoji | Type | Use For |
|-------|------|---------|
| ✨ | feat | New feature |
| 🐛 | fix | Bug fix |
| 📝 | docs | Documentation |
| 🎨 | style | Formatting, no code change |
| ♻️ | refactor | Code refactoring |
| ⚡ | perf | Performance |
| ✅ | test | Tests |
| 🔧 | chore | Config/build |
| 🔒 | security | Security fix |
| 🗑️ | remove | Removing code/files |
| 🚀 | deploy | Deployment related |
| 📦 | deps | Dependency updates |

### Specification Maintenance (NON-NEGOTIABLE)

**CLAUDE.md MUST be kept synchronized with project state.**

| When | Action |
|------|--------|
| **Feature added** | Update feature list, architecture, and relevant sections |
| **Feature removed** | Remove from spec, update architecture |
| **Command changed** | Update command syntax and examples |
| **Config changed** | Update configuration sections |
| **Build process changed** | Update Makefile and CI/CD sections |
| **After major refactor** | Review entire spec for accuracy |

**This specification is the single source of truth for the project.**

### CLI Output Standards (NON-NEGOTIABLE)

**--help and --version MUST follow these exact formats:**

#### --help Output Format

```
pkmgr v1.0.0 - Universal Package Manager

Usage:
  pkmgr <source> <action> [target] [options]
  pkmgr <command> [options]

Core Commands:
  install <package>       Install package from system package manager
  remove <package>        Remove package completely
  update [package|all]    Update packages (all if no target)
  search <query>          Search system package manager
  list [installed|available]  List packages
  info <package>          Show package information
  where <package>         Show package location
  whatis <package>        Show package description
  fix                     Fix broken dependencies

Language Commands:
  <lang> install <version>    Install language version
  <lang> install <package>    Install package for current version
  <lang> use <version>        Switch active version
  <lang> list                 Show installed versions
  <lang> remove <version>     Remove language version
  <lang> current              Show current version

Languages: node, python, go, rust, ruby, php, java, dotnet

Universal Flags:
  --force, -f             Override safety checks
  --quiet, -q             Minimal output
  --verbose, -v           Detailed output
  --yes, -y               Auto-confirm prompts
  --dry-run               Show what would happen
  --explain               Show underlying commands
  --global                System-wide installation
  --user                  User-space installation
  --help, -h              Show this help
  --version               Show version

Examples:
  pkmgr install docker
  pkmgr node install 20.10.0
  pkmgr node use 20
  pkmgr binary install jesseduffield/lazydocker
  pkmgr iso install ubuntu

For complete documentation: https://github.com/pkmgr/pkmgr
```

#### --version Output Format

```
pkmgr v1.0.0 (abc1234) built 2025-12-23
```

Format: `{binary} v{version} ({commit}) built {date}`

### Exit Codes (NON-NEGOTIABLE)

| Code | Meaning |
|------|---------|
| 0 | Success |
| 1 | General error |
| 2 | Misuse of command |
| 3 | Cannot execute |
| 4 | Command not found |
| 5 | Package not found |
| 6 | Already installed |
| 7 | Permission denied |
| 8 | Network error |
| 9 | Disk full |
| 10 | Corrupted download |
| 11 | Failed verification |
| 12 | Dependency conflict |
| 13 | Operation cancelled |
| 14 | Lock timeout |
| 15 | Invalid configuration |
| 126 | Command found but not executable |
| 127 | Command not found |
| 130 | Interrupted (Ctrl+C) |
| 143 | Terminated |

## Implementation Status
- **Current Version**: 1.0.0-alpha
- **Completion**: 98% (ready for testing)
- **Implementation Date**: December 23, 2025
- **Status**: Feature-complete, pending compilation verification

### What's Implemented
- ✅ All 7 package managers (apt, dnf, pacman, homebrew, winget, chocolatey, scoop)
- ✅ All 8 core commands (install, remove, update, search, list, info, where, whatis)
- ✅ Language version management (8-level resolution priority)
- ✅ Binary asset management (GitHub/GitLab)
- ✅ ISO management (30+ distributions)
- ✅ USB bootable media creation
- ✅ Repository management with GPG verification
- ✅ Error recovery system (250+ patterns)
- ✅ Profile management
- ✅ Shell integration
- ✅ Beautiful terminal UI
- ✅ Package name normalization

### Project Structure
```
pkmgr/
├── README.md              # Main documentation
├── CLAUDE.md              # This specification file
├── TODO.md                # Implementation tracking
├── src/                   # All Rust source code (94 files)
│   ├── main.rs           # Entry point
│   ├── commands/         # Command implementations
│   ├── managers/         # Package manager implementations
│   ├── core/             # Core functionality
│   ├── languages/        # Language version management
│   └── ...
├── scripts/               # Production installer scripts
│   ├── install.sh        # Universal installer (Linux/macOS/BSD)
│   ├── linux.sh          # Linux-specific wrapper
│   ├── bsd.sh            # BSD-specific wrapper
│   ├── windows.ps1       # Windows installer
│   └── build/test/debug/clean.sh
├── tests/                 # Test and development scripts
├── docker/                # Docker configuration
└── docs/                  # Additional documentation
```

## Project Identity and Licensing
- **Project Name**: CasjaysDev Package Manager
- **Binary Name**: pkmgr
- **License**: MIT License (must include full MIT license text in LICENSE file)
- **Copyright**: Copyright (c) 2025 CasjaysDev
- **Repository**: https://github.com/pkmgr/pkmgr
- **Version**: 1.0.0
- **Author**: Jason Hempstead
- **Contact**: jason@casjaysdev.pro

## Core Philosophy and Design Principles
- Universal package manager that provides one consistent interface across all package sources
- Smart wrapper around existing tools - never reinvent, always enhance
- Single static Rust binary with zero dependencies
- Beautiful user experience with intuitive, self-explanatory commands
- MIT licensed code only - can shell out to GPL tools but our code must be MIT
- Enterprise-friendly with no vendor lock-in
- Default to system packages, explicit language targeting, binary preference for development tools
- Never break existing packages or system integrity
- No virtual environments - keep it simple for normal users
- Privilege escalation auto-detection using OS native authentication
- Fuzzy search by default, no --fuzzy flag needed
- Minimal output by default, verbose when requested
- Show helpful information initially, then reduce verbosity after user learns
- Project-aware version management using VCS detection
- Safety first - USB operations only on removable devices
- Standard formats output - no proprietary lock-in
- Extremely smart error handling with pattern-based recovery
- No AI/ML - just comprehensive logic patterns
- NEVER execute curl | sh or any piped scripts
- No external scripts - everything compiled into the binary

## Docker Rules (NON-NEGOTIABLE)

### Multi-Stage Dockerfile Requirements

| Rule | Description |
|------|-------------|
| **Multi-stage build** | Builder stage (rust:alpine) + Runtime stage (alpine:latest) |
| **Dockerfile location** | `./docker/Dockerfile` - NEVER in project root |
| **Required packages** | `curl`, `bash` in runtime image |
| **Static binary** | Copy from builder, no Rust runtime needed |
| **LABEL metadata** | org.opencontainers.image.* labels |

### Dockerfile Template

```dockerfile
# Builder stage
FROM rust:alpine AS builder
WORKDIR /build
RUN apk add --no-cache musl-dev
COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl

# Runtime stage
FROM alpine:latest
RUN apk add --no-cache bash curl ca-certificates
COPY --from=builder /build/target/x86_64-unknown-linux-musl/release/pkmgr /usr/local/bin/
LABEL org.opencontainers.image.title="pkmgr"
LABEL org.opencontainers.image.description="Universal Package Manager"
LABEL org.opencontainers.image.authors="Jason Hempstead <jason@casjaysdev.pro>"
ENTRYPOINT ["/usr/local/bin/pkmgr"]
```

### Docker Compose Template

```yaml
version: '3.8'
services:
  pkmgr:
    image: ghcr.io/pkmgr/pkmgr:latest
    container_name: pkmgr
    volumes:
      - ./config:/config
      - ./data:/data
    environment:
      - PKMGR_CONFIG_DIR=/config
      - PKMGR_DATA_DIR=/data
```

## Privilege Escalation Best Practices (NON-NEGOTIABLE)

### Platform-Specific Approaches

| Platform | Method | Behavior |
|----------|--------|----------|
| **Linux** | `sudo -n true` | Test non-interactive, never prompt |
| **macOS** | Native Authorization API | Can show GUI prompt |
| **Windows** | UAC elevation | Can show elevation prompt |
| **BSD** | `sudo -n true` | Same as Linux |

### Linux/BSD Sudoers Configuration

When first run with sudo, automatically create `/etc/sudoers.d/pkmgr`:

```sudoers
# /etc/sudoers.d/pkmgr
# Generated by pkmgr - safe static binary
# Allows passwordless execution of pkmgr only

%sudo ALL=(ALL) NOPASSWD: /usr/local/bin/pkmgr
%wheel ALL=(ALL) NOPASSWD: /usr/local/bin/pkmgr
%admin ALL=(ALL) NOPASSWD: /usr/local/bin/pkmgr

# Alternative for user installations
%sudo ALL=(ALL) NOPASSWD: /home/*/bin/pkmgr
%wheel ALL=(ALL) NOPASSWD: /home/*/bin/pkmgr
%admin ALL=(ALL) NOPASSWD: /home/*/bin/pkmgr
```

**This is safe because pkmgr is a single static binary with no shell execution.**

### Privilege Escalation Logic

```
Linux/BSD:
  1. Attempt: sudo -n true (non-interactive test)
  2. If successful: Can escalate silently (proceed with system install)
  3. If failed: Running in headless/CI mode (proceed with user install)
  4. NEVER prompt for password (could be remote/automated)
  
macOS:
  1. Check if running as admin
  2. If not: Show native GUI prompt for admin password
  3. If user cancels: Proceed with user install
  4. If authorized: Proceed with system install
  
Windows:
  1. Check if running as Administrator
  2. If not: Show UAC elevation prompt
  3. If user cancels: Proceed with user install
  4. If authorized: Proceed with system install
```

### Installation Base Directories

**System Installation (with privileges):**
```
Base: /usr/local/share/pkmgr/
Binary symlinks: /usr/local/bin/
Config: /etc/pkmgr/
```

**User Installation (without privileges):**
```
Base: ~/.local/share/pkmgr/
Binary symlinks: ~/.local/bin/
Config: ~/.config/pkmgr/
```

**Critical Rule: Never Override OS Package Managers**
- Never install pip packages to system Python
- Never install npm packages to system Node
- Never modify /usr/bin or /bin
- Always use isolated environments in base directory

## Makefile Structure (NON-NEGOTIABLE)

### Required Targets

| Target | Purpose | Must Have |
|--------|---------|-----------|
| `build` | Build static binaries for all platforms + host | YES |
| `release` | Create GitHub release with all binaries | YES |
| `docker` | Build and push container image | YES |
| `test` | Run comprehensive tests across environments | YES |
| `clean` | Clean up build artifacts | YES |
| `help` | Show available targets (default) | YES |

### Makefile Rules

| Rule | Description |
|------|-------------|
| **All builds use Docker** | Never run cargo directly on host |
| **Static binaries** | MUSL targets for Linux, static linking |
| **8 platforms minimum** | Linux, macOS, Windows, BSD × amd64, arm64 |
| **Binary naming** | `{project}-{os}-{arch}` (windows adds `.exe`) |
| **Version from tag** | Use git tags, strip `v` prefix |

### Example Build Flow

```makefile
build: build-all build-host

build-all:
    @echo "Building static binaries for all platforms..."
    @docker build --target builder -t pkmgr-builder .
    @for target in $(TARGETS); do \
        docker run --rm -v $(PWD):/app -w /app pkmgr-builder \
            cargo build --release --target $$target; \
    done

build-host:
    @echo "Building host binary..."
    @docker run --rm -v $(PWD):/app -w /app pkmgr-builder \
        cargo build --release --target x86_64-unknown-linux-musl
```

## CI/CD Workflows (NON-NEGOTIABLE)

### Required Workflows

| Workflow | Trigger | Purpose |
|----------|---------|---------|
| **release.yml** | Tag push (`v*.*.*`) | Build all platforms, create GitHub release |
| **beta.yml** | Tag push (`v*.*.*-beta*`) | Build beta release |
| **docker.yml** | Any push | Build and push Docker image |
| **test.yml** | PR, push to main | Run tests |

### Docker Image Tagging Strategy

| Trigger | Tags Applied |
|---------|--------------|
| **Any push** | `devel`, `{short-sha}` |
| **Beta tag** | `beta`, `{version}-beta`, `{short-sha}` |
| **Release tag** | `{version}`, `latest`, `{YYMM}`, `{short-sha}` |

### Version Extraction Rules

```bash
# Strip 'v' prefix from tags
VERSION=${GITHUB_REF#refs/tags/v}  # v1.2.3 → 1.2.3
```

### Build Environment Variables

```yaml
env:
  CARGO_TERM_COLOR: always
  RUSTFLAGS: '-C target-feature=+crt-static'
```

### Release Asset Naming

```
pkmgr-linux-x86_64
pkmgr-linux-aarch64
pkmgr-darwin-x86_64
pkmgr-darwin-aarch64
pkmgr-windows-x86_64.exe
pkmgr-freebsd-x86_64
pkmgr-freebsd-aarch64
```

## Testing Strategy (NON-NEGOTIABLE)

### Docker Testing (Development)

```bash
# Build in Docker
docker run --rm -v $(pwd):/build -w /build rust:alpine \
    sh -c "apk add --no-cache musl-dev && cargo build --release"

# Binary smoke test
docker run --rm -v $(pwd):/app alpine:latest /app/target/release/pkmgr --help
```

### Incus Testing (Integration)

```bash
# Create test containers for each distro
incus launch images:debian/12 pkmgr-test-debian
incus launch images:ubuntu/22.04 pkmgr-test-ubuntu
incus launch images:fedora/39 pkmgr-test-fedora
incus launch images:almalinux/9 pkmgr-test-alma

# Copy binary to each
incus file push target/release/pkmgr pkmgr-test-debian/tmp/

# Run tests
incus exec pkmgr-test-debian -- /tmp/pkmgr install git
incus exec pkmgr-test-ubuntu -- /tmp/pkmgr search vim
incus exec pkmgr-test-fedora -- /tmp/pkmgr list installed
incus exec pkmgr-test-alma -- /tmp/pkmgr update all --dry-run

# Cleanup
incus delete pkmgr-test-debian pkmgr-test-ubuntu pkmgr-test-fedora pkmgr-test-alma --force
```

### CI/CD Workflow Templates

#### GitHub Actions: Release Workflow

```yaml
# .github/workflows/release.yml
name: Release

on:
  push:
    tags:
      - 'v*.*.*'

env:
  CARGO_TERM_COLOR: always
  RUSTFLAGS: '-C target-feature=+crt-static'

jobs:
  build:
    name: Build ${{ matrix.target }}
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        include:
          - os: ubuntu-latest
            target: x86_64-unknown-linux-musl
            name: pkmgr-linux-amd64
          - os: ubuntu-latest
            target: aarch64-unknown-linux-musl
            name: pkmgr-linux-arm64
          - os: macos-latest
            target: x86_64-apple-darwin
            name: pkmgr-darwin-amd64
          - os: macos-latest
            target: aarch64-apple-darwin
            name: pkmgr-darwin-arm64
          - os: windows-latest
            target: x86_64-pc-windows-gnu
            name: pkmgr-windows-amd64.exe
          - os: ubuntu-latest
            target: x86_64-unknown-freebsd
            name: pkmgr-freebsd-amd64

    steps:
      - uses: actions/checkout@v4
      
      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          target: ${{ matrix.target }}
          override: true
          
      - name: Install cross-compilation tools
        if: matrix.os == 'ubuntu-latest'
        run: |
          sudo apt-get update
          sudo apt-get install -y musl-tools
          
      - name: Build
        run: cargo build --release --target ${{ matrix.target }}
        
      - name: Rename binary
        run: |
          cd target/${{ matrix.target }}/release
          mv pkmgr* ${{ matrix.name }} || true
          
      - name: Upload artifact
        uses: actions/upload-artifact@v4
        with:
          name: ${{ matrix.name }}
          path: target/${{ matrix.target }}/release/${{ matrix.name }}

  release:
    name: Create Release
    needs: build
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Download all artifacts
        uses: actions/download-artifact@v4
        with:
          path: ./binaries
          
      - name: Generate checksums
        run: |
          cd binaries
          sha256sum * > SHA256SUMS.txt
          sha512sum * > SHA512SUMS.txt
          
      - name: Extract version
        id: version
        run: echo "VERSION=${GITHUB_REF#refs/tags/v}" >> $GITHUB_OUTPUT
        
      - name: Create Release
        uses: softprops/action-gh-release@v1
        with:
          name: pkmgr v${{ steps.version.outputs.VERSION }}
          draft: false
          prerelease: false
          files: |
            binaries/**/*
            binaries/SHA256SUMS.txt
            binaries/SHA512SUMS.txt
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
```

#### GitHub Actions: Docker Workflow

```yaml
# .github/workflows/docker.yml
name: Docker Build

on:
  push:
    branches: ['**']
    tags: ['v*.*.*']

env:
  REGISTRY: ghcr.io
  IMAGE_NAME: pkmgr/pkmgr

jobs:
  docker:
    runs-on: ubuntu-latest
    permissions:
      contents: read
      packages: write
      
    steps:
      - uses: actions/checkout@v4
      
      - name: Log in to Container Registry
        uses: docker/login-action@v3
        with:
          registry: ${{ env.REGISTRY }}
          username: ${{ github.actor }}
          password: ${{ secrets.GITHUB_TOKEN }}
          
      - name: Extract metadata
        id: meta
        run: |
          VERSION=$(echo ${{ github.ref }} | sed 's|refs/tags/v||' | sed 's|refs/heads/||')
          COMMIT=${GITHUB_SHA::7}
          DATE=$(date -u +"%Y-%m-%d")
          
          TAGS=""
          if [[ "${{ github.ref }}" == refs/tags/v* ]]; then
            # Release tag: version, latest, YYMM, commit
            TAGS="${{ env.REGISTRY }}/${{ env.IMAGE_NAME }}:${VERSION}"
            TAGS="${TAGS},${{ env.REGISTRY }}/${{ env.IMAGE_NAME }}:latest"
            TAGS="${TAGS},${{ env.REGISTRY }}/${{ env.IMAGE_NAME }}:$(date +%y%m)"
            TAGS="${TAGS},${{ env.REGISTRY }}/${{ env.IMAGE_NAME }}:${COMMIT}"
          elif [[ "${{ github.ref }}" == refs/tags/v*-beta* ]]; then
            # Beta tag: beta, version-beta, commit
            TAGS="${{ env.REGISTRY }}/${{ env.IMAGE_NAME }}:beta"
            TAGS="${TAGS},${{ env.REGISTRY }}/${{ env.IMAGE_NAME }}:${VERSION}"
            TAGS="${TAGS},${{ env.REGISTRY }}/${{ env.IMAGE_NAME }}:${COMMIT}"
          else
            # Branch push: devel, commit
            TAGS="${{ env.REGISTRY }}/${{ env.IMAGE_NAME }}:devel"
            TAGS="${TAGS},${{ env.REGISTRY }}/${{ env.IMAGE_NAME }}:${COMMIT}"
          fi
          
          echo "TAGS=${TAGS}" >> $GITHUB_OUTPUT
          echo "VERSION=${VERSION}" >> $GITHUB_OUTPUT
          echo "COMMIT=${COMMIT}" >> $GITHUB_OUTPUT
          echo "DATE=${DATE}" >> $GITHUB_OUTPUT
          
      - name: Build and push
        uses: docker/build-push-action@v5
        with:
          context: .
          file: docker/Dockerfile
          push: true
          tags: ${{ steps.meta.outputs.TAGS }}
          build-args: |
            VERSION=${{ steps.meta.outputs.VERSION }}
            COMMIT=${{ steps.meta.outputs.COMMIT }}
            BUILD_DATE=${{ steps.meta.outputs.DATE }}
          labels: |
            org.opencontainers.image.title=pkmgr
            org.opencontainers.image.description=Universal Package Manager
            org.opencontainers.image.version=${{ steps.meta.outputs.VERSION }}
            org.opencontainers.image.revision=${{ steps.meta.outputs.COMMIT }}
            org.opencontainers.image.created=${{ steps.meta.outputs.DATE }}
```

#### Gitea Actions (Same Structure)

Gitea workflows are identical to GitHub Actions - copy the above to `.gitea/workflows/`

#### Jenkins Pipeline

```groovy
// Jenkinsfile
pipeline {
    agent any
    
    environment {
        CARGO_TERM_COLOR = 'always'
        RUSTFLAGS = '-C target-feature=+crt-static'
        VERSION = sh(script: "git describe --tags --always | sed 's/^v//'", returnStdout: true).trim()
        COMMIT = sh(script: "git rev-parse --short HEAD", returnStdout: true).trim()
        DATE = sh(script: "date -u +%Y-%m-%d", returnStdout: true).trim()
    }
    
    stages {
        stage('Build') {
            matrix {
                axes {
                    axis {
                        name 'TARGET'
                        values 'x86_64-unknown-linux-musl', 'aarch64-unknown-linux-musl', 
                               'x86_64-apple-darwin', 'aarch64-apple-darwin',
                               'x86_64-pc-windows-gnu', 'x86_64-unknown-freebsd'
                    }
                }
                stages {
                    stage('Build Target') {
                        steps {
                            sh """
                                docker run --rm -v \$(pwd):/build -w /build rust:alpine \
                                    sh -c "apk add --no-cache musl-dev && \
                                           rustup target add ${TARGET} && \
                                           cargo build --release --target ${TARGET}"
                            """
                        }
                    }
                }
            }
        }
        
        stage('Test') {
            steps {
                sh 'make test'
            }
        }
        
        stage('Release') {
            when {
                tag pattern: "v\\d+\\.\\d+\\.\\d+", comparator: "REGEXP"
            }
            steps {
                sh """
                    mkdir -p releases/${VERSION}
                    cp target/*/release/pkmgr* releases/${VERSION}/
                    cd releases/${VERSION}
                    sha256sum * > SHA256SUMS.txt
                """
                archiveArtifacts artifacts: 'releases/**/*', fingerprint: true
            }
        }
        
        stage('Docker') {
            steps {
                sh """
                    docker build -t pkmgr/pkmgr:${VERSION} -f docker/Dockerfile .
                    docker tag pkmgr/pkmgr:${VERSION} pkmgr/pkmgr:latest
                    docker push pkmgr/pkmgr:${VERSION}
                    docker push pkmgr/pkmgr:latest
                """
            }
        }
    }
    
    post {
        always {
            cleanWs()
        }
    }
}
```

incus exec pkmgr-test-ubuntu -- /tmp/pkmgr search vim
incus exec pkmgr-test-fedora -- /tmp/pkmgr update
incus exec pkmgr-test-alma -- /tmp/pkmgr list

# Clean up
incus delete -f pkmgr-test-debian pkmgr-test-ubuntu pkmgr-test-fedora pkmgr-test-alma
```

### Test Matrix

| Distro | Version | Package Manager | Must Test |
|--------|---------|-----------------|-----------|
| Debian | 12 | apt | YES |
| Ubuntu | 22.04 | apt | YES |
| Fedora | 39 | dnf | YES |
| AlmaLinux | 9 | dnf | YES |
| Arch | latest | pacman | Optional |
| Alpine | latest | apk | Optional |

## Architecture and Implementation

### Single Static Rust Binary
- All logic compiled into one binary
- No external scripts or dependencies
- Binary acts as dispatcher based on how it's called
- Direct syscalls only - no shell execution
- Input validation in Rust - memory safe
- Cannot be hijacked via PATH manipulation

### Symlink Strategy for Language Commands
```
~/.local/bin/
├── pkmgr           (the actual binary)
├── python → pkmgr  (symlink)
├── python3 → pkmgr (symlink)
├── node → pkmgr    (symlink)
├── npm → pkmgr     (symlink)
├── pip → pkmgr     (symlink)
├── pip3 → pkmgr    (symlink)
├── ruby → pkmgr    (symlink)
├── gem → pkmgr     (symlink)
├── go → pkmgr      (symlink)
├── cargo → pkmgr   (symlink)
├── rustc → pkmgr   (symlink)
├── java → pkmgr    (symlink)
├── javac → pkmgr   (symlink)
└── dotnet → pkmgr  (symlink)
```

### Binary Detection Logic
When called as a language command (e.g., 'python'), pkmgr:
1. Detects how it was called (argv[0])
2. Checks for project version files in current directory
3. Searches parent directories up to VCS root
4. Checks project manifest files for version requirements
5. Uses user's default version from ~/.local/share/pkmgr/languages/*/current
6. Falls back to system version if available
7. If missing and TTY detected: Prompts to install
8. If missing and no TTY: Falls back to system or error
9. Sets appropriate environment variables
10. Executes actual binary with execve()

### Version Resolution Priority
```
1. Command line override (--version flag)
2. Current directory version file (.python-version, .node-version, etc.)
3. Parent directory search (up to VCS root or 5 levels)
4. Project manifest file (package.json, pyproject.toml, Gemfile, etc.)
5. User default (~/.local/share/pkmgr/languages/*/current)
6. System default (/usr/local/share/pkmgr/languages/*/current)
7. System installed version (/usr/bin/python, etc.)
8. Prompt to install if TTY, error if non-interactive
```

### Privilege Escalation Logic
```
Linux:
  - First attempt: sudo -n true (non-interactive test)
  - If successful: Can escalate privileges silently
  - If failed: Running in headless/CI/CD mode, proceed with user install
  - Never prompt for password on Linux (could be remote/automated)
  
macOS:
  - Use native Authorization Services API
  - Can show GUI prompt for admin password
  - If user cancels: Proceed with user install
  - If authorized: Proceed with system install
  
Windows:
  - Check if running as Administrator
  - If not: Use UAC elevation prompt
  - If user cancels: Proceed with user install
  - If authorized: Proceed with system install
  
BSD:
  - Same as Linux (sudo -n true test)
  - Assume headless/remote possibility
```

### Automatic Sudo Configuration
When first run with sudo, pkmgr creates `/etc/sudoers.d/pkmgr`:
```sudoers
# /etc/sudoers.d/pkmgr
# Generated by pkmgr - safe static binary
# Allows passwordless execution of pkmgr only

%sudo ALL=(ALL) NOPASSWD: /usr/local/bin/pkmgr
%wheel ALL=(ALL) NOPASSWD: /usr/local/bin/pkmgr
%admin ALL=(ALL) NOPASSWD: /usr/local/bin/pkmgr

# Alternative for user installations
%sudo ALL=(ALL) NOPASSWD: /home/*/bin/pkmgr
%wheel ALL=(ALL) NOPASSWD: /home/*/bin/pkmgr
%admin ALL=(ALL) NOPASSWD: /home/*/bin/pkmgr
```

This is safe because pkmgr is a single static binary with no shell execution.

### Installation Base Directories

**System Installation (with privileges):**
```
Base Directory: /usr/local/share/pkmgr/
Structure:
  /usr/local/share/pkmgr/
    ├── languages/          # Language versions
    │   ├── node/
    │   │   ├── 20.10.0/
    │   │   ├── 18.19.0/
    │   │   └── current → 20.10.0
    │   ├── python/
    │   │   ├── 3.11.7/
    │   │   ├── 3.10.13/
    │   │   └── current → 3.11.7
    │   └── ...
    ├── binaries/          # Downloaded binaries
    │   └── installed.toml # Tracking file
    ├── cache/             # System cache
    └── data/              # System data

Binaries Symlinked to: /usr/local/bin/
Config Location: /etc/pkmgr/
```

**User Installation (without privileges):**
```
Base Directory: ~/.local/share/pkmgr/
Structure: (same as system but in user home)
Binaries Symlinked to: ~/.local/bin/
Config Location: ~/.config/pkmgr/
```

**Critical Rule: Never Override OS Package Managers**
- Never install pip packages to system Python
- Never install npm packages to system Node
- Never modify /usr/bin or /bin
- Never modify system package manager paths
- Always use isolated environments in our base directory
- Language packages go to: `<base>/languages/<lang>/<version>/lib/`

### Build Tools Requirements
When installing packages that require compilation, automatically install build essentials:

**Linux Debian/Ubuntu:**
- build-essential, gcc, g++, make, cmake, pkg-config, libssl-dev, libffi-dev, python3-dev

**Linux Fedora/RHEL:**
- @development-tools, gcc, gcc-c++, make, cmake, openssl-devel, libffi-devel, python3-devel

**Linux Arch:**
- base-devel, gcc, make, cmake, openssl, libffi, python

**macOS:**
- Xcode Command Line Tools (via xcode-select --install)

**Windows:**
- Visual Studio Build Tools or MinGW-w64

## Command Structure and Syntax

All commands follow the pattern: `pkmgr <source> <action> [target] [options]`

### Universal Flags (available on all commands)
- `--force`: Override safety checks and confirmations
- `--quiet, -q`: Minimal output only
- `--verbose, -v`: Detailed operation output
- `--yes, -y`: Auto-confirm all prompts
- `--dry-run`: Show what would happen without executing
- `--explain`: Show underlying native commands that would be executed
- `--profile <name>`: Use specific configuration profile
- `--arch <architecture>`: Specify target architecture
- `--version <version>`: Specify target version
- `--global`: Force system-wide installation
- `--user`: Force user-space installation

### Core Package Management Commands
- `pkmgr install <package>`: Install via system package manager (default behavior)
- `pkmgr remove <package>`: Complete purge removal with cleanup
- `pkmgr update [package|all]`: Update packages (all if no target specified)
- `pkmgr list [installed|available]`: List packages
- `pkmgr search <query>`: Search system package manager only
- `pkmgr info <package>`: Show detailed package information
- `pkmgr where <package>`: Show installation location/path
- `pkmgr whatis <package>`: Show package description
- `pkmgr fix`: Fix broken dependencies and installations

### Command Aliases
```
Built-in Aliases:
pkmgr i → install
pkmgr r → remove  
pkmgr u → update
pkmgr s → search
pkmgr ls → list
pkmgr rm → remove
pkmgr up → update
pkmgr dl → install (download)
```

### Language Version Management Commands
For each language (node, python, go, rust, ruby, php, java, dotnet):
- `pkmgr <lang> install <version>`: Install specific language version
- `pkmgr <lang> install <package>`: Install package for current version
- `pkmgr <lang> use <version>`: Switch active version
- `pkmgr <lang> list`: Show installed versions
- `pkmgr <lang> list --available`: Show available versions for installation
- `pkmgr <lang> remove <version>`: Remove language version
- `pkmgr <lang> current`: Show current active version
- `pkmgr <lang> info <package>`: Show package information
- `pkmgr <lang> search <query>`: Search language-specific packages (PyPI, npm, etc.)

### Binary Management Commands
- `pkmgr binary search <query>`: Search for binary releases
- `pkmgr binary install <user/repo>[@version]`: Install from GitHub/GitLab
- `pkmgr binary install <url>`: Install from direct URL
- `pkmgr binary list`: Show installed binaries
- `pkmgr binary update [name]`: Update binaries
- `pkmgr binary remove <name>`: Remove binary
- `pkmgr binary info <user/repo>`: Show repository information

### ISO Management Commands
- `pkmgr iso list`: Show all supported distributions
- `pkmgr iso list <distro>`: Show available versions for specific distribution
- `pkmgr iso list --downloaded`: Show locally downloaded ISOs
- `pkmgr iso install <distro> [version]`: Download ISO (current version if no version specified)
- `pkmgr iso remove <iso-file>`: Delete downloaded ISO file
- `pkmgr iso info <distro>`: Show distribution information
- `pkmgr iso verify [iso-file]`: Verify ISO checksums and signatures
- `pkmgr iso clean`: Remove old/duplicate ISO files

### USB Management Commands
- `pkmgr usb`: Launch interactive USB wizard
- `pkmgr usb erase <device>`: Completely wipe USB device
- `pkmgr usb write <iso-file> <device>`: Write single ISO to USB (dd-style)
- `pkmgr usb boot <device>`: Create or manage multi-boot USB
- `pkmgr usb boot add <iso|distro>`: Add ISO to multi-boot USB
- `pkmgr usb boot remove <iso|distro>`: Remove ISO from multi-boot USB
- `pkmgr usb boot list`: Show ISOs on multi-boot USB
- `pkmgr usb boot clean`: Remove old/duplicate ISOs from USB

### Profile Management Commands
- `pkmgr profile list`: Show all profiles
- `pkmgr profile list <name>`: Show specific profile details
- `pkmgr profile create <name>`: Create new profile
- `pkmgr profile create <name> --copy-current`: Create profile from current state
- `pkmgr profile use <name>`: Switch to profile
- `pkmgr profile remove <name>`: Delete profile
- `pkmgr profile edit <name>`: Edit profile in $EDITOR
- `pkmgr profile diff <name1> <name2>`: Compare two profiles
- `pkmgr profile export <name>`: Export profile to file
- `pkmgr profile import <file>`: Import profile from file

### Configuration Management Commands
- `pkmgr config list`: Show all configuration settings
- `pkmgr config get <key>`: Get specific configuration value
- `pkmgr config set <key> <value>`: Set configuration value
- `pkmgr config remove <key>`: Remove configuration setting
- `pkmgr config reset`: Reset to default configuration
- `pkmgr config edit`: Edit configuration in $EDITOR

### Repository Management Commands
- `pkmgr repos list`: Show all configured repositories
- `pkmgr repos list <repo>`: Show specific repository details
- `pkmgr repos add <repo>`: Add repository (auto-detects type and handles GPG keys)
- `pkmgr repos remove <repo>`: Remove repository
- `pkmgr repos update`: Refresh all repository metadata
- `pkmgr repos info <repo>`: Show repository information

### Cache Management Commands
- `pkmgr cache list`: Show cache contents and usage
- `pkmgr cache clean`: Clean all caches
- `pkmgr cache clean <source>`: Clean specific cache
- `pkmgr cache info`: Show cache usage and locations
- `pkmgr cache refresh`: Force refresh all cached data

### System Health and Diagnostics Commands
- `pkmgr doctor`: Quick system health check
- `pkmgr doctor --full`: Comprehensive system health check across all sources
- `pkmgr doctor --packages`: Package management health only
- `pkmgr doctor --usb`: USB device health check
- `pkmgr doctor --security`: Security status check
- `pkmgr doctor --fix`: Auto-fix issues where possible

### Bootstrap and Sync Commands
- `pkmgr bootstrap`: Interactive setup wizard for new systems
- `pkmgr bootstrap install <file|url>`: Install packages from list file or URL
- `pkmgr bootstrap export`: Export current system configuration to files
- `pkmgr bootstrap apply <profile>`: Apply complete profile configuration
- `pkmgr sync push`: Push configuration to git repository
- `pkmgr sync pull`: Pull configuration from git repository
- `pkmgr sync init <repo-url>`: Initialize configuration sync with repository

### Update Checking Commands
- `pkmgr check`: Interactive check for updates with notification and update offer
- `pkmgr check script`: Output only the number of available updates for scripting

### Built-in Updater Commands
- `pkmgr update-self`: Check for pkmgr updates (default: check)
- `pkmgr update-self check`: Check for updates without installing (no privileges required)
- `pkmgr update-self yes`: Download and install update with restart
- `pkmgr update-self branch stable`: Set update branch to stable (default)
- `pkmgr update-self branch beta`: Set update branch to beta pre-releases
- `pkmgr update-self branch daily`: Set update branch to daily builds

**Update Branches:**

| Branch | Release Type | Tag Pattern | Example | Description |
|--------|--------------|-------------|---------|-------------|
| `stable` (default) | Release | `v*`, `*.*.*` | `v1.0.0` | Official releases |
| `beta` | Pre-release | `*-beta` | `202512051430-beta` | Beta testing releases |
| `daily` | Pre-release | `YYYYMMDDHHMMSS` | `20251205143022` | Daily development builds |

**Exit Codes:**
- 0: Successful update or no update available
- 1: Error during update

**HTTP 404 from GitHub API means no updates available (already current).**

**Update Process:**
1. Fetch latest version from GitHub releases
2. Compare with current version
3. Download binary for current OS/arch
4. Create backup of current binary (.bak)
5. Replace binary with new version
6. Set permissions (Unix: 755)
7. Notify user to restart pkmgr

**Branch Configuration:**
Stored in `~/.config/pkmgr/update.toml`:
```toml
branch = "stable"
```

### Shell Integration Commands
- `eval $(pkmgr shell load)`: Auto-detect shell and load integration
- `eval $(pkmgr shell load <shell>)`: Load integration for specific shell
- `pkmgr shell completions <shell>`: Generate tab completions for shell
- `pkmgr shell add`: Add ~/.local/bin to current session PATH
- `pkmgr shell remove`: Remove ~/.local/bin from current session PATH
- `pkmgr shell env`: Show shell integration status

## Beautiful UI Specifications

### Emoji and Icon Standards
```
Success Operations:
  ✅ Package installed successfully
  ✨ New version available
  🎉 All systems updated
  🚀 Binary deployed
  ⚡ Cache cleared
  🔄 Repository refreshed
  ✓  Verification passed
  🎯 Target achieved
  💾 Configuration saved
  🔐 Security check passed

Progress Indicators:
  ⏳ Initializing...
  🔍 Searching repositories...
  📦 Downloading packages...
  🔧 Installing dependencies...
  🏗️  Building from source...
  📝 Writing configuration...
  🔄 Updating system...
  🧹 Cleaning up...
  ⚙️  Configuring settings...
  🔗 Creating symlinks...

Warning States:
  ⚠️  Warning: Non-critical issue
  🔶 Caution: Requires attention
  ⏰ Timeout warning
  💾 Low disk space
  🔋 Resource limitation
  🌐 Network issue (recoverable)

Error States:
  ❌ Operation failed
  🚫 Access denied
  💔 Broken dependency
  🔥 Critical error
  ⛔ Operation blocked
  📵 Connection failed

Information:
  ℹ️  Information
  💡 Tip/Hint
  📌 Note
  📊 Statistics
  🔍 Details available
  📚 Documentation reference
  🎯 Current target
  🌍 System-wide
  👤 User-specific
  📁 Project-specific

Package/Language Types:
  🐍 Python
  📦 Node.js/npm
  💎 Ruby
  🦀 Rust
  ☕ Java
  🔷 .NET
  🐹 Go
  🐘 PHP
  🍺 Homebrew
  🐧 System package
  🐳 Container/Docker
  ⚙️  Binary

Operations:
  ➕ Adding
  ➖ Removing
  🔄 Updating
  ♻️  Reinstalling
  🗑️  Purging
  📥 Downloading
  📤 Uploading
  📋 Copying
  ✂️  Moving
  🔀 Switching versions

USB/ISO Operations:
  💿 ISO file
  💾 USB device
  🖥️  Operating System
  🛡️  Security distribution
  🛠️  Utility/Tool
  🔥 Writing to device
  ⏏️  Ejecting safely
  🗂️  Multi-boot menu
  ✨ Formatting device
  ✅ Verification complete
```

### Beautiful Progress Bars
```
Download Progress:
┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
┃ 📦 Downloading: nodejs-20.10.0-linux-x64.tar.gz  ┃
┃ ████████████████████░░░░░░░░░░░░░  45%          ┃
┃ 📊 450 MB / 1.0 GB | ⚡ 5.2 MB/s | ⏱️  1m 45s     ┃
┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛

Multi-package Progress:
╔════════════════════════════════════════════════════╗
║ 📦 Installing Packages (3/10)                      ║
╟────────────────────────────────────────────────────╢
║ ✅ typescript         4.9.5    installed           ║
║ ✅ @types/node       20.10.0  installed           ║
║ ⏳ eslint            8.56.0   installing...       ║
║   ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓░░░░░░░░  72%                 ║
║ ⏸️  prettier          3.1.0    pending             ║
║ ⏸️  webpack           5.89.0   pending             ║
╟────────────────────────────────────────────────────╢
║ Total: 30% ████████░░░░░░░░░░░░░ ETA: 2m 15s      ║
╚════════════════════════════════════════════════════╝

USB Write Progress:
╔═══════════════════════════════════════════════════════╗
║ 💾 Writing ISO to USB Device                         ║
║ 📀 ubuntu-22.04.3-desktop-amd64.iso → /dev/sdb      ║
╟───────────────────────────────────────────────────────╢
║ ▰▰▰▰▰▰▰▰▰▰▰▰▰▰▰▰▰▱▱▱▱▱▱▱▱▱▱▱  67%                  ║
║ 📊 2.8 GB / 4.2 GB | ⚡ 48 MB/s | ⏱️  35s remaining  ║
╟───────────────────────────────────────────────────────╢
║ 🔥 Buffer: 92% | 🌡️ Device: OK | ✓ Verified: 2.8 GB ║
╚═══════════════════════════════════════════════════════╝

Spinner Animations (when indeterminate):
  🌍 Checking repositories   ⣾⣽⣻⢿⡿⣟⣯⣷
  🔄 Refreshing cache       ◐◓◑◒
  🔍 Searching packages     ⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏
  📡 Connecting            ∙∙●∙∙ → ∙●●●∙ → ●●●●●
```

### Time Display Formats
```
ETA Display Rules:
  < 1 minute:     "45s"
  < 1 hour:       "5m 23s"  
  < 24 hours:     "2h 15m"
  > 24 hours:     "1d 6h"
  
Speed Display:
  < 1 KB/s:       "512 B/s"
  < 1 MB/s:       "768 KB/s"
  < 1 GB/s:       "45.2 MB/s"
  > 1 GB/s:       "1.3 GB/s"

Time Elapsed:
  < 1 minute:     "just now"
  < 1 hour:       "5 minutes ago"
  < 24 hours:     "3 hours ago"
  < 7 days:       "2 days ago"
  < 30 days:      "2 weeks ago"
  > 30 days:      "2024-01-15"
```

### Interactive Menus
```
Version Selection Menu:
╔════════════════════════════════════════════╗
║ 🐍 Select Python Version                   ║
╟────────────────────────────────────────────╢
║ → 3.12.1  (latest)         ✨ NEW          ║
║   3.11.7  (LTS)           🏷️ STABLE       ║
║   3.10.13                 ✅ Installed     ║
║   3.9.18  (security only) ⚠️  EOL Soon    ║
║   3.8.18  (security only) 🔒 Security     ║
╟────────────────────────────────────────────╢
║ 📝 Current: 3.10.13 | 🎯 Project: 3.11+   ║
║ Use ↑↓ to navigate, Enter to select       ║
╚════════════════════════════════════════════╝
```

### Status Dashboard
```
pkmgr doctor output:
╔═══════════════════════════════════════════════════════╗
║                   🏥 System Health                    ║
╟───────────────────────────────────────────────────────╢
║ 🖥️  System      Ubuntu 22.04.3 LTS (x86_64)          ║
║ 📦 Packages    ✅ 247 installed | ✨ 12 updates      ║
║ 🗄️  Storage     ✅ 45.2 GB free (72%)                ║
║ 🧠 Memory      ✅ 8.4 GB available                   ║
║ 🌐 Network     ✅ Connected (45ms latency)           ║
║ 🔐 GPG Keys    ⚠️  2 expiring soon                   ║
║ 💾 Cache       ✅ 1.2 GB used (24%)                  ║
║ 🔄 Repos       ✅ All 7 repositories OK              ║
╟───────────────────────────────────────────────────────╢
║ 📊 Language Versions:                                 ║
║   🐍 Python    3.11.7  ✅ (3.12.1 available)        ║
║   📦 Node      20.10.0 ✅ (20.11.0 available)       ║
║   🦀 Rust      1.75.0  ✅ (latest)                  ║
║   💎 Ruby      3.2.2   ✅ (3.3.0 available)         ║
╟───────────────────────────────────────────────────────╢
║ 💡 Run 'pkmgr update all' to install updates         ║
╚═══════════════════════════════════════════════════════╝
```

## Complete Default Specifications

### Network and Download Defaults
```
Connection timeout: 30 seconds
Read timeout: 300 seconds (5 minutes for large files)
Retry attempts: 3
Retry delay: 5 seconds (exponential backoff: 5s, 10s, 20s)
Parallel downloads: 4 concurrent
Max redirects: 10
User agent: "pkmgr/1.0.0 (OS/Version)"
Accept encodings: gzip, deflate, br
Mirror selection: Fastest (ping test), fallback to geographic nearest
Bandwidth limit: None by default (--limit-rate flag to set)
Proxy detection: Auto-detect from environment (HTTP_PROXY, HTTPS_PROXY)
Resume downloads: Always attempt if server supports
Chunk size: 8192 bytes for streaming downloads
SSL/TLS: Verify certificates by default (--insecure to disable)
```

### File Size and Storage Defaults
```
Cache directory size limit: 5 GB
Cache cleanup: When exceeds 4 GB (keep 20% free)
Cache expiry: 30 days for package metadata, 90 days for downloads
ISO storage warning: When < 10 GB free space
ISO download space check: Require 2x ISO size free
USB minimum size: 4-8 GB for single ISO
USB minimum size: At least 16 GB for multi-boot (prefer 64+, support for 128GB+)
USB maximum ISOs: 50 per device
Temporary directory: System temp (/tmp or %TEMP%)
Temp file cleanup: On exit and every 24 hours for orphans
Log file size: 10 MB per file (no compression, raw text)
Log rotation: None (overwrite when size exceeded)
Binary install size check: Warn if binary > 500 MB
```

### Terminal Detection and Display
```
Terminal Capabilities Detection:
  Unicode support: Test with echo -e "\u2713"
  256 colors: Check TERM env and tput colors
  True color: Check COLORTERM=truecolor
  Width: From terminal size, min 80, max 200
  Height: From terminal size, min 24
  
Fallback Modes:
  No Unicode: Use ASCII (-, +, *, [OK], [ERROR])
  No color: Monochrome with text indicators
  Narrow term: Compress progress bars
  Non-TTY: Simple line-based output
  CI/CD mode: Minimal output with ::group:: markers
  JSON mode: Structured JSON output (--json flag)

Color output: Auto-detect TTY (force with --color=always/never/auto)
Progress bar style: [████████░░░░░░░░░░░░] 45% | 450 MB/1 GB | 5.2 MB/s | ETA: 1m 45s
Progress bar width: Terminal width - 40 chars (min 40, max 120)
Update frequency: Every 100ms for progress, 1s for speeds
Spinner style: ⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏ (Unicode) or -\|/ (ASCII fallback)
Confirmation prompt: "[y/N]" (default No for dangerous ops)
Verbose output: Disabled (enable with -v, -vv, -vvv for levels)
Quiet output: Show errors only
Table format: Aligned columns with | separators
List pagination: 25 items (--all to show everything)
Time format: Relative (5 min ago) under 24h, absolute (2024-01-15) after
Size format: Human readable (1.5 GB) by default, --bytes for exact
```

### Confirmation Messages
```
Destructive operations require: "Type 'yes' to confirm: "
USB erase: "This will PERMANENTLY ERASE all data on %s (%s - %s GB)\nType 'YES' in capitals to proceed: "
System package removal: Automatic (no prompt)
Multiple package install: Automatic (no prompt)
Repository add: Automatic when needed for package
Profile switch: Automatic
Auto-install build tools: Automatic (no prompt)
Version not found: "Python %s required by project but not installed.\nInstall now? [Y/n]: "
```

### Performance Defaults
```
Max concurrent operations: 4
Max memory usage: 1 GB (configurable)
CPU cores for operations: All available - 1 (leave 1 for system)
Background operations: Disabled by default (--background to enable)
Nice level for background: 10 (lower priority)
IO nice class: Idle (lowest priority for background)
Database vacuum: Every 100 operations or weekly
Index rebuild: When query time > 1 second
Package database sync: Daily for active repos, weekly for inactive
Binary delta updates: Enabled if saves > 50% bandwidth
Compression level: 6 (balanced speed/size)
Decompression memory limit: 512 MB
Archive extraction threads: 4 or CPU count, whichever is less
Memory Buffer Sizes:
  File read: 8 KB chunks
  Network download: 64 KB chunks
  USB write: 4 MB chunks
  Decompression: 1 MB chunks
Memory Limits:
  Max in-memory file: 100 MB
  Max cache entries: 10,000
  Max search results: 1,000
  Max dependency depth: 100
```

### Security Defaults
```
GPG key auto-fetch: Yes from official keyservers
GPG keyservers: 
  1. hkps://keys.openpgp.org (primary)
  2. hkps://keyserver.ubuntu.com (fallback)
  3. hkps://pgp.mit.edu (fallback)
Key refresh interval: 30 days
Expired key action: Auto-refresh then retry
Missing key action: Auto-fetch (no prompt with --yes)
Checksum verification: Required for ISOs, optional for packages
Checksum algorithms: SHA256 preferred, SHA512/SHA1/MD5 fallback
Signature verification: Required if .sig/.asc file exists
Certificate validation: Required (--insecure to bypass)
Permission elevation timeout: 5 minutes (sudo cache)
Sandbox downloads: Yes (download to temp, verify, then move)
URL validation: Block local IPs (127.*, 192.168.*, 10.*, etc) unless --allow-local
Binary permissions: 755 for executables, 644 for data
Config file permissions: 600 (sensitive configs)
```

### Error Handling Defaults
```
Network error retries: 3 attempts
Package conflict resolution: Automatic with smart logic
Dependency failure: Rollback entire transaction
Disk full: Cleanup cache and retry once
Permission denied: Attempt privilege escalation once
Corrupted download: Delete and retry twice
Invalid signature: Skip with warning (unless --strict)
Missing dependency: Auto-install
Version conflict: Use newest unless pinned
Lock file timeout: 30 seconds
Stale lock file: Remove if older than 1 hour
Rollback on error: Yes for system changes, No for user changes
Error log detail: Full backtrace in verbose mode
Recovery mode: Automatic for minor issues
Orphan cleanup: Automatic after failed operation
```

### Update Strategy Defaults
```
Update cache: Refresh package lists (timeout: 30 seconds per repo)
GPG key refresh: Keys expiring within 30 days
Held packages: Skip automatically
Security updates: Priority 1, highlight with 🔒
Major version updates: Auto-update (no prompt unless known breaking)
Minor version updates: Auto-update
Patch version updates: Auto-update
Breaking change detection: Major version number change (1.x → 2.x)
Update display: Security first, then major, then regular
```

### Cleanup Policies
```
Automatic Cleanup:
  Package cache: After 30 days
  Download cache: After 90 days
  Temp files: On exit
  Orphaned locks: After 1 hour
  Old logs: Overwrite at 10 MB
  Failed downloads: After 24 hours
  
Manual Cleanup Commands:
  pkmgr cache clean: All caches
  pkmgr cache clean --downloads: Just downloads
  pkmgr cache clean --packages: Just packages
  pkmgr cache clean --logs: Just logs
  pkmgr cache clean --all --force: Everything immediately
```

### Exit Codes
```
0: Success
1: General error
2: Misuse of command
3: Cannot execute
4: Command not found
5: Package not found
6: Already installed
7: Permission denied
8: Network error
9: Disk full
10: Corrupted download
11: Failed verification
12: Dependency conflict
13: Operation cancelled
14: Lock timeout
15: Invalid configuration
126: Command found but not executable
127: Command not found
130: Interrupted (Ctrl+C)
143: Terminated
```

## Windows Package Manager Support

### Detection and Priority
```
Detection Order:
- Check for winget first (native to Windows 10/11)
- Check for chocolatey second (most packages)
- Check for scoop third (developer focused)

Package Search Priority:
- If multiple managers installed: Search all, show results grouped by manager
- Install preference: winget > chocolatey > scoop
- User can override with: pkmgr choco install <package> or pkmgr scoop install <package>
```

### Automatic Package Manager Installation
```
Winget Installation:
- Check: winget --version
- If missing: Download App Installer from Microsoft Store or GitHub
- Never use scripts, always use official installer

Chocolatey Installation:
- Check: choco --version
- If missing: Download official installer from chocolatey.org
- Download ChocolateyInstall.zip, extract, run in admin PowerShell
- Never pipe from web

Scoop Installation:
- Check: scoop --version
- If missing: Download installer from scoop.sh
- Review script, then execute
- Requires PowerShell 5+ and .NET Framework 4.5+

No prompts - automatically install when needed:
📦 Installing Chocolatey package manager...
  ⏳ Downloading official installer...
  ⏳ Verifying signature...
  ⏳ Installing (requires admin)...
  ✅ Chocolatey installed successfully!
```

## Package Name Normalization

### Complete Mapping Table
```
Universal Name → Package Manager Specific Names

python → 
  apt: python3, python3.11, python3.12
  dnf: python3, python39, python311
  pacman: python
  brew: python@3.11, python@3.12
  winget: Python.Python.3.11, Python.Python.3.12
  choco: python, python3, python311

node/nodejs →
  apt: nodejs
  dnf: nodejs
  pacman: nodejs
  brew: node
  winget: OpenJS.NodeJS
  choco: nodejs

docker →
  apt: docker-ce (with repo), docker.io (deprecated - never install)
  dnf: docker-ce (with repo), podman (alternative)
  pacman: docker
  brew: docker
  winget: Docker.DockerDesktop
  choco: docker-desktop

git →
  apt: git
  dnf: git
  pacman: git
  brew: git
  winget: Git.Git
  choco: git

vscode/code →
  apt: code (with repo)
  dnf: code (with repo)
  pacman: visual-studio-code-bin (AUR)
  brew: visual-studio-code
  winget: Microsoft.VisualStudioCode
  choco: vscode

chrome →
  apt: google-chrome-stable (with repo)
  dnf: google-chrome-stable (with repo)
  pacman: google-chrome (AUR)
  brew: google-chrome
  winget: Google.Chrome
  choco: googlechrome

gcc →
  apt: gcc, build-essential
  dnf: gcc, gcc-c++, make
  pacman: gcc, base-devel
  brew: gcc
  winget: N/A (use MinGW or MSVC)
  choco: mingw, msys2

mysql →
  apt: mysql-server, mysql-client
  dnf: mysql-server, mysql
  pacman: mariadb (MySQL replaced)
  brew: mysql
  winget: Oracle.MySQL
  choco: mysql

postgresql →
  apt: postgresql, postgresql-client
  dnf: postgresql-server, postgresql
  pacman: postgresql
  brew: postgresql@16
  winget: PostgreSQL.PostgreSQL
  choco: postgresql

redis →
  apt: redis-server, redis-tools
  dnf: redis
  pacman: redis
  brew: redis
  winget: Redis.Redis
  choco: redis-64

nginx →
  apt: nginx
  dnf: nginx
  pacman: nginx
  brew: nginx
  winget: N/A
  choco: nginx

apache →
  apt: apache2
  dnf: httpd
  pacman: apache
  brew: httpd
  winget: ApacheFriends.Xampp
  choco: apache-httpd
```

### Resolution Strategy
```
1. Check exact match first
2. Check normalized name mapping
3. Check common variations (python/python3, nodejs/node)
4. Fuzzy search if no match (Levenshtein distance ≤2)
5. Suggest alternatives if similar found
```

## Dependency Conflict Resolution

### Smart Resolution Strategy

**Single Version Policy:**
```
What We DON'T Do:
- ❌ Install multiple library versions
- ❌ Create dependency hell
- ❌ Use LD_LIBRARY_PATH hacks
- ❌ Install conflicting packages side-by-side

What We DO Instead:
- ✅ Find the ONE version that works for everything
- ✅ Use compatibility packages when available
- ✅ Rebuild packages to use current versions
- ✅ Find alternatives that don't conflict
```

### Resolution Priority
```
1. Check if package can be rebuilt against current library
2. Check if official compat package exists (libssl1.1-compat)
3. Check if updated version available
4. Find alternative package that provides same functionality
5. Use containerized version (Flatpak/AppImage) as last resort
6. Fail safely with clear explanation
```

### What We Never Do
```
❌ Never manually compile and install system libraries
❌ Never use LD_LIBRARY_PATH hacks
❌ Never modify system library paths
❌ Never install libraries to /usr/local/lib manually
❌ Never override system libraries with symlinks
```

## Rollback Mechanism

### Transaction Structure
```
Transaction ID: 2024-01-15-103045-a7b9c2
Location: ~/.local/share/pkmgr/transactions/current.toml

[transaction]
id = "2024-01-15-103045-a7b9c2"
timestamp = "2024-01-15T10:30:45Z"
operation = "install"
status = "in_progress"

[packages]
installed = ["docker-ce-24.0.7", "docker-cli-24.0.7"]
removed = []
upgraded = []

[files]
created = ["/usr/local/bin/docker", "/etc/docker/daemon.json"]
modified = ["/etc/group", "/etc/systemd/system/"]
removed = []

[repositories]
added = ["docker-official"]
removed = []

[config_backup]
"/etc/docker/daemon.json" = "~/.local/share/pkmgr/backups/daemon.json.bak"
```

### Rollback Process
```
1. Mark transaction as "rolling_back"
2. Stop all current operations
3. Restore configuration files from backup
4. Remove newly installed packages
5. Reinstall removed packages
6. Restore modified files
7. Remove added repositories
8. Clean temporary files
9. Mark transaction as "rolled_back"
10. Show rollback summary

Keep last 10 transactions, auto-cleanup older
```

### Rollback Triggers
```
- Installation failure (non-zero exit)
- Download corruption
- Dependency break detected
- Disk space exhausted
- User cancellation (SIGINT/SIGTERM)
- Post-install script failure
- Verification failure
```

## Binary Asset Selection

### Selection Priority
```
1. Static binaries (portable, no dependencies)
   - Patterns: *-static*, *-standalone*, *-musl*, *-full*
   
2. AppImage (portable Linux apps)
   - Patterns: *.AppImage, *.appimage
   
3. Platform-specific single binary
   - Patterns: {name}-{os}-{arch}, {name}_{os}_{arch}
   
4. Generic binary name
   - Patterns: {name} with no extension

5. Compressed binaries
   - Patterns: *.tar.gz, *.zip, *.tar.xz, *.7z
```

### Platform Detection
```
OS Patterns (case-insensitive):
- Linux: linux, ubuntu, debian, fedora, rhel, alpine
- macOS: darwin, macos, osx, mac, apple
- Windows: windows, win64, win32, msvc, mingw

Architecture Patterns:
- x86_64: x86_64, amd64, x64, 64, 64bit
- ARM64: aarch64, arm64, armv8
- Universal: universal, all, any

Special Cases:
- musl: For Alpine Linux (prefer over glibc for static builds)
- gnu: For standard Linux (glibc)
- msvc: For Windows (Visual C++)
- mingw: For Windows (MinGW)
```

### GUI Detection
```
Linux:
- Check DISPLAY env var
- Check WAYLAND_DISPLAY env var
- Check XDG_SESSION_TYPE
- Check /tmp/.X11-unix/

macOS:
- Always assume GUI capable
- Check for Terminal.app in process tree

Windows:
- Always assume GUI capable
- Check for conhost.exe or cmd.exe parent

Patterns:
- GUI: *-gui*, *-desktop*, *-qt*, *-gtk*
- CLI: *-cli*, *-terminal*, *-console*, *-headless*
```

### Architecture Validation
```
For generic binaries without arch in name:
1. Download to temp location
2. Run 'file' command to detect architecture
3. Check if matches system architecture
4. If match → Install
5. If no match → Delete and try next asset
6. If no valid asset → Error

Architecture Detection Methods:
- Linux/macOS: file command output parsing
- Check ELF headers for Linux
- Check Mach-O headers for macOS  
- Check PE headers for Windows
- Fallback: Try to execute with --version
```

## ISO Management

### Supported Distributions
```
Linux Distributions:
- Ubuntu: LTS versions (22.04, 20.04, 18.04) and latest non-LTS
- Debian: Current stable, oldstable, oldoldstable
- Fedora: Current and 2 previous versions
- Arch Linux: Always latest rolling release
- Manjaro: Multiple desktop environments (XFCE, KDE, GNOME)
- openSUSE: Tumbleweed (rolling) and Leap (stable)
- CentOS: Stream and legacy versions
- Rocky Linux: Current versions (9.x, 8.x)
- AlmaLinux: Current versions (9.x, 8.x)
- Alpine Linux: Current and previous versions
- Void Linux: Current with multiple desktop environments
- Gentoo: Current stage3 and LiveGUI
- NixOS: Current releases

Security/Penetration Testing:
- Kali Linux: Current and recent versions, multiple flavors
- Parrot Security: Current versions, security and home editions
- BlackArch Linux: Current versions
- Tails: Current version for privacy/anonymity

Server/Enterprise:
- Proxmox VE: Current versions
- TrueNAS/FreeNAS: Current versions
- pfSense: Current community edition
- opnSense: Current versions
- VyOS: Current versions

BSD Systems:
- FreeBSD: Current and previous major versions
- OpenBSD: Current and previous versions
- NetBSD: Current versions

Utility/Rescue Tools:
- GParted Live: Current version
- Clonezilla Live: Current version
- SystemRescue: Current version
- MemTest86+: Current version
- Hiren's Boot CD: Current version
- Ultimate Boot CD: Current version
```

### ISO Storage and Organization
```
Directory: ~/Downloads/ISOs/ (configurable via PKMGR_ISO_DIR)
Structure:
OS/
  Linux/
    ubuntu-22.04-desktop-amd64.iso
    debian-12.2.0-amd64-netinst.iso
    fedora-39-workstation-x86_64.iso
    arch-2023.11.01-x86_64.iso
  Windows/
    Windows-11-22H2-x64.iso
    WindowsServer-2022-x64.iso
  Mac/
    macOS-Sonoma-14.1.iso
Security/
  kali-linux-2023.4-live-amd64.iso
  parrot-security-5.3-amd64.iso
Tools/
  gparted-live-1.5.0-1-amd64.iso
  clonezilla-live-3.1.0-22-amd64.iso
Server/
  proxmox-ve-8.0-2.iso
  freenas-13.0-U5.3.iso
```

### ISO Verification
```
Verification Steps:
  1. Download checksums file (SHA256SUMS, SHA512SUMS)
  2. Download signature file (.sig, .asc)
  3. Verify signature of checksums file
  4. Verify ISO against checksums
  5. Show verification status with icons
  
Missing Checksums:
  - Warn user
  - Offer to continue anyway
  - Log unverified downloads
  
Failed Verification:
  - Delete corrupted file
  - Retry download (max 2 times)
  - Fail with clear error
```

### Commercial OS Support
```
macOS: Show legal warning about Apple Software License Agreement
Windows: Show legal warning about Microsoft EULA
Legal warnings shown on first use, less verbose after acknowledgment
Include 'pkmgr legal' command for legal information
```

## USB Management

### USB Wizard Flow
```
Step 1: Device Selection
╔════════════════════════════════════════════════════════╗
║ 💾 USB Device Setup Wizard                             ║
╟────────────────────────────────────────────────────────╢
║ 🔍 Detected USB devices:                               ║
║                                                         ║
║ 1. /dev/sdb - SanDisk Ultra 32GB                      ║
║    └─ 29.3 GB available, currently FAT32              ║
║                                                         ║
║ 2. /dev/sdc - Kingston DataTraveler 64GB              ║
║    └─ 59.6 GB available, currently exFAT              ║
║                                                         ║
║ R. 🔄 Refresh device list                              ║
║ Q. ❌ Quit wizard                                      ║
╟────────────────────────────────────────────────────────╢
║ Select device [1-2]: _                                 ║
╚════════════════════════════════════════════════════════╝

Step 2: Operation Type
Step 3: ISO Selection or Multi-boot Configuration
Step 4: Partition Layout (if multi-boot)
Step 5: Initial ISOs Selection
Step 6: Confirmation with clear warnings
Step 7: Progress with detailed status
Step 8: Completion with next steps
```

### Safety Requirements
```
USB operations MUST only work on removable devices
Detect removable status via platform-specific methods:
  - Linux: Check /sys/block/*/removable flag
  - macOS: Use diskutil to check "removable media" property
  - Windows: Check device properties for removable flag
  
Multiple safety layers before any write operation:
  1. Device is removable (via system APIs)
  2. Device is USB connected
  3. Device is not mounted as system partition
  4. Device is not in /etc/fstab or equivalent
  5. Device size is reasonable for USB (not multi-TB internal drives)
  6. User confirmation with device details
  
Block operations on internal drives, NVMe drives, system disks
Show clear error messages when attempting to use non-removable devices
```

### Multi-boot USB Structure
```
Default: Single partition (simpler, more compatible)
  - Entire device: FAT32 or exFAT (based on size)
  - FAT32: For devices ≤32 GB (maximum compatibility)
  - exFAT: For devices >32 GB (supports large files)

Directory Structure:
/boot/
  ├── grub/
  │   ├── grub.cfg          # Auto-generated boot menu
  │   └── themes/           # Boot themes
  └── syslinux/
      └── syslinux.cfg      # Alternative boot loader
/isos/
  ├── OS/
  │   ├── Linux/           # Linux ISOs here
  │   ├── Windows/         # Windows ISOs here
  │   └── Mac/             # macOS ISOs here
  ├── Security/            # Security ISOs here
  ├── Tools/               # Utility ISOs here
  └── Server/              # Server ISOs here
/persistence/             # For persistent storage (optional)
```

### Boot Menu Structure
```
Main Menu
├── Operating Systems →
│   ├── Linux →
│   │   ├── Debian →
│   │   │   ├── Debian 12.2.0 (Bookworm)
│   │   │   └── Debian 11.8.0 (Bullseye)
│   │   ├── Ubuntu →
│   │   │   ├── Ubuntu 22.04.3 LTS
│   │   │   └── Ubuntu 23.10
│   │   ├── RHEL Family →
│   │   │   ├── Rocky Linux 9.3
│   │   │   ├── AlmaLinux 9.3
│   │   │   └── CentOS Stream 9
│   │   └── Arch Based →
│   │       ├── Arch Linux 2023.11.01
│   │       └── Manjaro 23.1
│   ├── Windows →
│   └── BSD →
├── Security Tools →
├── System Tools →
└── Builtin Tools →
    ├── Memory Test (builtin)
    ├── Hardware Detection (builtin)
    ├── Network Boot (PXE)
    └── GRUB Command Line
```

### ISO Boot Entry Templates
```
Ubuntu/Debian:
menuentry "Ubuntu 22.04 LTS" {
    set isofile="/isos/OS/Linux/Ubuntu/ubuntu-22.04.3-desktop-amd64.iso"
    loopback loop $isofile
    linux (loop)/casper/vmlinuz boot=casper iso-scan/filename=$isofile quiet splash
    initrd (loop)/casper/initrd
}

Fedora/RHEL:
menuentry "Fedora 39" {
    set isofile="/isos/OS/Linux/Fedora/fedora-39-workstation-x86_64.iso"
    loopback loop $isofile
    linux (loop)/isolinux/vmlinuz inst.stage2=hd:LABEL=Fedora iso-scan/filename=$isofile quiet
    initrd (loop)/isolinux/initrd.img
}

Arch Linux:
menuentry "Arch Linux" {
    set isofile="/isos/OS/Linux/Arch/arch-2023.11.01-x86_64.iso"
    loopback loop $isofile
    linux (loop)/arch/boot/x86_64/vmlinuz-linux archisodevice=/dev/loop0 img_dev=/dev/disk/by-label/MULTIBOOT img_loop=$isofile
    initrd (loop)/arch/boot/x86_64/initramfs-linux.img
}
```

## Repository Management

### Smart Repository Detection
```
Detection Methods (in priority order):
1. GPG key fingerprint matching (most reliable - can't be faked)
2. Package availability check (functional test)
3. Repository metadata (Origin/Label in Release file)
4. URL pattern matching (including mirrors)
5. Domain name (least reliable due to mirrors)

Require 2+ matches for confidence
```

### Mirror Handling
```
Common mirror patterns:
China:
  - mirrors.aliyun.com/*
  - mirrors.tuna.tsinghua.edu.cn/*
  - mirrors.ustc.edu.cn/*
  - mirrors.cloud.tencent.com/*

Europe:
  - mirror.eu/*
  - ftp.{country}.debian.org/*
  - {country}.archive.ubuntu.com/*

CDN:
  - *.cloudfront.net/*
  - *.fastly.net/*
  - *.azureedge.net/*

Corporate mirrors:
  - Private IP ranges (192.168.*, 10.*, etc.)
  - .local domains
  - Custom ports
  - Re-signed with corporate keys
```

### Repository Trust and Key Management
```
Trust Levels:
  Official: OS vendor repos
  Verified: Known vendors (Docker, Microsoft, etc.)
  Community: PPAs, AUR, COPR
  Corporate: Internal mirrors with different keys
  Unknown: User-added

Key Rotation Handling:
  1. Detect signature failure with known repo
  2. Check key servers for new key from same identity
  3. Verify key transition signature (signed by old key)
  4. Check DNS TXT records for key info
  5. Update internal key database

Corporate Re-signing:
  1. Detect different key but identical packages
  2. Mark as "custom-signed mirror"
  3. Trust if explicitly configured
  4. Don't replace official repo entry
```

### Repository Mappings and Auto-Configuration

#### Docker
```
Aliases: docker, docker.io, docker-ce, docker-engine
Packages: docker-ce, docker-ce-cli, containerd.io, docker-buildx-plugin, docker-compose-plugin
Detection: docker.com URLs, GPG key 9DC858229FC7DD38854AE2D88D81803C0EBFCD88
Auto-setup: Add official Docker repository when docker-ce requested
Note: NEVER install docker.io (outdated), always use docker-ce
```

#### PostgreSQL PGDG
```
Aliases: postgresql, postgres, pgsql, pgdg
Packages: postgresql-16, postgresql-16-server, postgresql-16-contrib
Detection: postgresql.org URLs, GPG key ACCC4CF8
URLs by distro:
  Debian: https://apt.postgresql.org/pub/repos/apt
  RHEL: https://download.postgresql.org/pub/repos/yum/
```

#### EPEL (Extra Packages for Enterprise Linux)
```
Aliases: epel, epel-release
Install method: Package (epel-release)
For: RHEL, CentOS, Rocky, AlmaLinux, Fedora
```

#### Remi Repository (PHP)
```
Aliases: remi, remi-php, php
Default PHP version: 7.4 (for compatibility)
Available versions: 7.4, 8.0, 8.1, 8.2, 8.3
URLs: https://rpms.remirepo.net/enterprise/
Note: Only supports x86_64, use system PHP on ARM
```

#### MongoDB
```
Aliases: mongodb, mongo, mongod
Detection: repo.mongodb.org URLs, GPG key B00A0BD1E2C63C11
Note: ARM64 requires Ubuntu 20.04+ or RHEL 8+
```

#### Kubernetes
```
Aliases: kubernetes, k8s, kubectl, kubeadm
Packages: kubectl, kubeadm, kubelet
Detection: kubernetes.io URLs, packages.cloud.google.com/apt
```

#### Microsoft
```
Aliases: microsoft, edge, vscode, code, dotnet, powershell
Products: Edge browser, VS Code, .NET SDK, PowerShell Core
Detection: packages.microsoft.com URLs
Note: Multiple sub-repos for different products
```

#### HashiCorp
```
Aliases: hashicorp, terraform, vault, consul, nomad, packer
Detection: apt.releases.hashicorp.com, rpm.releases.hashicorp.com
GPG key: E8A032E094D8EB4EA189D270DA418C88A3219F7B
```

### Implicit Repository Addition
```
When installing a package that needs a repository:
- Automatically add the repository (no prompt)
- Import GPG keys
- Update package cache
- Install the package
- User requested install = implicit permission to add repo

Example:
$ pkmgr install docker-ce
📦 Package 'docker-ce' requires Docker repository
⏳ Adding Docker repository...
  • Downloading GPG key...
  • Adding repository to /etc/apt/sources.list.d/docker.list
  • Updating package cache...
✅ Repository added successfully
📦 Installing docker-ce...
```

## Arch Linux Special Handling

### Package Source Selection
```
Default: Official repositories via pacman
AUR: Only for specific packages or when not in repos

Automatically prefer AUR for:
- Proprietary software (VS Code, Slack, Discord, Spotify, Chrome)
- Official vendor builds (Teams, Zoom, Dropbox)
- Software not in official repos

Always use official repos for:
- System libraries and core tools
- Open source software
- Security-critical packages
- Development languages (Python, Go, Rust, etc.)
```

### AUR Preference List
```
Development Tools:
- visual-studio-code-bin (Microsoft's official binary)
- sublime-text-4 (proprietary)
- jetbrains-toolbox
- postman-bin
- insomnia-bin

Browsers:
- google-chrome (official Google build)
- microsoft-edge-stable-bin
- brave-bin
- vivaldi

Communication:
- slack-desktop
- discord
- teams
- zoom

Productivity:
- notion-app-electron
- obsidian
- typora

System Tools:
- timeshift
- yay-bin (pre-compiled)
- paru-bin (pre-compiled)

Media:
- spotify
- plex-media-server
```

### AUR Conflict Resolution
```
When AUR operations fail:

"exists in filesystem" errors:
  1. Check file ownership
  2. If unowned → Force overwrite
  3. If owned by related package → Replace
  4. Auto-fix: yay -S package --overwrite '*'

Package conflicts:
  1. Determine which is newer/better
  2. Auto-remove old, install new
  3. Rebuild dependents if needed

GPG key issues:
  1. Import key from keyserver
  2. Try multiple keyservers
  3. Skip verification if needed (with warning)

Build failures:
  1. Clear cache and retry
  2. Check for -bin alternative
  3. Modify PKGBUILD if needed
```

### Arch Update Strategy
```
System Update Process:
1. Update official repos first: pacman -Syu
2. Update AUR packages: yay -Sua
3. Rebuild AUR packages if needed
4. Handle conflicts automatically

Keyring Issues (common on Arch):
  1. Update archlinux-keyring first
  2. Refresh all keys
  3. Reset if needed: pacman-key --init && pacman-key --populate
  4. Update system time (often causes key issues)
```

## Extremely Smart Error Recovery

### Pattern Database
```
Built into binary: Database of common errors and solutions
- 250+ common pacman/AUR errors (Arch)
- 200+ apt error patterns (Debian/Ubuntu)
- 150+ DNF/YUM errors (Fedora/RHEL)
- Cross-distro patterns for common issues
```

### Universal Recovery Strategies

#### Strategy 1: Rebuild Against Current Libraries
```
Error: Package built against old library
Automatic Fix:
  1. Trigger rebuild with current libraries
  2. For AUR: yay -S package --rebuild
  3. Success Rate: 90%
```

#### Strategy 2: Configuration Migration
```
Error: Config format changed after update
Automatic Fix:
  1. Detect old config format
  2. Backup original
  3. Convert to new format
  4. Validate conversion
  5. Success Rate: 95%
```

#### Strategy 3: Clean Environment Rebuild
```
Error: Build fails due to environment
Automatic Fix:
  1. Clear all build caches
  2. Reset environment variables
  3. Use minimal PATH
  4. Rebuild in clean chroot
  5. Success Rate: 95%
```

#### Strategy 4: Force Overwrite (When Safe)
```
Error: File exists in filesystem
Automatic Fix:
  1. Check if file is orphaned
  2. Check if safe to overwrite
  3. Force overwrite if safe
  4. Backup original
  5. Success Rate: 98%
```

### Distribution-Specific Recovery

#### Arch Linux
```
Partial upgrade issues:
  - Force full system upgrade
  - Rebuild all AUR packages
  - Success rate: 95%

Keyring problems:
  - Update archlinux-keyring
  - Refresh keys
  - Reset keyring if needed
  - Success rate: 99%
```

#### Debian/Ubuntu
```
Broken dependencies:
  - dpkg --configure -a
  - apt --fix-broken install
  - Clear cache and retry
  - Success rate: 98%

Repository GPG issues:
  - Auto-fetch key from keyserver
  - Success rate: 100%
```

#### Fedora/RHEL
```
DNF database corruption:
  - rpm --rebuilddb
  - dnf clean all
  - Success rate: 100%

Module conflicts:
  - Reset module stream
  - Enable correct stream
  - Success rate: 90%
```

### Success Metrics
```
Error types handled automatically: 95%
Errors requiring user choice: 4%
Unrecoverable errors: 1% (hardware failure, network down, etc.)
```

## Package Preferences and Coexistence

### Smart Package Selection
```
Prefer modern alternatives (new installs only):
- mysql → mariadb (drop-in replacement)
- redis → valkey (open source fork)
- pulseaudio → pipewire (with compatibility)
- jack → jack2

Keep for compatibility:
- ifconfig (too many scripts use it, install alongside iproute2)
- Python 2 (if system needs it, never remove)
- PHP 7.4 (default for compatibility, not 8.x)
- Apache (when needed, configure with Nginx)
```

### Apache + Nginx Smart Configuration
```
When both needed:
1. Find free port starting at 64080
2. Store selection in ~/.config/pkmgr/service-ports.toml
3. Configure Apache on 127.0.0.1:{port}
4. Configure Nginx on *:80 and *:443
5. Auto-generate Nginx reverse proxy configs
6. Result: Nginx handles SSL/static, Apache handles apps

Port Selection:
  - Check if port already assigned in config
  - If not, scan from 64080-64099 for free port
  - Save selection for consistency
  - Reuse same port on future operations
```

### Intelligent Coexistence Examples
```
Web Server Stack:
  Apache + Nginx automatically configured
  Apache: 127.0.0.1:64082 (backend)
  Nginx: *:80/*:443 (frontend)
  Auto-proxy all Apache vhosts

Python Versions:
  Keep Python 2 if system needs it
  Default to Python 3 for user
  Never break system dependencies

PHP Versions:
  Default to 7.4 for compatibility
  Install 8.x only if explicitly needed
  Use Remi repo on RHEL systems
```

## Distribution-Aware Paths

### Service Paths by Distribution
```
DEBIAN/UBUNTU:
  Apache: /etc/apache2/
  Sites: /etc/apache2/sites-available/
  Service: apache2.service
  Binary: apache2

RHEL/FEDORA/CENTOS:
  Apache: /etc/httpd/
  Sites: /etc/httpd/conf.d/
  Service: httpd.service
  Binary: httpd

ARCH:
  Apache: /etc/httpd/conf/
  Sites: /etc/httpd/conf/extra/
  Service: httpd.service
  Binary: httpd

OPENSUSE:
  Apache: /etc/apache2/
  Sites: /etc/apache2/vhosts.d/
  Service: apache2.service
  Binary: httpd
```

### PHP Paths by Distribution
```
DEBIAN/UBUNTU:
  Config: /etc/php/7.4/apache2/php.ini
  FPM: /etc/php/7.4/fpm/
  CLI: /etc/php/7.4/cli/
  Mods: /etc/php/7.4/mods-available/

RHEL/FEDORA (with Remi):
  Config: /etc/opt/remi/php74/php.ini
  FPM: /etc/opt/remi/php74/php-fpm.d/

ARCH:
  Config: /etc/php/php.ini
  FPM: /etc/php/php-fpm.d/
  Extensions: /etc/php/conf.d/
```

### Vhost Organization (Our Standard)
```
Preferred structure:
/etc/{service}/vhosts.d/{fqdn}.conf

Examples:
/etc/nginx/vhosts.d/example.com.conf
/etc/httpd/conf/vhosts.d/shop.example.com.conf
/etc/apache2/vhosts.d/blog.example.org.conf

Debian compatibility:
- Create our vhosts.d/ structure
- Symlink from sites-available/enabled for compatibility
- Both methods work simultaneously
```

### Smart Path Detection
```
Finding configs:
1. Try distro-specific path first
2. Search common locations
3. Use 'apache2 -V' or 'httpd -V' to find HTTPD_ROOT
4. Cache result for speed

Service names:
  Debian/Ubuntu: systemctl start apache2
  RHEL/Fedora: systemctl start httpd
  All: systemctl start nginx
```

## Configuration Schema

```toml
# ~/.config/pkmgr/config.toml

[pkmgr]
version = "1.0.0"
last_update_check = "2024-01-15T10:30:00Z"
install_id = "auto-generated-uuid"

[defaults]
# Installation
install_location = "auto"        # auto|system|user
prefer_binary = true             # Prefer binary over source
allow_prerelease = false         # Include pre-release versions
parallel_downloads = 4           # Concurrent downloads
parallel_operations = 2          # Concurrent installs

# Display
color_output = "auto"           # auto|always|never
emoji_enabled = true            # Use emoji in output
progress_style = "bar"          # bar|dots|spinner|percent
verbosity = "normal"            # quiet|normal|verbose|debug
pager = "auto"                  # auto|less|more|never

# Behavior
auto_cleanup = true             # Clean cache automatically
auto_update_check = true        # Check updates daily
confirm_major_updates = true    # Prompt for major versions
keep_downloads = false          # Keep downloaded files
use_cache = true               # Use package cache

[paths]
cache_dir = "~/.cache/pkmgr"
data_dir = "~/.local/share/pkmgr"
config_dir = "~/.config/pkmgr"
install_dir = "~/.local"
iso_dir = "~/Downloads/ISOs"
temp_dir = "/tmp/pkmgr"

[network]
timeout = 30                    # Connection timeout seconds
retry_count = 3                 # Retry failed downloads
retry_delay = 5                 # Seconds between retries
bandwidth_limit = 0             # 0 = unlimited (KB/s)
proxy = ""                      # http://proxy:port
parallel_downloads = 4          # Concurrent downloads

[security]
verify_signatures = true        # Verify GPG signatures
verify_checksums = true         # Verify file checksums
allow_untrusted = false        # Allow untrusted sources
keyserver = "hkps://keys.openpgp.org"
key_refresh_days = 30          # Refresh expiring keys

[repositories]
# Custom repositories added by user
# Format: name = "url"

[aliases]
# Command aliases
i = "install"
r = "remove"
u = "update"
s = "search"

[language_defaults]
php = "7.4"                     # Default PHP version
python = "3"                    # Default Python version
node = "20"                     # Default Node.js version
ruby = "3.2"                    # Default Ruby version
go = "1.21"                     # Default Go version

[binary_sources]
prefer_github = true            # Prefer GitHub over GitLab
include_prerelease = false      # Include pre-release versions
asset_preference = ["static", "appimage", "archive"]
```

### Service Ports Configuration
```toml
# ~/.config/pkmgr/service-ports.toml
[apache]
backend_port = 64082  # Dynamically found free port

[tomcat]
backend_port = 64100

[services]
# Other services that need ports
```

### Binary Tracking
```toml
# ~/.local/share/pkmgr/binaries/installed.toml
[lazydocker]
source = "github"
repository = "jesseduffield/lazydocker"
version = "v0.23.1"
installed_date = "2024-01-15T10:30:00Z"
install_path = "/home/user/.local/bin/lazydocker"
download_url = "https://github.com/jesseduffield/lazydocker/releases/download/v0.23.1/lazydocker_0.23.1_Linux_x86_64.tar.gz"
size = 8912896
checksum = "sha256:abc123..."

[terraform]
source = "direct"
url = "https://releases.hashicorp.com/terraform/1.6.0/terraform_1.6.0_linux_amd64.zip"
version = "1.6.0"
installed_date = "2024-01-14T15:45:00Z"
install_path = "/usr/local/bin/terraform"
```

## Language-Specific Settings

### Python
```
Version file: .python-version
Manifest files: pyproject.toml, setup.py, requirements.txt
Version regex: ^\d+\.\d+(\.\d+)?$
Download URL: https://www.python.org/ftp/python/
Package command: pip
Package registry: https://pypi.org/simple/
Site packages: lib/python{major}.{minor}/site-packages/
Binary location: bin/python{major}.{minor}
Environment vars:
  PYTHONPATH: {base}/lib/python{version}/site-packages
  PYTHONUSERBASE: {base}
  PYTHONNOUSERSITE: 1
```

### Node.js
```
Version file: .nvmrc, .node-version
Manifest file: package.json (engines.node field)
Version regex: ^v?\d+\.\d+\.\d+$
Download URL: https://nodejs.org/dist/
Package command: npm
Package registry: https://registry.npmjs.org/
Package location: lib/node_modules/
Binary location: bin/node
Environment vars:
  NODE_PATH: {base}/lib/node_modules
  NPM_CONFIG_PREFIX: {base}
  NPM_CONFIG_USERCONFIG: {base}/.npmrc
```

### Go
```
Version file: .go-version, go.mod (go directive)
Version regex: ^\d+\.\d+(\.\d+)?$
Download URL: https://go.dev/dl/
Package location: pkg/
Binary location: bin/go
Environment vars:
  GOROOT: {base}
  GOPATH: {home}/go
  GOBIN: {base}/bin
  GO111MODULE: on
```

### Rust
```
Version file: rust-toolchain.toml, rust-toolchain
Version regex: ^\d+\.\d+\.\d+$
Download URL: https://forge.rust-lang.org/
Package command: cargo
Registry: https://crates.io/
Target dir: target/
Binary location: bin/rustc, bin/cargo
Environment vars:
  RUSTUP_HOME: {base}
  CARGO_HOME: {base}
  RUSTC: {base}/bin/rustc
```

### Ruby
```
Version file: .ruby-version, Gemfile (.ruby directive)
Version regex: ^\d+\.\d+\.\d+$
Download URL: https://cache.ruby-lang.org/pub/ruby/
Package command: gem
Package registry: https://rubygems.org/
Gem home: lib/ruby/gems/{version}/
Binary location: bin/ruby
Environment vars:
  GEM_HOME: {base}/lib/ruby/gems/{version}
  GEM_PATH: {base}/lib/ruby/gems/{version}
  RUBYLIB: {base}/lib/ruby/{version}
```

### PHP
```
Version file: .php-version, composer.json (require.php field)
Version regex: ^\d+\.\d+\.\d+$
Download URL: https://www.php.net/distributions/
Package command: composer
Package registry: https://packagist.org/
Config location: etc/php.ini
Binary location: bin/php
Default version: 7.4 (for compatibility)
Environment vars:
  PHP_INI_DIR: {base}/etc
  COMPOSER_HOME: {base}/.composer
```

### Java
```
Version file: .java-version, pom.xml, build.gradle
Version regex: ^\d+(\.\d+\.\d+)?$
Download URL: https://adoptium.net/
Binary location: bin/java, bin/javac
Environment vars:
  JAVA_HOME: {base}
  JRE_HOME: {base}/jre
  CLASSPATH: {base}/lib
```

### .NET
```
Version file: global.json, *.csproj (TargetFramework)
Version regex: ^\d+\.\d+(\.\d+)?$
Download URL: https://dotnet.microsoft.com/download/
Binary location: dotnet
Environment vars:
  DOTNET_ROOT: {base}
  DOTNET_CLI_HOME: {base}
  DOTNET_TOOLS_PATH: {base}/tools
```

## Logging Configuration
```
Log Settings:
- Single log file: ~/.local/share/pkmgr/pkmgr.log
- Maximum size: 10 MB
- No compression (raw text)
- No rotation (overwrite when size exceeded)
- UTF-8 encoding
- Human-readable format

Log Format:
2024-01-15 10:30:45 [INFO] Starting pkmgr v1.0.0
2024-01-15 10:30:45 [INFO] System: Ubuntu 22.04 x86_64
2024-01-15 10:30:45 [DEBUG] Loading config from ~/.config/pkmgr/config.toml
2024-01-15 10:30:46 [INFO] Command: install docker-ce
2024-01-15 10:30:46 [DEBUG] Checking for Docker repository
2024-01-15 10:30:47 [INFO] Adding Docker repository
2024-01-15 10:30:48 [SUCCESS] Repository added successfully
2024-01-15 10:30:49 [INFO] Installing package: docker-ce
2024-01-15 10:31:15 [SUCCESS] Package installed: docker-ce-24.0.7
2024-01-15 10:31:15 [INFO] Total time: 30 seconds

Log Levels:
- ERROR: Failed operations, exceptions
- WARN: Potential issues, deprecations
- INFO: Normal operations, status updates
- SUCCESS: Completed operations
- DEBUG: Detailed debugging info (verbose mode)

Privacy in Logs:
- Never log passwords or tokens
- Mask sensitive paths (/home/user → ~)
- No personal information
- No full URLs with credentials
- Hash identifiers for tracking
```

## SQLite Database (Optional)
```
Database Location: ~/.local/share/pkmgr/pkmgr.db

Schema:
-- Installed packages tracking
CREATE TABLE packages (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    version TEXT NOT NULL,
    source TEXT NOT NULL,
    install_date TIMESTAMP,
    install_path TEXT,
    size INTEGER,
    UNIQUE(name, source)
);

-- Binary installations
CREATE TABLE binaries (
    id INTEGER PRIMARY KEY,
    name TEXT UNIQUE NOT NULL,
    repository TEXT,
    version TEXT,
    url TEXT,
    install_path TEXT,
    install_date TIMESTAMP,
    auto_update BOOLEAN DEFAULT 1
);

-- Repository cache
CREATE TABLE repo_cache (
    source TEXT,
    package TEXT,
    version TEXT,
    description TEXT,
    size INTEGER,
    updated TIMESTAMP,
    PRIMARY KEY(source, package)
);

-- Transaction history
CREATE TABLE transactions (
    id TEXT PRIMARY KEY,
    timestamp TIMESTAMP,
    operation TEXT,
    packages TEXT,  -- JSON array
    success BOOLEAN,
    rollback_data TEXT  -- JSON
);
```

## Error Message Standards

### Network Errors
```
Connection Timeout:
❌ Connection to {host} timed out after 30 seconds
💡 Check your internet connection or try again later

DNS Resolution Failed:
❌ Cannot resolve hostname: {host}
💡 Check your DNS settings or network connection

SSL Certificate Error:
❌ SSL certificate verification failed for {host}
💡 Certificate may be expired or self-signed
   Use --insecure to bypass (not recommended)

Rate Limited:
⚠️ API rate limit exceeded for {service}
💡 Authenticate with token or wait {time} minutes
```

### Permission Errors
```
Insufficient Privileges:
❌ Permission denied: {operation} requires elevated privileges
💡 Re-running with sudo or install to user directory with --user

File Access Denied:
❌ Cannot access {path}: Permission denied
💡 Check file permissions or ownership
```

### Package Errors
```
Package Not Found:
❌ Package '{name}' not found in {source}
💡 Try: pkmgr search {name} to find similar packages

Version Conflict:
❌ Package '{name}' version {requested} conflicts with installed {current}
💡 Remove existing version first or use --force

Dependency Missing:
❌ Cannot install '{name}': Missing dependency '{dep}'
💡 Install dependency first: pkmgr install {dep}
```

### Disk Space Errors
```
Insufficient Space:
❌ Not enough disk space: Need {required}, have {available}
💡 Free up space or use different directory with --path

Cache Full:
⚠️ Cache directory full ({size} used)
💡 Run: pkmgr cache clean to free space
```

## Signal Handling
```
SIGINT (Ctrl+C):
  - Clean shutdown
  - Rollback in-progress operations
  - Remove partial downloads
  - Release locks
  - Exit code: 130
  
SIGTERM:
  - Same as SIGINT
  - Exit code: 143
  
SIGHUP:
  - Ignore (continue running)
  
SIGPIPE:
  - Handle gracefully (broken pipe)
  
Cleanup Time Limit: 5 seconds before force exit
```

## Architecture Support

### Architecture Detection and Compatibility
```
Architecture Detection:
  x86_64: [x86_64, amd64, x64, intel64]
  aarch64: [aarch64, arm64, armv8]
  armv7: [armv7, armhf, armv7l]
  i686: [i686, i386, x86, ia32]
  ppc64le: [ppc64le]
  s390x: [s390x]
  riscv64: [riscv64]

Binary Selection Priority:
  1. Exact architecture match
  2. Compatible architecture (e.g., i686 on x86_64)
  3. Universal/fat binary
  4. Source code (if build tools available)
```

### Architecture-Specific Repository Support
```
Repository Architecture Matrix:
  Docker: x86_64 ✓, aarch64 ✓, armv7 ✓
  PostgreSQL: x86_64 ✓, aarch64 ✓ (14+)
  MongoDB: x86_64 ✓, aarch64 ✓ (Ubuntu 20.04+/RHEL 8+)
  Remi (PHP): x86_64 ✓ only
  HashiCorp: x86_64 ✓, aarch64 ✓ (varies by tool)
  Microsoft: x86_64 ✓, aarch64 ✓ (partial)
  Elastic: x86_64 ✓, aarch64 ✓ (7.10+)
  Kubernetes: x86_64 ✓, aarch64 ✓, armv7 ✓
```

### Architecture-Specific Error Handling
```
When package not available for architecture:
1. Check for alternative packages
2. Suggest compilation from source
3. Recommend containerized version
4. Provide clear explanation

Example:
⚠️ Architecture Limitation
Remi repository only supports x86_64. Use system PHP packages instead.

Alternatives for PHP on ARM64:
1. Use system PHP packages (may be older)
2. Compile PHP from source
3. Use Docker container with PHP
```

## Cross-Platform Support

### Operating Systems
```
Linux:
  - All major distributions
  - Ubuntu, Debian, Fedora, Arch, openSUSE, Gentoo
  - Alpine, Void, NixOS
  - RHEL, CentOS, Rocky, AlmaLinux

macOS:
  - Intel (x86_64)
  - Apple Silicon (M1/M2/M3 - aarch64)
  - macOS 10.15+ (Catalina and newer)

Windows:
  - Windows 10+ (64-bit)
  - Windows 11
  - Windows Server 2019+
  - WSL2 support

BSD:
  - FreeBSD
  - OpenBSD
  - NetBSD
```

### Package Manager Detection
```
Linux:
  Ubuntu/Debian: apt (preferred), apt-get (fallback)
  Arch: pacman + AUR via yay/paru
  Fedora/RHEL: dnf (modern), yum (legacy)
  openSUSE: zypper + OBS
  Alpine: apk
  Gentoo: portage/emerge
  Void: xbps

macOS:
  Homebrew (primary)
  MacPorts (fallback)

Windows:
  winget (primary)
  chocolatey (secondary)
  scoop (tertiary)

BSD:
  FreeBSD: pkg
  OpenBSD: pkg_add
  NetBSD: pkgin
```

## Security and Trust

### NEVER Execute
```
❌ curl | sh
❌ wget | bash
❌ Any piped script execution
❌ Unverified scripts
❌ Scripts from HTTP (non-HTTPS)

Instead:
✅ Download script → Verify → Review → Execute
✅ Use official package repositories
✅ Verify GPG signatures
✅ Use checksums
```

### Repository Trust Levels
```
Trust Categories:
  Official: OS vendor repos (auto-trust)
  Verified: Known vendors like Docker, Microsoft (auto-trust)
  Community: PPAs, AUR, COPR (prompt user)
  Corporate: Internal mirrors (check consistency)
  Unknown: User-added (require confirmation)
```

### GPG Key Management
```
Key Verification:
  - Check fingerprint against known database
  - Verify key transitions (old key signs new)
  - Check DNS TXT records for key publication
  - Refresh expiring keys automatically
  
Key Sources:
  1. Official keyservers
  2. DNS TXT records
  3. HTTPS endpoints
  4. Package metadata
```

## System Safety Guarantees

### We Will NOT Break Systems
```
Protection Mechanisms:
1. Trust package managers (battle-tested)
2. Never bypass safety checks
3. Smart recovery for known issues
4. Backup critical files
5. Transaction rollback capability
6. Detect dangerous operations
7. Handle 99% of errors automatically
```

### Protected Operations
```
Never Remove:
  - Kernel
  - Init system (systemd)
  - Package manager itself
  - Core libraries (glibc)
  - Python if system depends on it
  
Never Modify:
  - /usr/bin (except through package manager)
  - /bin (system critical)
  - System library paths
  - Boot configuration (without backup)
```

### Safety Verification
```
Before operations:
  - Simulate operation (dry run)
  - Check dependency impacts
  - Verify disk space
  - Ensure network connectivity
  - Validate permissions
  
After operations:
  - Verify installation
  - Check system integrity
  - Ensure services running
  - Validate configurations
```

## Installation Scripts

### Production Installer Scripts (scripts/)

The project includes production-ready installer scripts for end users:

#### Universal Installer (install.sh)
Single installer that works across Linux, macOS, and BSD:
```bash
curl -fsSL https://raw.githubusercontent.com/pkmgr/pkmgr/main/scripts/install.sh | bash
```

Features:
- Auto-detects OS (Linux, macOS, FreeBSD, OpenBSD, NetBSD)
- Auto-detects architecture (x86_64, aarch64, armv7, i686)
- Downloads appropriate binary from GitHub releases
- Installs to system (`/usr/local/bin`) with sudo
- Falls back to user install (`~/.local/bin`) if no sudo
- Verifies installation after completion
- Beautiful emoji-based UI with fallback for non-Unicode terminals
- Proper error handling and recovery

#### Platform-Specific Wrappers
- **linux.sh**: Linux-specific wrapper (calls universal installer)
- **bsd.sh**: BSD-specific wrapper (calls universal installer)
- Future: Can add platform-specific logic if needed

#### Windows PowerShell Installer (windows.ps1)
PowerShell installer for Windows 10/11:
```powershell
iwr -useb https://raw.githubusercontent.com/pkmgr/pkmgr/main/scripts/windows.ps1 | iex
```

Features:
- Auto-detects architecture (x86_64, i686)
- Downloads Windows binary from GitHub releases
- Installs to Program Files with admin privileges
- Falls back to user install (LocalAppData) if not admin
- Updates PATH environment variable automatically
- Beautiful emoji-based output
- Proper error handling

### Development Scripts (scripts/)

Docker-based development scripts:

#### build.sh
Builds static binary using Docker multi-stage build:
```bash
./scripts/build.sh
```
- Uses musl target for true static linking
- Multi-stage Dockerfile for minimal size
- Outputs binary to `target/x86_64-unknown-linux-musl/release/pkmgr`

#### test.sh
Tests across multiple distributions:
```bash
./scripts/test.sh
```
- Tests on Ubuntu, Debian, Fedora, Arch
- Verifies package operations
- Tests command functionality

#### debug.sh
Interactive development environment:
```bash
./scripts/debug.sh
```
- Full Rust toolchain
- Persistent cargo cache
- Interactive shell access

#### clean.sh
Cleanup Docker resources:
```bash
./scripts/clean.sh
```
- Stops all containers
- Removes images
- Cleans up volumes

### Test Scripts (tests/)

Development and testing utilities:

#### check-compile.sh
Quick compilation verification:
```bash
./tests/check-compile.sh
```
- Runs `cargo check` for fast feedback
- Doesn't build full binary
- Useful for CI/CD

#### build-test.sh
Full build and test:
```bash
./tests/build-test.sh
```
- Builds release binary
- Runs basic tests
- Verifies functionality

### Build and Test Strategy

**IMPORTANT: Always use containers - NEVER run binaries directly on host**

#### Building
- **Always use Docker** for building the binary
- Multi-stage Dockerfile with musl target for static linking
- Reproducible builds across all environments
- No host contamination

```bash
# Build using Docker
./scripts/build.sh

# Or manually
docker-compose build pkmgr-dev
docker-compose run --rm pkmgr-dev cargo build --release
```

#### Testing
- **Always use Incus** (full OS containers) for testing
- Test on real distributions: Debian, Ubuntu, Fedora, AlmaLinux
- Never test binary on host system
- Ensures real-world behavior

```bash
# Test across distributions using Incus
incus launch images:debian/12 pkmgr-test-debian
incus launch images:ubuntu/22.04 pkmgr-test-ubuntu
incus launch images:fedora/39 pkmgr-test-fedora
incus launch images:almalinux/9 pkmgr-test-alma

# Copy binary to test container
incus file push target/release/pkmgr pkmgr-test-debian/tmp/

# Test inside container
incus exec pkmgr-test-debian -- /tmp/pkmgr --version
incus exec pkmgr-test-debian -- /tmp/pkmgr search vim
```

#### Why Containers Only?

**Safety:**
- Prevents breaking host system
- Isolated testing environment
- No host package manager interference

**Reproducibility:**
- Same environment every time
- Clean slate for each test
- Matches user environments

**Real-world Testing:**
- Test on actual distributions
- Real package managers
- Actual system behavior

### Installation Examples

**System installation (requires sudo):**
```bash
# Universal installer
curl -fsSL https://raw.githubusercontent.com/pkmgr/pkmgr/main/scripts/install.sh | bash

# Verify
pkmgr --version
```

**User installation (no sudo):**
```bash
# Download and install to ~/.local/bin
curl -fsSL https://raw.githubusercontent.com/pkmgr/pkmgr/main/scripts/install.sh | bash -s -- --user

# Add to PATH if needed
export PATH="$HOME/.local/bin:$PATH"
```

**Windows installation:**
```powershell
# Run as Administrator for system install
iwr -useb https://raw.githubusercontent.com/pkmgr/pkmgr/main/scripts/windows.ps1 | iex

# Or run as regular user for user install
iwr -useb https://raw.githubusercontent.com/pkmgr/pkmgr/main/scripts/windows.ps1 | iex -ArgumentList "--user"
```

**Build from source (using Docker):**
```bash
git clone https://github.com/pkmgr/pkmgr.git
cd pkmgr

# Build using Docker (NEVER build on host)
./scripts/build.sh

# Test in Incus container (NEVER test on host)
incus launch images:ubuntu/22.04 pkmgr-test
incus file push target/x86_64-unknown-linux-musl/release/pkmgr pkmgr-test/tmp/
incus exec pkmgr-test -- /tmp/pkmgr --version
```

## Summary

This specification defines a complete, production-ready universal package manager that:

- **Never breaks systems** through intelligent safety checks and single version policy
- **Handles all errors** with 99% automatic recovery using pattern database
- **Works everywhere** with cross-platform, multi-architecture support
- **Stays organized** with consistent vhost paths and smart configurations
- **Remains secure** with GPG verification, trust levels, and no shell execution
- **Provides beautiful UX** with emojis, progress bars, and minimal prompts
- **Learns and adapts** to each system's configuration and user preferences
- **Recovers gracefully** with transactions, rollback, and automatic fixes
- **Respects the system** by working with package managers, not against them
- **Simplifies complexity** with automatic repo addition, build tools, and dependencies

The entire system is implemented as a **single static Rust binary** with:
- No external dependencies
- No shell script execution
- Direct syscalls only
- Memory-safe operations
- Input validation
- Automatic sudo configuration for passwordless operation

This makes pkmgr portable, fast, secure, and reliable across all platforms while providing an exceptional user experience that "just works" without requiring deep technical knowledge.

