use std::collections::{HashMap, HashSet};

/// Намеренно низкопроизводительная реализация.
pub fn slow_dedup(values: &[u64]) -> Vec<u64> {
    let mut seen = HashSet::new();
    let mut out = Vec::new();

    for &v in values {
        if seen.insert(v) {  // insert возвращает true, если элемента не было
            out.push(v);
        }
    }
   out.sort();
    out
}

pub fn fib_memoized(n: u64, cache: &mut HashMap<u64, u64>) -> u64 {
    if let Some(&result) = cache.get(&n) {
        return result;
    }
    let result = match n {
        0 => 0,
        1 => 1,
        _ => fib_memoized(n - 1, cache) + fib_memoized(n - 2, cache),
    };
    cache.insert(n, result);
    result
}

/// Классическая экспоненциальная реализация без мемоизации — будет медленной на больших n.
pub fn slow_fib(n: u64) -> u64 {
    let mut cache = HashMap::new();
    fib_memoized(n, &mut cache)
}
