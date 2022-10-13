pub mod out_builder;
pub mod producer;
pub mod searchable;

use std::time::Instant;

use crate::result::SearchResult;
use engine::{pushable::FilteredMaxCounter, utils::page_from_pqueue_with_max_dist};
use log::debug;
use out_builder::OutputBuilder;
use searchable::Searchable;
use types::jotoba::search::guess::{Guess, GuessType};

/// Max items to count for estimation
pub const MAX_ESTIMATE: usize = 100;

/// Executes a search
pub struct SearchExecutor<S: Searchable> {
    search: S,
}

impl<S: Searchable> SearchExecutor<S> {
    /// Creates a new SearchExecutor
    #[inline]
    pub fn new(search: S) -> Self {
        Self { search }
    }

    /// Executes the search
    pub fn run(self) -> SearchResult<S::OutItem, S::ResAdd> {
        let query = self.search.get_query();
        let limit = query.settings.page_size as usize;
        let offset = query.page_offset;

        let mut out = OutputBuilder::new(|i| self.search.filter(i), limit + offset);

        for prod in self.search.get_producer() {
            if !prod.should_run(out.p.total_pushed()) {
                continue;
            }
            let before = out.p.total_pushed();
            let start = Instant::now();
            prod.produce(&mut out);
            let dur = start.elapsed();
            let after = out.p.total_pushed();
            let name = prod.name();
            debug!("{name}: {} Elements in {:?}", after - before, dur);
        }

        self.search.mod_output(&mut out);

        if out.is_empty() {
            return SearchResult::default();
        }

        // Get total len of results
        let len;
        if let Some(max_top_dist) = self.search.max_top_dist() {
            println!("max: {}", out.max);
            len = out
                .rel_list
                .iter()
                .filter(|i| **i + max_top_dist >= out.max)
                .count();
        } else {
            len = out.p.total_pushed();
        }
        assert_eq!(out.p.total_pushed(), out.rel_list.len());

        let max_top_dist = self.search.max_top_dist().unwrap_or(0.0);
        let items: Vec<_> =
            page_from_pqueue_with_max_dist(limit, offset, max_top_dist, out.max, out.p)
                .into_iter()
                .map(|i| self.search.to_output_item(i.item))
                .collect();

        SearchResult::with_other_data(items, len, out.output_add)
    }

    pub fn guess(&self) -> Option<Guess> {
        let start = Instant::now();

        let mut counter =
            FilteredMaxCounter::<S::Item>::new(MAX_ESTIMATE + 1, |i| self.search.filter(i));

        // Keep track of real count to give `should_run` a correct value
        let mut c = 0;
        for prod in self.search.get_producer() {
            if !prod.should_run(c) {
                continue;
            }

            let old_counter = counter.val();
            prod.estimate_to(&mut counter);

            // Add たった今数えた量 to `c`
            c += counter.val() - old_counter;

            if counter.is_full() {
                break;
            }
        }

        let sum = counter.val();

        let gt;
        if sum > MAX_ESTIMATE {
            gt = GuessType::MoreThan;
        } else {
            gt = GuessType::Accurate;
        }

        debug!("Guessing took: {:?}", start.elapsed());
        Some(Guess::new(sum.min(MAX_ESTIMATE) as u32, gt))
    }
}
