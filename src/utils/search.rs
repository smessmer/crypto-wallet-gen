use anyhow::Result;
use futures::future::{self, LocalBoxFuture};

/// Search an index space 0.. by calling `query` on each index
/// until `stop_after_n_misses` consecutive queries returned [None].
/// Return an iterator over the result of all queries that didn't return [None].
/// Queries are run concurrently using async/await.
pub async fn search<'a, T>(
    stop_after_n_misses: u32,
    query: impl Fn(u32) -> LocalBoxFuture<'a, Result<Option<T>>>,
) -> Result<impl Iterator<Item = T>> {
    let mut first_unchecked = 0;
    let mut last_found_plus_one = 0;
    let mut all_results = vec![];

    while first_unchecked < last_found_plus_one + stop_after_n_misses {
        let end = last_found_plus_one + stop_after_n_misses;
        let range = first_unchecked..end;
        let range_results: Vec<(u32, T)> = future::try_join_all(range.map(&query))
            .await?
            .into_iter()
            .enumerate()
            .filter_map(|(i, v)| v.map(|v| (i as u32 + first_unchecked, v)))
            .collect();
        first_unchecked = end;
        if let Some(last) = range_results.last() {
            last_found_plus_one = last.0 + 1;
        }
        all_results.push(range_results.into_iter().map(|(_, v)| v));
    }

    Ok(all_results.into_iter().flatten())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;
    use std::sync::Arc;
    use std::sync::Mutex;

    #[derive(Debug, PartialEq, Eq)]
    struct Wrapper(u32);

    #[tokio::test]
    async fn search_no_results() {
        let r = search(10, |_| Box::pin(future::ready(Ok(Option::<Wrapper>::None))))
            .await
            .unwrap();
        assert_eq!(0, r.count());
    }

    #[tokio::test]
    async fn search_no_attempts() {
        let r = search(0, |i| Box::pin(future::ready(Ok(Some(Wrapper(i))))))
            .await
            .unwrap();
        assert_eq!(0, r.count());
    }

    #[tokio::test]
    async fn search_find_in_initial_run() {
        let r = search(10, |i| {
            let res = if i < 8 { Some(Wrapper(i)) } else { None };
            Box::pin(future::ready(Ok(res)))
        })
        .await
        .unwrap();
        assert_eq!(
            (0..8).map(|i| Wrapper(i)).collect::<Vec<_>>(),
            r.collect::<Vec<_>>(),
        );
    }

    #[tokio::test]
    async fn search_find_in_later_run() {
        let r = search(10, |i| {
            let res = if i < 55 { Some(Wrapper(i)) } else { None };
            Box::pin(future::ready(Ok(res)))
        })
        .await
        .unwrap();
        assert_eq!(
            (0..55).map(|i| Wrapper(i)).collect::<Vec<_>>(),
            r.collect::<Vec<_>>(),
        );
    }

    #[tokio::test]
    async fn checks_correct_indices_and_only_once() {
        let checked_indices = Arc::new(Mutex::new(HashSet::new()));
        let _ = search(10, |i| {
            let mut checked_indices = checked_indices.lock().unwrap();
            assert!(
                !checked_indices.contains(&i),
                "Checked index {} multiple times",
                i
            );
            checked_indices.insert(i);
            let res = if i < 55 { Some(Wrapper(i)) } else { None };
            Box::pin(future::ready(Ok(res)))
        })
        .await
        .unwrap();
        assert_eq!(
            (0..65).collect::<HashSet<_>>(),
            *checked_indices.lock().unwrap(),
        );
    }
}
