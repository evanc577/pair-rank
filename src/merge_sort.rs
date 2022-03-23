/// Sort slice via merge sort
pub fn merge_sort<T: Ord + Copy>(input: &mut [T]) {
    let length = input.len();

    // base case
    if length <= 1 {
        return;
    }

    // sort left and right subslices
    let (left, right) = input.split_at_mut(length / 2);
    merge_sort(left);
    merge_sort(right);

    // merge
    let mut merged = Vec::with_capacity(length);
    let mut left_iter = left.iter().peekable();
    let mut right_iter = right.iter().peekable();
    while let (Some(&x), Some(&y)) = (left_iter.peek(), right_iter.peek()) {
        if *x < *y {
            merged.push(*(left_iter.next().unwrap()));
        } else {
            merged.push(*(right_iter.next().unwrap()));
        }
    }
    for x in left_iter {
        merged.push(*x);
    }
    for y in right_iter {
        merged.push(*y);
    }

    input.copy_from_slice(&merged[..]);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn merge_sort_vec<T: Ord + Copy>(mut input: Vec<T>) -> Vec<T> {
        merge_sort(&mut input[..]);
        input
    }

    #[test]
    fn merge_sort_test() {
        assert_eq!(merge_sort_vec(vec![4, 3]), vec![3, 4]);
        assert_eq!(merge_sort_vec(vec![3]), vec![3]);
        assert_eq!(
            merge_sort_vec(vec![100, 90, 50, 14, 9, 7, 3]),
            vec![3, 7, 9, 14, 50, 90, 100]
        );
        assert_eq!(merge_sort_vec(vec![1, 2, 3, 4]), vec![1, 2, 3, 4]);
        assert_eq!(merge_sort_vec(vec!["a", "c", "b"]), vec!["a", "b", "c"]);
    }
}
