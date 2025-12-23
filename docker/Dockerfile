# Multi-stage Docker build for pkmgr universal package manager
# According to CLAUDE.md specification - Single static Rust binary
FROM rust:latest AS builder

# Install comprehensive system dependencies for cross-compilation and testing
RUN apt-get update && apt-get install -y \
    # Core build tools
    pkg-config \
    libssl-dev \
    libsqlite3-dev \
    build-essential \
    # Static linking support
    musl-tools \
    musl-dev \
    # Cross-compilation support
    gcc-aarch64-linux-gnu \
    gcc-arm-linux-gnueabihf \
    # Additional development tools
    git \
    curl \
    wget \
    file \
    which \
    # Package manager testing tools
    gnupg \
    ca-certificates \
    apt-transport-https \
    && rm -rf /var/lib/apt/lists/*

# Install rustup targets for cross-compilation according to spec
RUN rustup target add \
    x86_64-unknown-linux-musl \
    aarch64-unknown-linux-musl \
    armv7-unknown-linux-musleabihf

# Set up workspace
WORKDIR /app

# Copy dependency files first for better caching
COPY Cargo.toml ./

# Copy all source files
COPY . .

# Build the actual application - single static Rust binary as specified
# Must have zero dependencies according to spec
ENV RUSTFLAGS="-C target-feature=+crt-static"
RUN cargo build --release --target x86_64-unknown-linux-musl

# Verify it's actually static (should fail with "not a dynamic executable")
RUN ldd target/x86_64-unknown-linux-musl/release/pkmgr || echo "✅ Static binary confirmed"

# Stage 2: Ubuntu testing environment
FROM ubuntu:22.04 AS test-ubuntu

# Install comprehensive testing dependencies according to spec
RUN apt-get update && apt-get install -y \
    # Core system tools
    ca-certificates \
    curl \
    wget \
    gnupg \
    lsb-release \
    sudo \
    git \
    file \
    # Language runtimes for testing version resolution
    python3 \
    python3-pip \
    nodejs \
    npm \
    ruby \
    gem \
    default-jre \
    golang-go \
    # Build tools that pkmgr should auto-install
    build-essential \
    gcc \
    g++ \
    make \
    cmake \
    pkg-config \
    # Package manager testing
    apt-transport-https \
    software-properties-common \
    # USB/ISO testing utilities
    dosfstools \
    genisoimage \
    syslinux \
    # Network utilities for testing
    netcat-traditional \
    && rm -rf /var/lib/apt/lists/*

# Create test user with sudo privileges
RUN useradd -m -s /bin/bash testuser && \
    echo 'testuser ALL=(ALL) NOPASSWD:ALL' >> /etc/sudoers

# Copy the built static binary from builder stage
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/pkmgr /usr/local/bin/pkmgr

# Make binary executable
RUN chmod +x /usr/local/bin/pkmgr

# Create language command symlinks as specified in CLAUDE.md
# This is the core symlink strategy - all language commands point to pkmgr
RUN mkdir -p /home/testuser/.local/bin && \
    # Python ecosystem
    ln -sf /usr/local/bin/pkmgr /home/testuser/.local/bin/python && \
    ln -sf /usr/local/bin/pkmgr /home/testuser/.local/bin/python3 && \
    ln -sf /usr/local/bin/pkmgr /home/testuser/.local/bin/pip && \
    ln -sf /usr/local/bin/pkmgr /home/testuser/.local/bin/pip3 && \
    # Node.js ecosystem
    ln -sf /usr/local/bin/pkmgr /home/testuser/.local/bin/node && \
    ln -sf /usr/local/bin/pkmgr /home/testuser/.local/bin/npm && \
    ln -sf /usr/local/bin/pkmgr /home/testuser/.local/bin/npx && \
    ln -sf /usr/local/bin/pkmgr /home/testuser/.local/bin/yarn && \
    # Ruby ecosystem
    ln -sf /usr/local/bin/pkmgr /home/testuser/.local/bin/ruby && \
    ln -sf /usr/local/bin/pkmgr /home/testuser/.local/bin/gem && \
    ln -sf /usr/local/bin/pkmgr /home/testuser/.local/bin/bundle && \
    ln -sf /usr/local/bin/pkmgr /home/testuser/.local/bin/irb && \
    # Rust ecosystem
    ln -sf /usr/local/bin/pkmgr /home/testuser/.local/bin/cargo && \
    ln -sf /usr/local/bin/pkmgr /home/testuser/.local/bin/rustc && \
    ln -sf /usr/local/bin/pkmgr /home/testuser/.local/bin/rustup && \
    # Go ecosystem
    ln -sf /usr/local/bin/pkmgr /home/testuser/.local/bin/go && \
    ln -sf /usr/local/bin/pkmgr /home/testuser/.local/bin/gofmt && \
    # Java ecosystem
    ln -sf /usr/local/bin/pkmgr /home/testuser/.local/bin/java && \
    ln -sf /usr/local/bin/pkmgr /home/testuser/.local/bin/javac && \
    ln -sf /usr/local/bin/pkmgr /home/testuser/.local/bin/jar && \
    # .NET ecosystem
    ln -sf /usr/local/bin/pkmgr /home/testuser/.local/bin/dotnet && \
    # PHP ecosystem
    ln -sf /usr/local/bin/pkmgr /home/testuser/.local/bin/php && \
    ln -sf /usr/local/bin/pkmgr /home/testuser/.local/bin/composer

# Create comprehensive pkmgr directory structure according to specification
RUN mkdir -p \
    /home/testuser/.config/pkmgr \
    /home/testuser/.local/share/pkmgr/languages \
    /home/testuser/.local/share/pkmgr/transactions \
    /home/testuser/.local/share/pkmgr/cache \
    /home/testuser/.local/share/pkmgr/binaries \
    /home/testuser/.cache/pkmgr \
    /home/testuser/Downloads/ISOs \
    /tmp/pkmgr && \
    chown -R testuser:testuser /home/testuser

# Set PATH to include pkmgr symlinks (critical for symlink strategy)
ENV PATH="/home/testuser/.local/bin:/usr/local/bin:${PATH}"

# Switch to test user for testing
USER testuser
WORKDIR /home/testuser

# Stage 3: Arch Linux testing environment
FROM archlinux:latest AS test-arch

# Update and install comprehensive testing tools
RUN pacman -Syu --noconfirm && \
    pacman -S --noconfirm \
    base-devel \
    git \
    sudo \
    which \
    file \
    curl \
    wget \
    python \
    nodejs \
    npm \
    ruby \
    go \
    rust \
    cargo

# Create non-root user for AUR testing (AUR requires non-root)
RUN useradd -m -G wheel testuser && \
    echo "testuser ALL=(ALL) NOPASSWD: ALL" >> /etc/sudoers

# Install yay AUR helper for testing AUR functionality
USER testuser
WORKDIR /home/testuser
RUN git clone https://aur.archlinux.org/yay.git && \
    cd yay && \
    makepkg -si --noconfirm

USER root
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/pkmgr /usr/local/bin/pkmgr
RUN chmod +x /usr/local/bin/pkmgr

USER testuser
ENV PATH="/home/testuser/.local/bin:/usr/local/bin:${PATH}"

# Stage 4: Fedora testing environment
FROM fedora:39 AS test-fedora

RUN dnf update -y && \
    dnf install -y \
    dnf-plugins-core \
    rpm-build \
    git \
    curl \
    wget \
    which \
    file \
    sudo \
    gcc \
    gcc-c++ \
    make \
    python3 \
    python3-pip \
    nodejs \
    npm \
    ruby \
    gem \
    golang \
    && dnf clean all

RUN useradd -m -s /bin/bash testuser && \
    echo 'testuser ALL=(ALL) NOPASSWD:ALL' >> /etc/sudoers

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/pkmgr /usr/local/bin/pkmgr
RUN chmod +x /usr/local/bin/pkmgr

USER testuser
ENV PATH="/home/testuser/.local/bin:/usr/local/bin:${PATH}"

# Stage 5: Final development environment with all testing capabilities
FROM test-ubuntu AS final

# Copy built binary for development
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/pkmgr /usr/local/bin/pkmgr

# Ensure proper permissions
USER root
RUN chmod +x /usr/local/bin/pkmgr

# Switch back to test user
USER testuser
WORKDIR /home/testuser

# Default command for testing and development
ENTRYPOINT ["/usr/local/bin/pkmgr"]
CMD ["--help"]