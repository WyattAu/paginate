//! Property-based tests for api-paginate crate.

use proptest::prelude::*;

use api_paginate::{PaginationParams, PaginatedResponse, CursorPagination};

#[test]
fn page_always_at_least_1() {
    proptest!(|(page in 0u32..10_000u32)| {
        let params = PaginationParams::new(page, 20);
        prop_assert!(params.page() >= 1);
    });
}

#[test]
fn per_page_clamped_to_1_100() {
    proptest!(|(per_page in 0u32..10_000u32)| {
        let params = PaginationParams::new(1, per_page);
        prop_assert!(params.per_page() >= 1 && params.per_page() <= 100);
    });
}

#[test]
fn offset_consistent_with_page_and_per_page() {
    proptest!(|(page in 1u32..1000u32, per_page in 1u32..100u32)| {
        let params = PaginationParams::new(page, per_page);
        let expected_offset = ((page - 1) * per_page) as usize;
        prop_assert_eq!(params.offset(), expected_offset);
    });
}

#[test]
fn offset_always_non_negative() {
    proptest!(|(page in 0u32..10_000u32, per_page in 1u32..100u32)| {
        let params = PaginationParams::new(page, per_page);
        prop_assert!(params.offset() >= 0);
    });
}

#[test]
fn default_params() {
    let params = PaginationParams::default();
    assert_eq!(params.page(), 1);
    assert_eq!(params.per_page(), 20);
    assert_eq!(params.offset(), 0);
}

#[test]
fn total_pages_computed_correctly() {
    proptest!(|(total in 0u64..10_000u64, per_page in 1u32..100u32)| {
        let response = PaginatedResponse::<i32>::new(vec![], 1, total, per_page);
        let expected = (total as f64 / per_page as f64).ceil() as u64;
        prop_assert_eq!(response.total_pages, expected);
    });
}

#[test]
fn from_vec_returns_correct_page() {
    proptest!(|(
        items in prop::collection::vec(0i32..1000, 0..200),
        page in 1u32..50u32,
        per_page in 1u32..50u32,
    )| {
        let total = items.len() as u64;
        let response = PaginatedResponse::from_vec(items, page, per_page);
        prop_assert_eq!(response.total, total);
        prop_assert!(response.items.len() <= per_page as usize);
    });
}

#[test]
fn cursor_limit_clamped() {
    proptest!(|(limit in 0u32..10_000u32)| {
        let pagination = CursorPagination::new(None, limit);
        prop_assert!(pagination.limit >= 1 && pagination.limit <= 100);
    });
}

#[test]
fn cursor_pagination_default() {
    let pagination = CursorPagination::default();
    assert!(pagination.cursor.is_none());
    assert_eq!(pagination.limit, 20);
}
