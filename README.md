# SManager

Minimal run guide for local development.

## Services

- agent: mTLS TCP server on 127.0.0.1:6969
- core: WebSocket server on ws://127.0.0.1:6767

## Quick Start

1. Create env file:

```powershell
Copy-Item .env.example .env
```

2. Start PostgreSQL containers:

```powershell
docker compose up -d
docker compose ps
```

3. Run migrations:

```powershell
cargo install sqlx-cli --no-default-features --features native-tls,postgres

$env:DATABASE_URL = ((Get-Content .env | Where-Object { $_ -match '^DATABASE_URL_Agent=' }) -replace '^DATABASE_URL_Agent=', '')
sqlx migrate run --source agent/migrations

$env:DATABASE_URL = ((Get-Content .env | Where-Object { $_ -match '^DATABASE_URL_CORE=' }) -replace '^DATABASE_URL_CORE=', '')
sqlx migrate run --source core/migrations
```

4. Start servers (in separate terminals):

```powershell
cargo run -p agent
cargo run -p core
```

## Notes

- agent and core load variables from .env automatically.
- Certificates are expected in certs/dev (see CERTIFICATES_LOCATION in .env).
- Stop DB containers with:

```powershell
docker compose down
```