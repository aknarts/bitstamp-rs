# Architecture

## Overview

`bitstamp-rs` is a Rust library crate for communicating with the [Bitstamp cryptocurrency exchange API](https://www.bitstamp.net/api/). It provides both a REST client (Bitstamp API v2) and a WebSocket streaming client for real-time market data.

## Tech Stack

| Component        | Technology                          | Version |
|------------------|-------------------------------------|---------|
| Language         | Rust (edition 2018)                 | —       |
| Async runtime    | Tokio                               | 1.10    |
| HTTP client      | hyper + hyper-tls                   | 0.14    |
| WebSocket        | tokio-tungstenite + tungstenite     | 0.15    |
| Serialization    | serde + serde_json                  | 1.0     |
| Auth (HMAC)      | hmac + sha2 + hex                   | 0.11    |
| Logging          | log                                 | 0.4     |
| License          | MIT                                 |         |

## Directory Structure

```
bitstamp-rs/
├── Cargo.toml
├── src/
│   ├── lib.rs              # Crate root — module declarations + pub re-exports
│   ├── client.rs           # Bitstamp struct, constructor, HTTP pipeline, auth
│   ├── error.rs            # Custom error type (boxed inner pattern)
│   ├── ws.rs               # BitstampEventStream (WebSocket client)
│   ├── types/
│   │   ├── mod.rs          # Re-exports all type submodules
│   │   ├── rest.rs         # REST API request/response types
│   │   ├── ws.rs           # WebSocket event types, channels, currency pairs
│   │   └── errors.rs       # API error response types (V1Error, V2Error)
│   └── api/
│       ├── mod.rs          # Module declarations for endpoint groups
│       ├── tickers.rs      # Ticker endpoints (get_ticker, get_hourly_ticker)
│       ├── market_info.rs  # get_trading_pairs_info, get_eur_usd
│       ├── order_book.rs   # get_order_book
│       ├── transactions.rs # get_transactions
│       └── account.rs      # get_balance
├── examples/
│   └── hello-world.rs
├── openapi.json            # Reference OpenAPI spec (Bitstamp API v2, ~85 endpoints)
├── PLAN.md                 # Implementation progress tracker
└── AGENTS.md               # Project knowledge base
```

## Core Components

### `Bitstamp` (REST Client) — `src/client.rs`

The main API client struct. Holds an HTTPS hyper client and API credentials (key + secret).

**Layered REST pipeline:**
```
get_*()              → Typed public methods in api/*.rs (build URL, call api_get/api_post)
  api_get/api_post() → pub(crate) wrappers in client.rs setting HTTP method
    rest_api()       → Deserializes JSON response into typed structs
      call_web_api_raw() → Builds HTTP request, handles auth headers, sends request, reads response body
```

**Design pattern: impl blocks split across files.** The `Bitstamp` struct is defined in `client.rs`. Each `api/*.rs` file adds an `impl Bitstamp { ... }` block with endpoint-specific methods. This is idiomatic Rust — no traits or sub-clients needed.

### `BitstampEventStream` (WebSocket Client) — `src/ws.rs`

Real-time event streaming via WebSocket. Created via `Bitstamp::event_stream()`.

**Public methods:**
- `next()` — Pull-based async iteration with timeout; returns `Result<Event, Error>`
- `subscribe(channel)` — Subscribe to a WebSocket channel; returns `Result<(), Error>`
- `unsubscribe(channel)` — Unsubscribe from a channel; returns `Result<(), Error>`

### Types — `src/types/`

All public data types, split by domain:
- **`rest.rs`** — REST response/request types: `Ticker`, `OrderBook`, `Transaction`, `PairInfo`, `ConversionRate`, `AccountBalance`, `Time`, `Offset`, `Empty`
- **`ws.rs`** — WebSocket types: `Event`, `EventEvent`, `EventChannel`, `EventData`, `OutEvent`, `OutEventData`, `CurrencyPairs`
- **`errors.rs`** — API error responses: `V2Error`, `V1Error`

All re-exported via `types/mod.rs` so consumers use `types::Ticker`, `types::Event`, etc.

### Error — `src/error.rs`

Private module with a public `Error` struct. Uses a boxed inner pattern for size optimization.

**Error variants (via `Kind` enum):**
- `Text(String)` — Generic text error
- `Status(StatusCode)` — HTTP status code only
- `ErrorV1(StatusCode, String)` — Bitstamp V1 error format
- `ErrorV2(StatusCode, String, String)` — Bitstamp V2 error format (reason + code)

## Data Flow

### REST API Call
```
User calls bts.get_ticker("btcusd")
  → api/tickers.rs builds URL: "ticker/btcusd/"
  → client.rs api_get() → rest_api() → call_web_api_raw()
  → For POST: generates HMAC-SHA256 signature with nonce + timestamp
  → Sends HTTPS request via hyper
  → Reads response body via hyper::body::to_bytes()
  → On error status: tries V2Error → V1Error → bare StatusCode
  → On success: deserializes JSON into typed struct
  → Returns Result<T, Error>
```

### WebSocket Event Stream
```
User calls bts.event_stream()
  → client.rs connects to wss://ws.bitstamp.net via tokio-tungstenite
  → Returns BitstampEventStream with 20s timeout

User calls ws.subscribe(EventChannel::LiveTrades(Btcusd))
  → ws.rs serializes OutEvent to JSON
  → Sends as WebSocket Text message, returns Result<(), Error>

User calls ws.next() in a loop
  → ws.rs reads next WebSocket message with timeout
  → Handles Binary/Text → deserializes to Event
  → Handles Ping/Pong/Close → returns None (skipped, resets timeout)
  → Returns Result<Event, Error>
```

## Authentication

HMAC-SHA256 signing for private endpoints (in `client.rs`):
1. Generate UUID v4 nonce
2. Get current UTC timestamp (seconds + milliseconds)
3. Build message: `BITSTAMP {key}POST{url}{content_type}{nonce}{timestamp}{payload}v2`
4. Sign with secret via HMAC-SHA256, hex-encode
5. Set headers: `X-Auth`, `X-Auth-Signature`, `X-Auth-Nonce`, `X-Auth-Timestamp`, `X-Auth-Version`

## Configuration

- **No config files** — all configuration is via constructor parameters
- `Bitstamp::new(secret, key)` — API credentials passed at construction time
- Environment variables used in example only: `BITSTAMP_ACCESS_SECRET`, `BITSTAMP_ACCESS_KEY`
- WebSocket timeout hardcoded to 20 seconds

## Build & Run

```bash
cargo build                    # Build the library
cargo test                     # Run tests
cargo check --example hello-world  # Check the example

# Run the example (requires API credentials)
BITSTAMP_ACCESS_SECRET=<secret> BITSTAMP_ACCESS_KEY=<key> cargo run --example hello-world
```
