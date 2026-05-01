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

Run migrations from the service directory.

For `agent`:

```bash
cd agent
sqlx migrate run --source migrations
```

For `core`:

```bash
cd agent
sqlx migrate run --source migrations
```

```bash
cargo run -p agent
cargo run -p core
```

### Commands for running connections to agents
First cert
```bash
openssl s_client \
        -connect 127.0.0.1:6969 \
        -cert certs/dev/client.crt \
        -key certs/dev/client.key \
        -CAfile certs/dev/ca.crt
```
Second cert
```bash
openssl s_client \
		-connect 127.0.0.1:6969 \
		-CAfile certs/dev/ca.crt \
		-cert certs/dev/client2.crt \
		-key certs/dev/client2.key
```
