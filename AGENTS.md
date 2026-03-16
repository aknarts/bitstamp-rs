# AGENTS.md — bitstamp-rs Knowledge Base

## Project Identity

**bitstamp-rs** is a Rust library crate for the [Bitstamp cryptocurrency exchange API](https://www.bitstamp.net/api/). It provides a REST client (API v2) and a WebSocket streaming client for real-time market data.

- **Crate name**: `bitstamp`
- **Edition**: 2018
- **License**: MIT

## Architecture

### Module Structure

```
src/
├── lib.rs              # Crate root — re-exports, module declarations
├── client.rs           # HTTP client internals (Bitstamp struct, auth, request pipeline)
├── error.rs            # Custom error type (boxed inner pattern)
├── types/
│   ├── mod.rs          # Re-exports all type submodules
│   ├── rest.rs         # REST API request/response types
│   ├── ws.rs           # WebSocket event types, channels, currency pairs
│   ├── errors.rs       # API error response types (V1Error, V2Error)
│   ├── deposits.rs     # Deposit address types
│   ├── derivatives.rs  # Derivatives position/margin types
│   ├── earn.rs         # Earn subscription types
│   ├── fees.rs         # Fee types
│   ├── instant_convert.rs # Instant convert address types
│   ├── orders.rs       # Order request/response types
│   ├── security.rs     # API key revocation types
│   ├── sub_account.rs  # Sub-account transfer types
│   ├── transactions.rs # Transaction types
│   ├── travel_rule.rs  # Travel rule (VASP, contact, address, satoshi test) types
│   ├── websocket_token.rs # WebSocket token type
│   └── withdrawals.rs  # Withdrawal request/response types
├── api/
│   ├── mod.rs          # Re-exports all API modules
│   ├── tickers.rs      # Ticker endpoints (public)
│   ├── market_info.rs  # Markets, OHLC, EUR/USD, funding rates (public)
│   ├── order_book.rs   # Order book endpoint (public)
│   ├── transactions.rs # Public + private transaction endpoints
│   ├── account.rs      # Account balances (private)
│   ├── fees.rs         # Trading + withdrawal fees (private)
│   ├── orders.rs       # Buy/sell/cancel/status (private)
│   ├── withdrawals.rs  # Withdrawal management (private)
│   ├── deposits.rs     # Deposit addresses (private)
│   ├── earn.rs         # Earn subscribe/unsubscribe (private)
│   ├── derivatives.rs  # Positions, margin, leverage (private)
│   ├── sub_account.rs  # Sub-account transfers (private)
│   ├── websocket.rs    # WebSocket token endpoint (private)
│   ├── travel_rule.rs  # Travel rule endpoints (VASP, contacts, addresses, satoshi test)
│   ├── instant_convert.rs # Instant convert address endpoints (private)
│   └── security.rs     # API key revocation (private)
└── ws.rs               # BitstampEventStream (WebSocket client)
```

### Key Design Patterns

- **Impl-blocks-across-files**: The `Bitstamp` struct is defined in `client.rs`. Each `api/*.rs` file adds an `impl Bitstamp { ... }` block with endpoint methods. This is idiomatic Rust — no traits or sub-clients needed.
- **Layered REST pipeline**: `get_*()` → `api_get()`/`api_post()` → `rest_api()` → `call_web_api_raw()` (auth + HTTP)
- **Error unification**: All public methods return `Result<T, Error>` using the custom error type
- **String-typed fields**: API response numeric fields (prices, amounts) stay as `String` matching Bitstamp's JSON format

### Authentication

HMAC-SHA256 signing for private endpoints:
1. Generate UUID v4 nonce
2. Build message: `BITSTAMP {key}POST{url}{content_type}{nonce}{timestamp}{payload}v2`
3. Sign with secret, hex-encode
4. Set headers: `X-Auth`, `X-Auth-Signature`, `X-Auth-Nonce`, `X-Auth-Timestamp`, `X-Auth-Version`

### Dependencies

| Crate              | Purpose                    |
|--------------------|----------------------------|
| hyper + hyper-tls  | HTTP client                |
| tokio-tungstenite  | WebSocket client           |
| serde + serde_json | Serialization              |
| hmac + sha2 + hex  | HMAC-SHA256 authentication |
| uuid               | Nonce generation           |
| chrono             | Timestamps                 |
| log                | Logging facade             |
| futures-util       | Async stream utilities     |

## Code Conventions

### Adding a New REST Endpoint

1. **Define types** in `src/types/rest.rs` (or appropriate submodule):
   ```rust
   #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
   pub struct NewResponse {
       pub field: String,
   }
   ```

2. **Add the method** in the appropriate `src/api/*.rs` file:
   ```rust
   use crate::error::Error;
   use crate::types::NewResponse;

   impl crate::client::Bitstamp {
       /// Description of endpoint
        pub async fn get_new_thing(&self, param: &str) -> Result<NewResponse, Error> {
            let rest_method = format!("new_thing/{}/", param);
            self.api_get(rest_method.as_str()).await
        }
    }
    ```

3. **For POST (authenticated) endpoints**, use `self.api_post(path, body).await`

4. **For GET endpoints that require authentication** (private data like positions, earn, contacts), use `self.api_auth_get(path).await` instead of `self.api_get()`

### Error Handling

- Use `text_error()` / `text_error_with_inner()` for generic errors
- Use `map_err(|e| text_error_with_inner(..., e))` with `?` — never `.unwrap()`
- Error constructors are `pub(crate)` — not exposed to consumers

### Naming

- Public API methods: `get_<resource>`, `cancel_<resource>`, etc.
- Types: PascalCase structs, derive `Default, Debug, Clone, PartialEq, Serialize, Deserialize`
- Use `String` for all API numeric fields in response types

### Testing

- Unit tests in `src/tests.rs` — 31 serde round-trip tests
- Run: `cargo test`
- When adding: use plain `#[test]` for serde tests, `#[tokio::test]` for async tests
- Mock the HTTP client for unit tests (don't hit real API)

### Running Examples with Credentials

API credentials are stored in `.env` (do NOT read this file directly). To run authenticated examples:

```sh
set -a && source .env && set +a && cargo run --example account
```

Environment variables: `BITSTAMP_ACCESS_KEY`, `BITSTAMP_ACCESS_SECRET`

Public examples (ticker, market_data, websocket) don't require credentials.

## Reference

- **OpenAPI spec**: `openapi.json` (full Bitstamp API v2 specification, ~85 endpoints)
- **Bitstamp API docs**: https://www.bitstamp.net/api/
- **Progress tracking**: See `PLAN.md`
