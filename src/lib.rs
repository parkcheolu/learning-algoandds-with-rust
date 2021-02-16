mod algo;
mod rand;
mod ds;

#[allow(unused_imports)]
use algo::*;
#[allow(unused_imports)]
use rand::*;
use ds::ll::*;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bubble_sort() {
        let mut v = vec![7, 4, 11, 34, 2, 6, 78, 1];
        bubble_sort(&mut v);
        assert_eq!(v, [1, 2, 4, 6, 7, 11, 34, 78]);
    }

    #[test]
    fn test_merge_sort() {
        let v = vec![7, 4, 11, 34, 2, 6, 78, 1];
        let v = merge_sort(v);
        assert_eq!(v, [1, 2, 4, 6, 7, 11, 34, 78]);
    }

    #[test]
    fn test_pivot() {
        let mut v = vec![7, 4, 11, 34, 2, 6, 78, 1];
        let _p = pivot(&mut v);
        for x in 0..v.len() {
            assert!((v[x] < v[_p]) == (x < _p));
        }
    }

    #[test]
    fn test_quick_sort() {
        let mut v = vec![7, 4, 11, 34, 2, 6, 78, 1];
        quick_sort(&mut v);
        assert_eq!(v, [1, 2, 4, 6, 7, 11, 34, 78]);
    }

    #[test]
    fn test_threaded_quick_sort() {
        let mut v = vec![7, 4, 11, 34, 2, 6, 78, 1];
        threaded_quick_sort(&mut v);
        assert_eq!(v, [1, 2, 4, 6, 7, 11, 34, 78]);
    }

}