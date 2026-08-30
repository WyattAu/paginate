/// Offset-based pagination parameters.
///
/// Parses and validates `page` and `per_page` query values, clamping
/// `per_page` to `[1, 100]` and `page` to `[1, ∞)`.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde_impl", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct PaginationParams {
    page: u32,
    per_page: u32,
}

impl PaginationParams {
    /// Creates new pagination parameters.
    ///
    /// `page` is clamped to a minimum of 1. `per_page` is clamped to `[1, 100]`.
    pub fn new(page: u32, per_page: u32) -> Self {
        Self {
            page: page.max(1),
            per_page: per_page.clamp(1, 100),
        }
    }

    /// Returns the current page number (1-indexed).
    pub fn page(&self) -> u32 {
        self.page
    }

    /// Returns the number of items per page.
    pub fn per_page(&self) -> u32 {
        self.per_page
    }

    /// Returns the offset (0-indexed) for database queries.
    pub fn offset(&self) -> usize {
        ((self.page - 1) * self.per_page) as usize
    }
}

impl Default for PaginationParams {
    fn default() -> Self {
        Self::new(1, 20)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_params() {
        let p = PaginationParams::default();
        assert_eq!(p.page(), 1);
        assert_eq!(p.per_page(), 20);
        assert_eq!(p.offset(), 0);
    }

    #[test]
    fn custom_params() {
        let p = PaginationParams::new(3, 25);
        assert_eq!(p.page(), 3);
        assert_eq!(p.per_page(), 25);
        assert_eq!(p.offset(), 50);
    }

    #[test]
    fn clamps_page_to_minimum_1() {
        let p = PaginationParams::new(0, 10);
        assert_eq!(p.page(), 1);
    }

    #[test]
    fn clamps_per_page_to_100() {
        let p = PaginationParams::new(1, 200);
        assert_eq!(p.per_page(), 100);
    }

    #[test]
    fn clamps_per_page_to_1() {
        let p = PaginationParams::new(1, 0);
        assert_eq!(p.per_page(), 1);
    }
}
