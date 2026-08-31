/// Cursor-based pagination parameters.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde_impl", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct CursorPagination {
    /// The cursor to resume from. `None` for the first page.
    pub cursor: Option<String>,
    /// Maximum number of items to return.
    pub limit: u32,
}

impl CursorPagination {
    /// Creates a new cursor-based pagination request.
    ///
    /// `limit` is clamped to `[1, 100]`.
    pub fn new(cursor: Option<String>, limit: u32) -> Self {
        Self {
            cursor,
            limit: limit.clamp(1, 100),
        }
    }
}

impl Default for CursorPagination {
    fn default() -> Self {
        Self {
            cursor: None,
            limit: 20,
        }
    }
}

/// Cursor-based paginated response.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde_impl", derive(serde::Serialize))]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct CursorResponse<T> {
    /// The items in this page.
    pub items: Vec<T>,
    /// The cursor to use for the next page. `None` if no more pages.
    pub cursor: Option<String>,
    /// Whether there are more items available.
    pub has_more: bool,
}

impl<T> CursorResponse<T> {
    /// Creates a new cursor-based response.
    pub fn new(items: Vec<T>, cursor: Option<String>, has_more: bool) -> Self {
        Self {
            items,
            cursor,
            has_more,
        }
    }

    /// Creates an empty response indicating no results.
    pub fn empty() -> Self {
        Self {
            items: Vec::new(),
            cursor: None,
            has_more: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cursor_pagination_clamps_limit() {
        let p = CursorPagination::new(None, 500);
        assert_eq!(p.limit, 100);
    }

    #[test]
    fn cursor_response_has_more() {
        let r = CursorResponse::new(vec![1, 2], Some("abc".into()), true);
        assert!(r.has_more);
        assert_eq!(r.cursor.as_deref(), Some("abc"));
    }

    #[test]
    fn cursor_response_empty() {
        let r: CursorResponse<i32> = CursorResponse::empty();
        assert!(r.items.is_empty());
        assert!(!r.has_more);
    }
}
