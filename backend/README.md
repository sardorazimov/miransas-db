# miransas-db backend

Backend-only Rust service for managing internal project metadata, database connection notes, encrypted secret records, and simple admin summaries for personal infrastructure work.

This is intentionally not a SaaS backend. There is no billing, organization model, subscription logic, or tenant isolation layer.

## Stack

- Rust
- axum
- tokio
- sqlx with PostgreSQL
- serde
- tracing
- uuid
- chrono
- anyhow / thiserror

## Local setup

1. Create a PostgreSQL database:

   ```sh
   createdb miransas_db
   ```

2. Create a local environment file:

   ```sh
   cp .env.example .env
   ```

3. Fill in the required values:

   - `DATABASE_URL`
   - `ADMIN_TOKEN`
   - `SECRET_KEY`

4. Run the backend:

   ```sh
   cargo run
   ```

The service listens on `APP_HOST:APP_PORT`, defaulting to `127.0.0.1:3001`.

## Environment variables

| Name | Required | Purpose |
| --- | --- | --- |
| `APP_HOST` | No | IP address to bind. Defaults to `127.0.0.1`. |
| `APP_PORT` | No | Port to bind. Defaults to `3001`. |
| `RUST_LOG` | No | Tracing filter. Example: `miransas_db=info,tower_http=info`. |
| `DATABASE_URL` | Yes | PostgreSQL connection URL. |
| `DATABASE_MAX_CONNECTIONS` | No | SQLx pool size. Defaults to `10`. |
| `ADMIN_TOKEN` | Yes | Bearer token for protected `/api` routes. Must be at least 16 characters. |
| `SECRET_KEY` | Yes | Key material used to encrypt stored secrets. Must be at least 32 characters. |

## Auth

`/health` is public. Every route under `/api` requires:

```http
Authorization: Bearer <ADMIN_TOKEN>
```

## Endpoints

- `GET /health`
- `GET /api/projects`
- `POST /api/projects`
- `GET /api/databases`
- `POST /api/databases`
- `GET /api/secrets`
- `POST /api/secrets`
- `GET /api/admin/summary`

## Migrations

Migrations live in `migrations/` and are embedded with `sqlx::migrate!`.

On startup, the backend connects to PostgreSQL and runs any pending migrations before accepting traffic.

The first migration creates:

- `projects`
- `databases`
- `secrets`
- `audit_logs`

Secret values are encrypted before being stored in `secrets.value_encrypted`. The crypto logic is isolated in `src/utils/crypto.rs` so it can be replaced or hardened later without touching handlers or database code.

## Tests

Run:

```sh
cargo test
```

The current tests exercise the public health endpoint and auth rejection for protected routes. They use a lazy SQLx pool, so PostgreSQL does not need to be running for those tests.
