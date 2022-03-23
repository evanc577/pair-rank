use std::{cmp::Ordering, collections::HashMap, hash::Hash};

/// Sort slice via merge-insertion sort for good worst-case complexity.
/// Adapted from [ford-johnson crate](https://crates.io/crates/ford-johnson)
pub fn merge_insertion_sort<T>(xs: &mut [T])
where
    T: Ord + Copy + Hash,
{
    if xs.len() < 2 {
        return;
    }

    // First, swap all the largest elements to the front.
    let mut partner: HashMap<T, Vec<T>> = HashMap::new();
    let half = xs.len() / 2;
    for i in 0..half {
        if T::cmp(&xs[i], &xs[i + half]) == Ordering::Less {
            xs.swap(i, i + half);
        }
        partner.entry(xs[i]).or_default().push(xs[i + half]);
    }

    // Now recursively sort those larger elements.
    merge_insertion_sort(&mut xs[..half]);

    // Now do an insertion-sort to get the latter half of the array into order.
    for i in 0..half {
        // Every step of the way we'll be inserting an extra element,
        // so `x[i]` will be located at `xs[2*i]`.
        let y = partner.get_mut(&xs[2 * i]).unwrap().pop().unwrap();
        // We known that y[i] < x[i], so we need to insert it to the left of x[i].
        let idx = find_insert_point(y, &xs[..2 * i]);
        // Make room.
        xs[idx..half + i + 1].rotate_right(1);
        // Insert it.
        xs[idx] = y;
    }
    if xs.len() % 2 > 0 {
        let i = xs.len() - 1;
        let idx = find_insert_point(xs[i], &xs[..i]);
        xs[idx..].rotate_right(1);
    }
}

fn find_insert_point<T>(x: T, xs: &[T]) -> usize
where
    T: Ord,
{
    let mut lo = 0;
    let mut hi = xs.len();
    while hi > lo {
        let mid = lo + (hi - lo) / 2;
        match T::cmp(&x, &xs[mid]) {
            Ordering::Equal => return mid,
            Ordering::Less => hi = mid,
            Ordering::Greater => lo = mid + 1,
        };
    }
    lo
}

#[cfg(test)]
mod tests {
    use super::*;

    fn merge_insertion_sort_vec<T: Ord + Copy + Hash>(mut input: Vec<T>) -> Vec<T> {
        merge_insertion_sort(&mut input[..]);
        input
    }

    #[test]
    fn merge_insertion_sort_test() {
        assert_eq!(merge_insertion_sort_vec(vec![4, 3]), vec![3, 4]);
        assert_eq!(merge_insertion_sort_vec(vec![3]), vec![3]);
        assert_eq!(
            merge_insertion_sort_vec(vec![100, 90, 50, 14, 9, 7, 3]),
            vec![3, 7, 9, 14, 50, 90, 100]
        );
        assert_eq!(merge_insertion_sort_vec(vec![1, 2, 3, 4]), vec![1, 2, 3, 4]);
        assert_eq!(
            merge_insertion_sort_vec(vec!["a", "c", "b"]),
            vec!["a", "b", "c"]
        );
    }
}
