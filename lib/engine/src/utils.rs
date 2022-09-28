use crate::relevance::item::RelItem;
use priority_container::StableUniquePrioContainerMax;

/// Takes the correct "limit" elements form a from a UniquePrioContainerMax at "offset"
pub fn page_from_pqueue<U: Ord>(
    limit: usize,
    offset: usize,
    pqueue: StableUniquePrioContainerMax<U>,
) -> Vec<U> {
    let len = pqueue.len();

    let take = (len.saturating_sub(offset)).min(limit);
    let to_skip = len.saturating_sub(offset + take);

    let mut o: Vec<_> = pqueue.into_iter().skip(to_skip).take(take).collect();
    o.reverse();
    o
}

/// Takes the correct "limit" elements form a from a UniquePrioContainerMax at "offset"
pub fn page_from_pqueue_with_max_dist<I: PartialEq>(
    limit: usize,
    offset: usize,
    max_dist: f32,
    max: f32,
    pqueue: StableUniquePrioContainerMax<RelItem<I>>,
) -> Vec<RelItem<I>> {
    let peeked = pqueue.peek();

    if peeked.is_none() {
        return vec![];
    }

    let len = pqueue.len();

    let take = (len.saturating_sub(offset)).min(limit);
    let to_skip = len.saturating_sub(offset + take);

    let mut o: Vec<_> = pqueue
        .into_iter()
        .filter(|i| i.relevance + max_dist >= max || max_dist == 0.0)
        .skip(to_skip)
        .take(take)
        .collect();
    o.reverse();
    o
}
