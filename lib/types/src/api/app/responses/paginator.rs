use serde::Serialize;

/// A generic API Response type implementing Serialize that can be used for any kind of Response
/// that can be a part of multiple pages
#[derive(Serialize, Clone)]
pub struct Paginator<T: Serialize + Clone> {
    /// Paginator content
    content: T,

    /// Total amount of Pages
    pages: usize,

    /// Current page
    current_page: usize,
}

impl<T: Serialize + Clone> Paginator<T> {
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
    pub fn with_pages(content: T, current_page: usize, pages: usize) -> Self {
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
    pub fn set_current_page(&mut self, current_page: usize) {
        assert!(current_page <= self.pages);
        self.current_page = current_page;
    }

    /// Set the paginator's pages.
    ///
    /// # Panics
    ///
    /// Panics if `current_page` > `pages`
    pub fn set_pages(&mut self, pages: usize) {
        assert!(self.current_page <= pages);
        self.pages = pages;
    }

    /// Get the paginator's pages.
    pub fn pages(&self) -> usize {
        self.pages
    }

    /// Get the paginator's current page.
    pub fn current_page(&self) -> usize {
        self.current_page
    }
}
