mod algo;
//use algo::sort::bubble_sort;

#[cfg(test)]
mod tests {
    use super::algo::sort::bubble_sort;

    #[test]
    fn test_bubble_sort() {
        let mut v = vec![1, 2, 3];
        bubble_sort(&mut v);
        assert_eq!(2 + 2, 4);
    }
}