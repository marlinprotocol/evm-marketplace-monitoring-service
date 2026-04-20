# Marketplace Monitoring Service

A Rust-based service that monitors the Oyster Marketplace smart contract for `JobOpened` events, verifies enclave reachability, and logs errors to a database.

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Diesel CLI](https://diesel.rs/guides/getting-started) (for running migrations)
- [PM2](https://pm2.keymetrics.io/)

## 1. Build the Project

```bash
# Clone the repository
git clone https://github.com/marlinprotocol/evm-marketplace-monitoring-service.git
cd evm-marketplace-monitoring-service

# Build in release mode
cargo build --release
```

## 2. Setup the Environment

Copy the example environment file and fill in your values:

```bash
cp .env.example .env
```

Edit `.env` with your configuration:

```env
RPC_URL=https://your-rpc-endpoint-url
CONTRACT_ADDRESS=0xYourContractAddress
DATABASE_URL=postgresql://user:password@host:port/database
NETWORK=ArbOne
```

| Variable           | Description                                          |
| ------------------ | ---------------------------------------------------- |
| `RPC_URL`          | RPC endpoint for the target blockchain               |
| `CONTRACT_ADDRESS` | Oyster Marketplace contract address                  |
| `DATABASE_URL`     | PostgreSQL connection string                         |
| `NETWORK`          | Network identifier (e.g. `ArbOne`, `BNB`)            |

### Run Database Migrations

```bash
diesel migration run
```

## 3. Start the Project

### Start with PM2

Use PM2 for process management, automatic restarts, and log management:

```bash
# Start the service
pm2 start ./target/release/marketplace-monitoring-service --name arb-monitor

# View logs
pm2 logs arb-monitor

# Restart / Stop
pm2 restart arb-monitor
pm2 stop arb-monitor

# Enable startup on boot
pm2 save
pm2 startup
```
