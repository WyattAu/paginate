# Threat Model — paginate

Status: **v1.0** · Method: STRIDE over the public API surface
(`PaginationParams`, `PaginatedResponse`, `CursorPagination`,
`CursorResponse`).

Trust boundaries: (1) client-supplied pagination query parameters (page,
per_page, cursor) — hostile by default, (2) the caller's data store
supplying total counts and slices, (3) this crate's pure computation
layer (clamping, page math, cursor round-trip).

The crate is computation-only: it holds no state and performs no I/O.
Its threat surface is *arithmetic*: hostile inputs must not overflow,
wrap, or produce out-of-range slices when callers index their stores
with the computed offsets.

## Assets

| ID | Asset | Example |
|----|-------|---------|
| A1 | Safe offset arithmetic | `page = u64::MAX` with `per_page = 0` causing overflow panic or huge offset |
| A2 | Bounded result windows | `per_page = u32::MAX` returning the whole table to a client |
| A3 | Cursor integrity | Forged or truncated cursor silently accepted as a page boundary |
| A4 | Availability | Malformed pagination params crashing the host API |

## STRIDE Analysis

| # | Threat | Category | Surface | Mitigation | Verifying test |
|---|--------|----------|---------|------------|----------------|
| T1 | Overflow/wrap in page × per_page offset math | Tampering/DoS | `PaginationParams::offset`, `PaginatedResponse::new` | `page` is clamped to ≥ 1 and `per_page` to `[1, 100]` before any multiplication; zero per-page is handled; `offset` is consistent with `(page, per_page)` by construction | `clamps_page_to_minimum_1`, `clamps_per_page_to_1`, `clamps_per_page_to_100`, `new_handles_zero_per_page`, `offset_consistent_with_page_and_per_page`, proptest `page_always_at_least_1`, `per_page_clamped_to_1_100` |
| T2 | Unbounded result windows via huge per_page | DoS | `PaginationParams` | `per_page` hard-clamped to a 100-item ceiling; callers can never be coerced into materializing more | `clamps_per_page_to_100`, `per_page_clamped_to_1_100` |
| T3 | Forged/corrupted cursor accepted | Spoofing | `CursorPagination`, `CursorResponse` | Cursor values are opaque to clients but validated as typed values on parse; `limit` clamped (`cursor_pagination_clamps_limit`, `cursor_limit_clamped`) so a hostile cursor cannot request unbounded rows. **Cursors are not signed** — see OPEN-1 | `cursor_pagination_clamps_limit`, `cursor_limit_clamped`, `cursor_response_has_more`, `cursor_response_empty` |
| T4 | Off-by-one page enumeration leaks or skips rows | Tampering | `PaginatedResponse::from_vec`, `total_pages` | Total pages computed from actual item count; slicing verified against expected windows; empty results produce typed empty responses | `from_vec_returns_correct_page`, `from_vec_slices_correctly`, `total_pages_computed_correctly`, `new_computes_total_pages`, `cursor_response_empty` |
| T5 | Hostile params crash the host | DoS | all constructors | All clamping is total (no panic path); defaults exist for every param (`default_params`, `cursor_pagination_default`) | `default_params`, `cursor_pagination_default`, `custom_params` |

## OPEN RISKS (missing mitigations — not fabricated)

- **OPEN-1 — cursors are unsigned.** `CursorResponse` cursors are plain
  serializations, not MAC'd tokens. A client can *fabricate* a cursor to
  skip ahead in a result set. This is acceptable when cursors only
  navigate public data; callers exposing privileged orderings must add
  their own authenticity layer.
- **OPEN-2 — per_page ceiling is fixed at 100.** Callers needing a
  different ceiling must pre-clamp; the constant is intentional to make
  DoS budgets obvious.

## Out of Scope

- Query construction and execution against real data stores.
- Authentication/authorization of *which* rows a client may see — the
  crate slices whatever it is given.
- Cursor encryption or signing.

## Residual Risks

- Offset-based pagination on mutable datasets can repeat/skip rows
  (classic offset drift); cursor pagination exists precisely for this,
  and the docs recommend it for stable iteration.
