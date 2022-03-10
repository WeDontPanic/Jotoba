use serde::Serialize;

/// A generic API Response type implementing Serialize that can be used for any kind of Response
/// that can be a part of multiple pages
#[derive(Serialize, Clone)]
pub struct Page<T: Serialize + Clone> {
    /// Paginator content
    content: T,

    /// Total amount of Pages
    pages: u32,

    /// Current page
    current_page: u32,
}

impl<T: Serialize + Clone> Page<T> {
    /// Creates a new Paginator with default values
    pub fn new(content: T) -> Self {
        Self {
            content,
            pages: 1,
            current_page: 1,
        }
    }

    /// Creates a new Paginator with non default page values
    ///
    /// # Panics
    ///
    /// Panics if `current_page` > `pages`
    pub fn with_pages(content: T, current_page: u32, pages: u32) -> Self {
        assert!(current_page <= pages);
        Self {
            content,
            current_page,
            pages,
        }
    }

    /// Set the paginator's current page.
    ///
    /// # Panics
    ///
    /// Panics if `current_page` > `pages`
    pub fn set_current_page(&mut self, current_page: u32) {
        assert!(current_page <= self.pages);
        self.current_page = current_page;
    }

    /// Set the paginator's pages.
    ///
    /// # Panics
    ///
    /// Panics if `current_page` > `pages`
    pub fn set_pages(&mut self, pages: u32) {
        assert!(self.current_page <= pages);
        self.pages = pages;
    }

    /// Get the paginator's pages.
    pub fn pages(&self) -> u32 {
        self.pages
    }

    /// Get the paginator's current page.
    pub fn current_page(&self) -> u32 {
        self.current_page
    }
}
