pub mod out_builder;
pub mod producer;
pub mod searchable;

use crate::result::SearchResult;
use out_builder::OutputBuilder;
use searchable::Searchable;
use types::jotoba::search::guess::{Guess, GuessType};

/// Executes a search
pub struct SearchExecutor<S: Searchable> {
    search: S,
}

impl<S: Searchable> SearchExecutor<S> {
    /// Creates a new SearchExecutor
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
            prod.produce(&mut out);
        }

        self.search.mod_output(&mut out);

        let len = out.p.total_pushed();
        let items: Vec<_> = crate::engine::utils::page_from_pqueue(limit, offset, out.p)
            .into_iter()
            .map(|i| self.search.to_output_item(i.item))
            .collect();
        SearchResult::with_other_data(items, len, out.output_add)
    }

    pub fn guess(&self) -> Option<Guess> {
        let mut sum = 0usize;
        let mut gt = GuessType::Accurate;

        for prod in self.search.get_producer() {
            if !prod.should_run(sum) {
                continue;
            }

            if let Some(guess) = prod.estimate() {
                sum += guess.value as usize;
                if guess.guess_type == GuessType::MoreThan {
                    gt = GuessType::MoreThan;
                }
            }
        }

        let sum = sum.min(100);
        Some(Guess::new(sum as u32, gt))
    }
}
