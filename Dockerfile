# ── Stage 1: Build ──────────────────────────────────────────────────────────
FROM rust:1-bookworm AS builder

# Add PGDG repository so we can get pg18 server-dev headers
RUN apt-get update && apt-get install -y gnupg2 curl ca-certificates && \
    curl -fsSL https://www.postgresql.org/media/keys/ACCC4CF8.asc \
        | gpg --dearmor -o /usr/share/keyrings/postgresql.gpg && \
    echo "deb [signed-by=/usr/share/keyrings/postgresql.gpg] \
https://apt.postgresql.org/pub/repos/apt bookworm-pgdg main" \
        > /etc/apt/sources.list.d/pgdg.list && \
    apt-get update && apt-get install -y \
        postgresql-server-dev-18 \
        libclang-dev \
        clang \
        pkg-config \
        libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /build

# Install cargo-pgrx at the exact version used by the project
RUN cargo install cargo-pgrx --version "=0.18.0" --locked

# Copy workspace manifests first (layer-cache friendly)
COPY Cargo.toml Cargo.lock ./
COPY pgpulse/Cargo.toml        pgpulse/Cargo.toml
COPY pgpulse-exporter/Cargo.toml pgpulse-exporter/Cargo.toml

# Copy full source
COPY pgpulse/        pgpulse/
COPY pgpulse-exporter/ pgpulse-exporter/

# Initialise pgrx home using the system pg18 installation (no source download).
RUN cargo pgrx init --pg18 /usr/lib/postgresql/18/bin/pg_config

# Build the extension package for PostgreSQL 18.
# Output lands in target/release/pgpulse-pg18/ mirroring the system layout.
RUN cargo pgrx package \
        --features pg18 \
        --pg-config /usr/lib/postgresql/18/bin/pg_config \
        -p pgpulse

# ── Stage 2: Runtime ────────────────────────────────────────────────────────
FROM postgres:18

# Extension shared library
COPY --from=builder \
    /build/target/release/pgpulse-pg18/usr/lib/postgresql/18/lib/pgpulse.so \
    /usr/lib/postgresql/18/lib/pgpulse.so

# Control file + SQL migration
COPY --from=builder \
    /build/target/release/pgpulse-pg18/usr/share/postgresql/18/extension/ \
    /usr/share/postgresql/18/extension/
