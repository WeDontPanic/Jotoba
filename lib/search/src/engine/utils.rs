use priority_container::unique::UniquePrioContainerMax;

/// Takes the correct "limit" elements form a from a UniquePrioContainerMax at "offset"
pub fn page_from_pqueue<U: Ord>(
    limit: usize,
    offset: usize,
    pqueue: UniquePrioContainerMax<U>,
) -> Vec<U> {
    let len = pqueue.len();

    let take = (len.saturating_sub(offset)).min(limit);
    let to_skip = len.saturating_sub(offset + take);

    let mut o: Vec<_> = pqueue.into_iter().skip(to_skip).take(take).collect();
    o.reverse();
    o
}
