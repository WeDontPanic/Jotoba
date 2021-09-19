use std::cmp::min;

/// The amount of buttons the paginator should display max.
const BUTTONS_TO_DISPLAY: u8 = 5;

/// A Pagination structure holding information about pagination
#[derive(Clone, Copy, Default)]
pub struct Pagination {
    pub curr_page: u32,
    pub items: u32,
    pub items_per_page: u32,
}

impl Pagination {
    /// Returns the number of the last page
    #[inline]
    pub fn get_last(&self) -> u32 {
        let last = (self.items as f32 / self.items_per_page as f32).ceil() as u32;
        // Max 100 pages
        min(last, 100)
    }

    /// Returns `true` if the current page is the first page
    #[inline]
    pub fn is_first(&self) -> bool {
        self.curr_page == 1
    }

    /// Returns `true` if the current page is the last page
    #[inline]
    pub fn is_last(&self) -> bool {
        self.curr_page == self.get_last()
    }

    /// Generates the pagination buttons
    pub fn gen_page_buttons(&self) -> impl Iterator<Item = PaginationButton> + '_ {
        let btn_count = min(BUTTONS_TO_DISPLAY as u32, self.get_last());
        let h_btns = btn_count / 2;

        let right_btns_inv = h_btns - (self.get_last() - self.curr_page).min(h_btns);
        let start = self
            .curr_page
            .saturating_sub(h_btns + right_btns_inv)
            // Don't show 0 pages if only one exists
            .max(1);

        let end = min(start + btn_count, self.get_last() + 1);

        (start..end).map(move |page| PaginationButton::new(page, page == self.curr_page))
    }
}

/// Data for a single frontend pagination button.
#[derive(Copy, Clone)]
pub struct PaginationButton {
    pub page_nr: u32,
    pub active: bool,
}

impl PaginationButton {
    /// Create a new `PaginationButton`
    #[inline]
    fn new(page: u32, active: bool) -> PaginationButton {
        PaginationButton {
            page_nr: page,
            active,
        }
    }
}
