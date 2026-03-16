# Code Style

## Naming Conventions

| Element          | Convention         | Example                                      |
|------------------|--------------------|----------------------------------------------|
| Crate            | kebab-case (repo), snake_case (lib) | `bitstamp-rs` / `bitstamp`      |
| Files            | snake_case         | `client.rs`, `order_book.rs`                 |
| Structs          | PascalCase         | `Bitstamp`, `OrderBook`, `PairInfo`          |
| Enums            | PascalCase         | `EventChannel`, `CurrencyPairs`, `Kind`      |
| Enum variants    | PascalCase         | `LiveTrades`, `Btcusd`, `ErrorV2`            |
| Functions        | snake_case         | `get_ticker`, `call_web_api_raw`             |
| Constants        | SCREAMING_SNAKE    | `REST_HOST_PREFIX`                           |
| Type aliases     | PascalCase         | `WStream`, `WebClient`, `HmacSha256`        |
| Public API methods | `get_<resource>` prefix | `get_ticker`, `get_balance`            |

## Module Organization

```
src/
├── lib.rs          # Thin crate root: module declarations + pub re-exports only
├── client.rs       # Bitstamp struct, constructor, HTTP transport (private module)
├── error.rs        # Error type and factory functions (private module)
├── ws.rs           # BitstampEventStream (public module)
├── types/          # All data types (public module)
│   ├── mod.rs      # Re-exports: pub use rest::*; pub use ws::*; pub use errors::*;
│   ├── rest.rs     # REST request/response types
│   ├── ws.rs       # WebSocket event types
│   └── errors.rs   # API error response types
└── api/            # Endpoint methods as impl Bitstamp blocks (private module)
    ├── mod.rs      # Module declarations
    └── *.rs        # One file per endpoint group
```

### Key conventions:
- `lib.rs` contains ONLY module declarations and `pub use` re-exports
- `client.rs` is `mod client` (private) — HTTP internals are hidden
- `api/` is `mod api` (private) — endpoint files add `impl crate::client::Bitstamp` blocks
- `types/` is `pub mod types` — all types are public
- `ws` is `pub mod ws` — WebSocket client is public

## Import Style

- Use `use log::{debug, warn};` (not `#[macro_use] extern crate log`)
- Use `use crate::` for internal imports
- Group imports: crate → external → std

## Derive Conventions

Standard derive set for data structs:
```rust
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ticker { ... }
```

For enums (no `Default`):
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventEvent { ... }
```

## Adding a New Endpoint

1. **Define types** in `src/types/rest.rs`:
   ```rust
   #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
   pub struct NewResponse { pub field: String }
   ```

2. **Add method** in `src/api/<group>.rs` (or create new file + add to `api/mod.rs`):
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

3. **For POST endpoints** (authenticated), use `self.api_post(path, body).await`

## Error Handling

- Use `text_error()` / `text_error_with_inner()` for generic errors
- Use `map_err(|e| text_error_with_inner(..., e))` with `?` — never `.unwrap()`
- Error constructors are `pub(crate)` — not exposed to consumers
- All public methods return `Result<T, Error>` (both REST and WebSocket)

## Struct Field Types

- All API response fields use `pub String` — numeric values (prices, amounts, fees) are kept as strings
- Exception: `PairInfo` uses `i64` for `base_decimals` and `counter_decimals`
- WebSocket `EventData` variants use native types (`i64`, `f64`, `String`)

## Logging

- Uses `log` crate via `use log::{debug, warn};`
- `debug!` for normal flow (connections, API calls, parsing)
- `warn!` for recoverable errors (deserialization failures)
- Example uses `env_logger` for output

## Do's and Don'ts

### Do
- Split endpoint methods into domain-specific `api/*.rs` files
- Use `impl crate::client::Bitstamp` in api files (impl-blocks-across-files pattern)
- Keep `api_get()`/`api_post()` as `pub(crate)` — accessible to api/ modules only
- Derive `Default, Debug, Clone, PartialEq, Serialize, Deserialize` on data structs
- Use `String` for API response numeric fields
- Return `Result<T, Error>` from all public methods
- Use `map_err` + `?` instead of `.unwrap()`

### Don't
- Don't put logic in `lib.rs` — it's only for re-exports
- Don't expose error construction to library consumers
- Don't use `.unwrap()` in library code
- Don't use `extern crate` — use modern `use` imports
- Don't change public API signatures without updating the example
