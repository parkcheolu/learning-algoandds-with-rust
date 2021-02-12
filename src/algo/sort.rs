use std::fmt::Debug;


// O(n^2)
pub fn bubble_sort<T: PartialOrd + Debug>(v: &mut [T]) {
    for p in 0..v.len() {
        println!("{:?}", v);
        // 더이상 정렬이 필요하지 않은 상황에 반드시 원소의 끝까지 확인할 필요는 없다.
        // 정렬이 다 된 상태라면 한 회차를 도는 동안 swap이 한 번도 발생하지 않음에 착안한 최적화이다.
        // 한 번이라도 swap이 발생하면 아직 정렬이 완료되지 않았을 수 있음을 의미한다.
        let mut sorted = true;
        // 뒤의 '- p'는 불필요한 비교를 피하기 위한 최적화이다.
        // 버블 정렬의 첫 번째 순회에서 가장 큰 값은 맨 뒤로 이동한다.
        // 때문에 두 번째 순회에서 마지막 값을 비교할 필요는 없다.
        // 두 번째 순회에서는 두 번째로 큰 값이 뒤에서 두 번재로 이동한다.
        // 세 번째 순회에서는 마지막 두 값을 비교할 필요가 없다.
        for i in 0..(v.len() - 1) - p  {
            if v[i] > v[i + 1] {
                v.swap(i, i + 1);
                sorted = false;
            }
        }
        if sorted {
            return;
        }
    }
}

pub fn merge_sort<T: PartialOrd + Debug>(v: &mut [T]) {
    
}

