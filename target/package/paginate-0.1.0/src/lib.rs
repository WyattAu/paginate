#![forbid(unsafe_code)]
//! Pagination types for Rust APIs.
//!
//! Provides offset-based and cursor-based pagination primitives with
//! optional serde and OpenAPI support.

mod cursor;
mod params;
mod response;

pub use cursor::{CursorPagination, CursorResponse};
pub use params::PaginationParams;
pub use response::PaginatedResponse;
