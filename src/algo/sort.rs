
pub fn bubble_sort<T: PartialOrd + std::fmt::Debug>(v: &mut [T]) {
    for i in 0..v.len() {
        println!("{:?}", i);
    }
}