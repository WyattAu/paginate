/// Paginated response for offset-based pagination.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde_impl", derive(serde::Serialize))]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct PaginatedResponse<T> {
    /// The items in this page.
    pub items: Vec<T>,
    /// Current page number (1-indexed).
    pub page: u32,
    /// Number of items per page.
    pub per_page: u32,
    /// Total number of items across all pages.
    pub total: u64,
    /// Total number of pages.
    pub total_pages: u64,
}

impl<T> PaginatedResponse<T> {
    /// Creates a new paginated response, computing `total_pages` automatically.
    pub fn new(items: Vec<T>, page: u32, total: u64, per_page: u32) -> Self {
        let total_pages = if per_page == 0 {
            0
        } else {
            (total as f64 / per_page as f64).ceil() as u64
        };

        Self {
            items,
            page,
            per_page,
            total,
            total_pages,
        }
    }

    /// Creates a paginated response from a pre-fetched `Vec`.
    ///
    /// This is a convenience for cases where all items are already in memory
    /// and you just want to slice + wrap them.
    pub fn from_vec(all_items: Vec<T>, page: u32, per_page: u32) -> Self {
        let total = all_items.len() as u64;
        let offset = ((page.max(1) - 1) * per_page) as usize;
        let items: Vec<T> = all_items
            .into_iter()
            .skip(offset)
            .take(per_page as usize)
            .collect();

        Self::new(items, page, total, per_page)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_computes_total_pages() {
        let r = PaginatedResponse::new(vec![1, 2, 3], 1, 100, 25);
        assert_eq!(r.total_pages, 4);
    }

    #[test]
    fn new_handles_zero_per_page() {
        let r: PaginatedResponse<i32> = PaginatedResponse::new(vec![], 1, 100, 0);
        assert_eq!(r.total_pages, 0);
    }

    #[test]
    fn from_vec_slices_correctly() {
        let all = vec![10, 20, 30, 40, 50];
        let r = PaginatedResponse::from_vec(all, 2, 2);
        assert_eq!(r.items, vec![30, 40]);
        assert_eq!(r.total, 5);
        assert_eq!(r.total_pages, 3);
    }
}
