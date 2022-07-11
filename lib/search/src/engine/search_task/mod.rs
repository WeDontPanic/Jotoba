pub mod cpushable;
pub mod pushable;
pub mod sort_item;

use self::{
    cpushable::{CPushable, MaxCounter},
    pushable::Pushable,
};

use super::{result::SearchResult, result_item::ResultItem, SearchEngine};
use error::Error;
use priority_container::StableUniquePrioContainerMax;
use sort_item::SortItem;
use std::{collections::HashSet, marker::PhantomData};
use types::jotoba::{
    languages::Language,
    search::guess::{Guess, GuessType},
};
use vector_space_model2::{term_store::TermIndexer, DocumentVector, Index, Vector};

pub struct SearchTask<T: SearchEngine> {
    /// Search query
    query_str: String,
    /// Language of query
    query_lang: Option<Language>,
    /// filter out vectors
    vec_filter: Option<Box<dyn Fn(&DocumentVector<T::Document>, &Vector) -> bool>>,
    /// Filter out results
    res_filter: Option<Box<dyn Fn(&T::Output) -> bool>>,
    /// Custom result order function
    cust_order: Option<Box<dyn Fn(SortItem<T::Output>) -> usize>>,
    /// Min relevance returned from vector space algo
    threshold: usize,
    vector_limit: usize,
    limit: usize,
    offset: usize,
    allow_align: bool,
    est_limit: usize,
    score_multiplier: f32,
    phantom: PhantomData<T>,
}

impl<T: SearchEngine> SearchTask<T> {
    /// Creates a new Search task
    #[inline]
    pub fn new<S: AsRef<str>>(query: S) -> Self {
        let mut task = Self::default();
        task.query_str = query.as_ref().to_string();
        task
    }

    /// Creates a new Search task with a query assigned language
    pub fn with_language<S: AsRef<str>>(query: S, language: Language) -> Self {
        let mut task = Self::default();
        task.query_str = query.as_ref().to_string();
        task.query_lang = Some(language);
        task
    }

    /// Set the total limit. This is the max amount of vectors which will be loaded and processed
    pub fn limit(mut self, total_limit: usize) -> Self {
        self.limit = total_limit;
        self
    }

    /// Sets the search task's threshold. This does not apply on the final score, which can be
    /// overwritten by `order` but applies to the vector space relevance itself.
    pub fn threshold(mut self, threshold: usize) -> Self {
        self.threshold = threshold;
        self
    }

    /// Sets the search task's threshold. This does not apply on the final score, which can be
    /// overwritten by `order` but applies to the vector space relevance itself.
    pub fn set_align(&mut self, align: bool) {
        self.allow_align = align;
    }

    /// Sets the offeset of the search. Can be used for pagination. Requires output of search being
    /// directly used and not manually reordered
    pub fn offset(mut self, offset: usize) -> Self {
        self.offset = offset;
        self
    }

    /// Adds a custom score multiplier to the task
    pub fn with_score_multiplier(mut self, m: f32) -> Self {
        self.score_multiplier = m;
        self
    }

    /// Set the search task's vector filter.
    pub fn set_vector_filter<F: 'static>(&mut self, vec_filter: F)
    where
        F: Fn(&DocumentVector<T::Document>, &Vector) -> bool,
    {
        self.vec_filter = Some(Box::new(vec_filter));
    }

    /// Set the search task's result filter.
    pub fn set_result_filter<F: 'static>(&mut self, res_filter: F)
    where
        F: Fn(&T::Output) -> bool,
    {
        self.res_filter = Some(Box::new(res_filter));
    }

    /// Set the search task's custom order function
    pub fn with_custom_order<F: 'static>(&mut self, res_filter: F)
    where
        F: Fn(SortItem<T::Output>) -> usize,
    {
        self.cust_order = Some(Box::new(res_filter));
    }

    fn gen_query_vec(&self) -> Option<Vector> {
        let index = self.get_index();
        let (vec, _) = T::gen_query_vector(index, &self.query_str, false, self.query_lang)?;
        Some(vec)
    }

    /// Returns `true` if the search task's query is a term in the corresponding index
    #[inline]
    pub fn has_term(&self) -> bool {
        let q_fmt = T::query_formatted(&self.query_str);
        self.get_index().get_indexer().find_term(&q_fmt).is_some()
    }

    #[inline]
    pub fn get_index(&self) -> &'static Index<T::Document, T::Metadata> {
        T::get_index(self.query_lang).expect("Lang not loaded")
    }

    pub fn get_indexer(language: Option<Language>) -> Option<&'static TermIndexer> {
        Some(T::get_index(language)?.get_indexer())
    }

    pub fn find_exact(&self) -> SearchResult<T::Output> {
        let index = self.get_index();
        let query_vec = match index.build_vector(&[&self.query_str], None) {
            Some(qv) => qv,
            None => return SearchResult::default(),
        };

        let mut out = StableUniquePrioContainerMax::new(self.offset + self.limit);
        self.find_by_vec(query_vec, &mut out);
        self.make_result(out)
    }

    /// Runs the search task and writes all items into the priority queue
    pub fn find_to<I: Pushable<Item = ResultItem<T::Output>>>(&self, out: &mut I) {
        if let Some(qvec) = self.gen_query_vec() {
            self.find_by_vec(qvec, out);
        }
    }

    /// Runs the search task and returns the result.
    pub fn find(&self) -> SearchResult<T::Output> {
        let cap = self.limit + self.offset;
        let mut pqueue = StableUniquePrioContainerMax::new_allocated(cap, cap);
        self.find_to(&mut pqueue);
        self.make_result(pqueue)
    }

    /// Builds output from the given Prio Queue
    fn make_result(
        &self,
        data: StableUniquePrioContainerMax<ResultItem<T::Output>>,
    ) -> SearchResult<T::Output> {
        let total_count = data.total_pushed();
        let p_items = self.take_page(data);
        SearchResult::new(p_items, total_count)
    }

    /// Takes the correct page from a UniquePrioContainerMax based on the given offset and limit
    #[inline]
    fn take_page<U: Ord>(&self, pqueue: StableUniquePrioContainerMax<U>) -> Vec<U> {
        super::utils::page_from_pqueue(self.limit, self.offset, pqueue)
    }

    fn find_by_vec<I: Pushable<Item = ResultItem<T::Output>>>(&self, q_vec: Vector, out: &mut I) {
        // Retrieve all document vectors that share at least one dimension with the query vector
        let vecs = self
            .get_index()
            .get_vector_store()
            .get_for_vec(&q_vec)
            .take(self.vector_limit);

        self.load_documents_to(vecs, &q_vec, out);
    }

    fn load_documents_to<P, I>(&self, dvec_iter: I, q_vec: &Vector, out: &mut P)
    where
        P: Pushable<Item = ResultItem<T::Output>>,
        I: Iterator<Item = DocumentVector<T::Document>>,
    {
        for dvec in dvec_iter {
            if !self.filter_vector(&dvec, &q_vec) {
                continue;
            }

            for res_doc in T::doc_to_output(&dvec.document).unwrap_or_default() {
                if !self.filter_result(&res_doc) {
                    continue;
                }

                let sort_item = SortItem::new(
                    &res_doc,
                    0.0,
                    &self.query_str,
                    self.query_lang,
                    q_vec,
                    dvec.vector(),
                );

                let score = self.calc_score(sort_item);
                if score < self.threshold as usize {
                    continue;
                }

                out.push(ResultItem::new_raw(res_doc, score, self.query_lang));
            }
        }
    }

    /// Estimates the amount of results efficiently. This 'guess' is defined as follows:
    ///
    /// Be 'm' the amount of items a full search would return.
    /// Be 'n' the guess returned by this function.
    ///
    /// - n = 0 => m = 0
    /// - n <= m
    pub fn estimate_result_count(&self) -> Result<Guess, Error> {
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
        Ok(Guess::new(est_result, guess_type))
    }

    #[inline]
    pub fn estimate_to<P>(&self, out: &mut P)
    where
        P: CPushable<Item = T::Output>,
    {
        if let Some(vec) = self.gen_query_vec() {
            self.estimate_by_vec_to(vec, out);
        }
    }

    fn estimate_by_vec_to<P>(&self, q_vec: Vector, out: &mut P)
    where
        P: CPushable<Item = T::Output>,
    {
        let vec_store = self.get_index().get_vector_store();
        let query_dimensions: Vec<_> = q_vec.vec_indices().collect();

        // Retrieve all document vectors that share at least one dimension with the query vector
        let document_vectors = vec_store
            .get_all_iter(&query_dimensions)
            .take(self.vector_limit);

        let mut unique = HashSet::with_capacity(50);

        'o: for dvec in document_vectors {
            if !self.filter_vector(&dvec, &q_vec) {
                continue;
            }

            for res_doc in T::doc_to_output(&dvec.document).unwrap_or_default() {
                if unique.contains(&res_doc) || !self.filter_result(&res_doc) {
                    continue;
                }

                // Don't ignore threshold if set
                if self.threshold > 0 {
                    let sort_item = SortItem::new(
                        &res_doc,
                        0.0,
                        &self.query_str,
                        self.query_lang,
                        &q_vec,
                        dvec.vector(),
                    );

                    let score = self.calc_score(sort_item);
                    if score < self.threshold as usize {
                        continue;
                    }
                }

                unique.insert(res_doc.clone());

                let can_push_more = out.push(res_doc);
                if !can_push_more {
                    break 'o;
                }
            }
        }
    }

    /// Calculates the score using a custom function if provided or just `rel` otherwise
    #[inline]
    fn calc_score(&self, si: SortItem<T::Output>) -> usize {
        match self.cust_order.as_ref() {
            Some(cust_sort) => (cust_sort(si) as f32 * self.score_multiplier) as usize,
            None => (T::score(si) as f32 * self.score_multiplier) as usize,
        }
    }

    #[inline]
    fn filter_result(&self, output: &T::Output) -> bool {
        self.res_filter.as_ref().map(|i| i(output)).unwrap_or(true)
    }

    #[inline]
    fn filter_vector(&self, vec: &DocumentVector<T::Document>, query_vec: &Vector) -> bool {
        self.vec_filter
            .as_ref()
            .map(|i| i(vec, query_vec))
            .unwrap_or(true)
    }
}

impl<T: SearchEngine> Default for SearchTask<T> {
    #[inline]
    fn default() -> Self {
        Self {
            query_str: String::default(),
            query_lang: None,
            vec_filter: None,
            res_filter: None,
            threshold: 0,
            limit: 1000,
            est_limit: 100,
            vector_limit: 100_000,
            offset: 0,
            allow_align: true,
            phantom: PhantomData,
            cust_order: None,
            score_multiplier: 1.0,
        }
    }
}
