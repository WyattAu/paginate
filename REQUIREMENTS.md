# Requirements — paginate

Numbered, testable requirements. Every requirement maps to at least one named
test; every security-relevant test cites at least one requirement. Threat
IDs reference `THREAT-MODEL.md`.

Scope note: `api-paginate` provides pagination types for Rust APIs —
offset-based (`PaginationParams`, `PaginatedResponse`) and cursor-based
(`CursorPagination`, `CursorResponse`) pagination with OpenAPI schema
support. Pure computation: clamping, page math, and windowing only.

## Functional

| ID | Requirement | Priority |
|----|-------------|----------|
| REQ-PG-001 | `PaginationParams::new(page, per_page)` computes `total_pages` from the item count, handling zero items and zero per-page without division errors | MUST |
| REQ-PG-002 | `offset()` derives the store offset consistent with `(page, per_page)` | MUST |
| REQ-PG-003 | `PaginatedResponse::from_vec` slices exactly the requested window and reports correct paging metadata | MUST |
| REQ-PG-004 | `CursorPagination` carries limit + cursor; `limit` is clamped to the allowed ceiling | MUST |
| REQ-PG-005 | `CursorResponse` reports `has_more` and produces an empty typed response when no items remain | MUST |
| REQ-PG-006 | Defaults exist for both pagination styles (`default_params`, `cursor_pagination_default`) and custom params construct cleanly | SHOULD |

## Security

| ID | Requirement | Priority |
|----|-------------|----------|
| REQ-PG-100 | `page` is clamped to ≥ 1 and `per_page` to `[1, 100]` before any offset multiplication — no overflow or zero-division panic from hostile query params (T1) | MUST |
| REQ-PG-101 | Result windows are bounded: no client-supplied input can request more than the per-page ceiling (T2) | MUST |
| REQ-PG-102 | Cursor limits are clamped so a forged cursor cannot demand unbounded rows (T3) | MUST |
| REQ-PG-103 | No panic path exists on any input: clamping is total and total-pages math is division-safe (T5) | MUST |

## Robustness

| ID | Requirement | Priority |
|----|-------------|----------|
| REQ-PG-200 | Offset math stays consistent across the full clamped input domain (property-tested) | MUST |
| REQ-PG-201 | Total-page computation is correct across item counts and page sizes | SHOULD |

## Traceability Matrix

| Requirement | Test (fn, file) | Property class |
|-------------|-----------------|----------------|
| REQ-PG-001 | `new_computes_total_pages`, `total_pages_computed_correctly`, `new_handles_zero_per_page` (`src/lib.rs` tests) | unit |
| REQ-PG-002 | `offset_consistent_with_page_and_per_page` | unit |
| REQ-PG-003 | `from_vec_returns_correct_page`, `from_vec_slices_correctly` | unit |
| REQ-PG-004 | `cursor_pagination_clamps_limit`, `cursor_limit_clamped` | unit |
| REQ-PG-005 | `cursor_response_has_more`, `cursor_response_empty` | unit |
| REQ-PG-006 | `default_params`, `cursor_pagination_default`, `custom_params` | unit |
| REQ-PG-100 | `clamps_page_to_minimum_1`, `clamps_per_page_to_1`, `clamps_per_page_to_100`, `page_always_at_least_1`, `per_page_clamped_to_1_100` (`tests/proptest.rs`) | unit/property |
| REQ-PG-101 | `clamps_per_page_to_100`, `per_page_clamped_to_1_100` | unit/property |
| REQ-PG-102 | `cursor_pagination_clamps_limit`, `cursor_limit_clamped` | unit |
| REQ-PG-103 | `new_handles_zero_per_page`, `page_always_at_least_1`, `per_page_clamped_to_1_100` | unit/property |
| REQ-PG-200 | `offset_consistent_with_page_and_per_page` (`tests/proptest.rs`) | property |
| REQ-PG-201 | `total_pages_computed_correctly`, `new_computes_total_pages` | unit |

## Test Count

- 19 `#[test]` functions in the unit suite (pagination/clamp property
  checks included in the same count).
- All-features suite passes with 0 failures; no-default-features suite passes.
