use criterion::{criterion_group, criterion_main, Criterion};
use api_paginate::{PaginationParams, PaginatedResponse, CursorPagination, CursorResponse};

fn bench_pagination_params_creation(c: &mut Criterion) {
    c.bench_function("pagination_params_creation", |b| {
        b.iter(|| PaginationParams::new(1, 20));
    });
}

fn bench_pagination_params_default(c: &mut Criterion) {
    c.bench_function("pagination_params_default", |b| {
        b.iter(|| PaginationParams::default());
    });
}

fn bench_pagination_params_offset(c: &mut Criterion) {
    let params = PaginationParams::new(5, 25);
    c.bench_function("pagination_params_offset", |b| {
        b.iter(|| params.offset());
    });
}

fn bench_pagination_params_clamp(c: &mut Criterion) {
    c.bench_function("pagination_params_clamp", |b| {
        b.iter(|| PaginationParams::new(0, 500));
    });
}

fn bench_paginated_response_new(c: &mut Criterion) {
    c.bench_function("paginated_response_new", |b| {
        b.iter(|| PaginatedResponse::new(vec![1, 2, 3], 1, 1000, 25));
    });
}

fn bench_paginated_response_from_vec(c: &mut Criterion) {
    let all_items: Vec<u32> = (0..1000).collect();
    c.bench_function("paginated_response_from_vec", |b| {
        b.iter(|| PaginatedResponse::from_vec(all_items.clone(), 5, 20));
    });
}

fn bench_paginated_response_from_vec_large(c: &mut Criterion) {
    let all_items: Vec<u32> = (0..10000).collect();
    c.bench_function("paginated_response_from_vec_large", |b| {
        b.iter(|| PaginatedResponse::from_vec(all_items.clone(), 50, 100));
    });
}

fn bench_cursor_pagination_creation(c: &mut Criterion) {
    c.bench_function("cursor_pagination_creation", |b| {
        b.iter(|| CursorPagination::new(Some("abc123".into()), 20));
    });
}

fn bench_cursor_pagination_default(c: &mut Criterion) {
    c.bench_function("cursor_pagination_default", |b| {
        b.iter(|| CursorPagination::default());
    });
}

fn bench_cursor_pagination_clamp(c: &mut Criterion) {
    c.bench_function("cursor_pagination_clamp", |b| {
        b.iter(|| CursorPagination::new(None, 500));
    });
}

fn bench_cursor_response_creation(c: &mut Criterion) {
    c.bench_function("cursor_response_creation", |b| {
        b.iter(|| CursorResponse::new(vec![1, 2, 3], Some("cursor-abc".into()), true));
    });
}

fn bench_cursor_response_empty(c: &mut Criterion) {
    c.bench_function("cursor_response_empty", |b| {
        b.iter(|| CursorResponse::<i32>::empty());
    });
}

criterion_group!(
    benches,
    bench_pagination_params_creation,
    bench_pagination_params_default,
    bench_pagination_params_offset,
    bench_pagination_params_clamp,
    bench_paginated_response_new,
    bench_paginated_response_from_vec,
    bench_paginated_response_from_vec_large,
    bench_cursor_pagination_creation,
    bench_cursor_pagination_default,
    bench_cursor_pagination_clamp,
    bench_cursor_response_creation,
    bench_cursor_response_empty,
);
criterion_main!(benches);
