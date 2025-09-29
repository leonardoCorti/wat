# Justfile for this project

# Default recipe: list tasks
default:
    @just --list

# -------------------
# Helpers
# -------------------

# Extract version from Cargo.toml
version := `cd client; cargo metadata --no-deps --format-version 1 | jq -r '.packages[0].version'`

# Extract crate name
crate_name := `cd client; cargo metadata --no-deps --format-version 1 | jq -r '.packages[0].name'`

# -------------------
# Rust
# -------------------

# Build Rust project
build:
    cd client && cargo build

# Run tests
test:
    cd client && cargo test

# Build release binary into ./release/bin/
release-rust:
    cd client && cargo build --release --all-features
    mkdir -p release/bin
    cp client/target/release/{{crate_name}} release/bin/

# -------------------
# Node.js server
# -------------------

# Install dependencies for server
server-install:
    cd server && npm install

# Run the server
server-run:
    cd server && node rest_api.js

# Run server in dev mode (auto-restart if nodemon is installed)
server-dev:
    cd server && npx nodemon rest_api.js

# -------------------
# Release bundle (Rust + Node)
# -------------------

release: release-rust
    # Copy server files (excluding node_modules & tokens)
    rsync -a --exclude node_modules --exclude '*.token' server/ release/server/

    # Install production deps
    cd release/server && npm install --production

    # Create tarball with version from Cargo.toml
    tar -czf wat-{{version}}.tgz -C release .
    echo "✅ Release package created: wat-{{version}}.tgz"

# -------------------
# Utilities
# -------------------

# Clean build artifacts
clean:
    cd client && cargo clean
    rm -rf release wat-*.tgz
    cd server && rm -rf node_modules

