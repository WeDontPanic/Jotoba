use crate::{
    pushable::{MaxCounter, PushMod, Pushable},
    relevance::item::RelItem,
    relevance::{data::SortData, RelevanceEngine},
    result::SearchResult,
    Engine,
};
use priority_container::StableUniquePrioContainerMax;
use std::marker::PhantomData;
use types::jotoba::{
    languages::Language,
    search::guess::{Guess, GuessType},
};

pub struct SearchTask<'index, E: Engine<'index>> {
    /// Search query
    query_str: String,

    /// Language to search in
    query_lang: Option<Language>,

    /// filter out items
    item_filter: Option<Box<dyn Fn(&E::Document) -> bool>>,

    /// Filter out results
    res_filter: Option<Box<dyn Fn(&E::Output) -> bool>>,

    /// Custom result order function
    cust_order: Option<
        Box<dyn RelevanceEngine<OutItem = E::Output, IndexItem = E::Document, Query = E::Query>>,
    >,

    /// Min relevance returned from search algo
    threshold: f32,
    limit: usize,
    offset: usize,
    est_limit: usize,
    phantom: PhantomData<E>,
}

impl<'index, E> SearchTask<'index, E>
where
    E: Engine<'index> + 'index,
{
    #[inline]
    pub fn new<S: AsRef<str>>(query: S) -> Self {
        let mut task = Self::default();
        task.query_str = query.as_ref().to_string();
        task
    }

    /// Creates a new Search task with a query assigned language
    #[inline]
    pub fn with_language<S: AsRef<str>>(query: S, language: Language) -> Self {
        let mut task = Self::default();
        task.query_str = query.as_ref().to_string();
        task.query_lang = Some(language);
        task
    }

    /// Returns `true` if the SearchTask has a language assigned
    #[inline]
    pub fn has_language(&self) -> bool {
        self.query_lang.is_some()
    }

    /// Set the total limit. This is the max amount of vectors which will be loaded and processed
    #[inline]
    pub fn with_limit(mut self, total_limit: usize) -> Self {
        self.limit = total_limit;
        self
    }

    /// Sets the search task's threshold. This does not apply on the final score, which can be
    /// overwritten by `order` but applies to the vector space relevance itself.
    #[inline]
    pub fn with_threshold(mut self, threshold: f32) -> Self {
        self.threshold = threshold;
        self
    }

    /// Returns `true` if there is a threshold set
    #[inline]
    pub fn has_threshold(&self) -> bool {
        self.threshold > 0.0
    }

    /// Sets the offeset of the search. Can be used for pagination. Requires output of search being
    /// directly used and not manually reordered
    pub fn with_offset(mut self, offset: usize) -> Self {
        self.offset = offset;
        self
    }

    /// Set the search task's result filter.
    pub fn with_result_filter<F: 'static>(mut self, res_filter: F) -> Self
    where
        F: Fn(&E::Output) -> bool,
    {
        self.res_filter = Some(Box::new(res_filter));
        self
    }

    /// Set the search task's custom order function
    pub fn with_custom_order(
        mut self,
        res_filter: impl RelevanceEngine<OutItem = E::Output, IndexItem = E::Document, Query = E::Query>
            + 'static,
    ) -> Self {
        self.cust_order = Some(Box::new(res_filter));
        self
    }

    /// Set the search task's raw document filter
    pub fn with_item_filter<F: 'static>(mut self, item_filter: F) -> Self
    where
        F: Fn(&E::Document) -> bool,
    {
        self.item_filter = Some(Box::new(item_filter));
        self
    }

    /// Runs the search task and returns the result.
    pub fn find(&mut self) -> SearchResult<E::Output> {
        let cap = self.limit + self.offset;
        let mut pqueue = StableUniquePrioContainerMax::new_allocated(cap, cap);
        self.find_to(&mut pqueue);
        self.make_result(pqueue)
    }

    /// Rettrieves results and pushes them into `out`
    #[inline]
    pub fn find_to<O>(&mut self, out: &mut O) -> Option<usize>
    where
        O: Pushable<Item = RelItem<E::Output>>,
    {
        self.find_to_inner(out, true)
    }

    /// Estimates the amount of results efficiently. This 'guess' is defined as follows:
    ///
    /// Be 'm' the amount of items a full search would return.
    /// Be 'n' the guess returned by this function.
    ///
    /// - n = 0 => m = 0
    /// - n <= m
    pub fn estimate_result_count(&mut self) -> Guess {
        let mut counter = MaxCounter::new(self.est_limit + 1);
        self.estimate_to(&mut counter);
        let estimated = counter.val();

        let mut guess_type = GuessType::Undefined;

        if (estimated <= self.est_limit) || estimated == 0 {
            // All filtering operations are applied in estimation algorithm as well.
            // Since we use the max value of query
            // result, we can only assure it being accurate if there was only one query and no
            // Limit was reached. From the 1st condition follows that estimated == 0 implies
            // an accurate results
            guess_type = GuessType::Accurate;
        } else if estimated > self.est_limit {
            // Were counting 1 more than `est_limit`. Thus `estimated` being bigger than limit
            // means there are more elements than the given limit. However since were returning a
            // number <= est_limit, relatively to the estimation the guess type is `Opentop`
            guess_type = GuessType::MoreThan;
        }

        let est_result = (estimated).min(self.est_limit) as u32;
        Guess::new(est_result, guess_type)
    }

    /// Estimates result count by pushing elements to `out`
    #[inline]
    pub fn estimate_to<P>(&mut self, out: &mut P)
    where
        P: Pushable<Item = E::Output>,
    {
        let mut out = PushMod::new(out, |i: RelItem<E::Output>| i.item);
        self.find_to_inner(&mut out, false);
    }

    /// Retrieves results and pushes all items into `out`. Calculates relevance for each item if `sort` is true or
    /// The SearchTask has a threshold set.
    fn find_to_inner<O>(&mut self, out: &mut O, sort: bool) -> Option<usize>
    where
        O: Pushable<Item = RelItem<E::Output>>,
    {
        let query = E::make_query(&self.query_str, self.query_lang)?;

        let mut retr: E::Retriever = E::retrieve_for(query.clone(), self.query_lang).get();

        let mut pushed = 0;

        loop {
            let (index_item, out_items) = match self.retrieve_next(&mut retr) {
                Some(v) => v,
                None => break,
            };

            for i in out_items {
                let score = if sort || self.has_threshold() {
                    self.score(&i, &index_item, &query)
                } else {
                    0.0
                };

                if self.has_threshold() && score < self.threshold {
                    continue;
                }

                // Break if caller doesn't want to consume more
                pushed += 1;
                if !out.push(RelItem::new(i, score)) {
                    break;
                }
            }
        }

        Some(pushed)
    }

    #[inline]
    fn score(&mut self, out_item: &E::Output, index_item: &E::Document, query: &E::Query) -> f32 {
        let s_data = SortData::new(
            out_item,
            index_item,
            0.0,
            query,
            &self.query_str,
            self.query_lang,
        );
        self.cust_order
            .as_mut()
            .map(|i| i.score(&s_data))
            .unwrap_or(0.0)
    }

    /// Builds output from the given Prio Queue
    fn make_result(
        &self,
        data: StableUniquePrioContainerMax<RelItem<E::Output>>,
    ) -> SearchResult<E::Output> {
        let total_count = data.total_pushed();
        let p_items = self.take_page(data);
        SearchResult::new(p_items, total_count)
    }

    /// Takes the correct page from a UniquePrioContainerMax based on the given offset and limit
    #[inline]
    fn take_page<U: Ord>(&self, pqueue: StableUniquePrioContainerMax<U>) -> Vec<U> {
        super::utils::page_from_pqueue(self.limit, self.offset, pqueue)
    }

    #[inline]
    fn retrieve_next(&self, retr: &mut E::Retriever) -> Option<(E::Document, Vec<E::Output>)> {
        let next = retr.next()?;

        if self.item_filter(&next) {
            return Some((next, vec![]));
        };

        let mut out_items = E::doc_to_output(&next).unwrap_or_default();
        if out_items.is_empty() {
            return Some((next, out_items));
        }

        if let Some(ref filter) = self.res_filter {
            out_items.retain(|i| filter(i));
        }

        Some((next, out_items))
    }

    /// Returns `false` if the item has to be removed from the result
    #[inline]
    fn item_filter(&self, item: &E::Document) -> bool {
        self.item_filter.as_ref().map(|i| i(item)).unwrap_or(true)
    }
}

impl<'a, T: Engine<'a>> Default for SearchTask<'a, T> {
    #[inline]
    fn default() -> Self {
        Self {
            query_str: Default::default(),
            query_lang: None,
            item_filter: None,
            res_filter: None,
            cust_order: None,
            threshold: 0.0,
            limit: 1000,
            offset: 0,
            est_limit: 100,
            phantom: PhantomData,
        }
    }
}
