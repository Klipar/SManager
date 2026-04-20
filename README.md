# SManager

## Quick start

To make the project work, you need to create `.env` files.

### 1. Create `.env` files

Copy `.env.example` into:

* root `.env`
* `agent/.env`
* `core/.env`

---

### 2. Service `.env` files

In `agent/.env` and `core/.env` keep **only this line**:

```env
DATABASE_URL=...
```

This is used by SQLx migrations and database connection.

---

### 3. Root `.env`

The root `.env` contains **all application configuration except database settings**, for example:

* logging
* environment (dev/prod)
* ports defaults
* general runtime settings

---

## Important rule

* Service `.env` -> only `DATABASE_URL`
* Root `.env` -> all other configuration

---

## Run

```bash
docker compose up -d
```

```bash
sqlx migrate run --source agent/migrations
sqlx migrate run --source core/migrations
```

```bash
cargo run -p agent
cargo run -p core
```