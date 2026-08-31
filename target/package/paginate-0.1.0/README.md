# paginate

Pagination types for Rust APIs — offset-based and cursor-based pagination with OpenAPI support.

## Overview

`paginate` provides strongly-typed pagination primitives for REST APIs. Instead of manually parsing query strings and building pagination responses, use the provided types to handle offset-based and cursor-based pagination with built-in clamping, validation, and response formatting.

## Features

- **Offset-based pagination** — `PaginationParams` and `PaginatedResponse<T>` for traditional page/per_page APIs
- **Cursor-based pagination** — `CursorPagination` and `CursorResponse<T>` for keyset/cursor APIs
- **OpenAPI support** — optional `utoipa` derives via the `openapi` feature
- **Serde support** — optional serialization via the `serde_impl` feature (enabled by default)
- **No `unsafe`** — the crate uses `#![forbid(unsafe_code)]`

## Usage

### Offset-based pagination

```rust
use paginate::{PaginationParams, PaginatedResponse};

// Parse from query parameters
let params = PaginationParams::new(3, 25);
assert_eq!(params.page(), 3);
assert_eq!(params.per_page(), 25);
assert_eq!(params.offset(), 50);

// Build a response
let items = vec!["a", "b", "c"];
let response = PaginatedResponse::new(items, 1, 100, 25);
assert_eq!(response.total_pages, 4);
assert_eq!(response.page, 1);
```

### Cursor-based pagination

```rust
use paginate::{CursorPagination, CursorResponse};

let cursor = CursorPagination::new(Some("abc123".into()), 20);
let items = vec![1, 2, 3];
let response = CursorResponse::new(items, Some("next_cursor".into()), true);
assert!(response.has_more);
assert_eq!(response.cursor.as_deref(), Some("next_cursor"));
```

## Comparison with manual pagination

Without `paginate`, you'd write something like:

```rust
// Manual approach — error-prone, repetitive
struct Params { page: u32, per_page: u32 }

let page = params.page.max(1);
let per_page = params.per_page.clamp(1, 100);
let offset = ((page - 1) * per_page) as usize;
let total_pages = (total as f64 / per_page as f64).ceil() as u64;
// ... plus serialization, validation, etc.
```

`paginate` handles all of this with a single type, including clamping bounds and computing derived fields.

## License

Licensed under either of [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE) at your option.
