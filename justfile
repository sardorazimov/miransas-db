# Run the application
run:
    cd backend && cargo run

# Run database migrations
migrate:
    cd backend && cargo sqlx migrate run

# Format code
fmt:
    cd backend && cargo fmt

# Run clippy linter
lint:
    cd backend && cargo clippy -- -D warnings

# Run tests
test:
    cd backend && cargo test

# Check formatting, linting, and tests
check:
    cd backend && cargo fmt --check && cargo clippy -- -D warnings && cargo test

# Start Docker services
docker-up:
    cd docker && docker compose up -d

# Stop Docker services
docker-down:
    cd docker && docker compose down

# Run benchmarks
bench:
    cd backend && cargo bench
