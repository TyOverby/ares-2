
pub fn iterators_same<T, A, B, F>(mut iter1: A, mut iter2: B, eq_fn: F) -> bool
where A: Iterator<Item=T>, B: Iterator<Item=T>, F: Fn(T, T) -> bool {
    loop {
        match (iter1.next(), iter2.next()) {
            (None, None) => return true,
            (Some(_), None) |
            (None, Some(_)) => return false,
            (Some(a), Some(b)) => {
                if !eq_fn(a, b) {
                    return false;
                }
            }
        }
    }
}
