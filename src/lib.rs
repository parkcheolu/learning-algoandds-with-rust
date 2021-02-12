mod algo;
use algo::bubble_sort;
//use algo::sort::bubble_sort;

#[cfg(test)]
mod tests {
    use super::bubble_sort;

    #[test]
    fn test_bubble_sort() {
        let mut v = vec![7, 4, 11, 34, 2, 6, 78, 1];
        bubble_sort(&mut v);
        assert_eq!(v, [1, 2, 4, 6, 7, 11, 34, 78]);
    }
}