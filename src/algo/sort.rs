use std::fmt::Debug;

use crate::rand;


// O(n^2)
#[allow(dead_code)]
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

#[allow(dead_code)]
pub fn merge_sort<T: PartialOrd + Debug>(mut v: Vec<T>) -> Vec<T> {
    println!("{:?}", v);
    if v.len() <= 1 {
        return v;
    }
    // 재귀호출의 결과를 저장할 vec. merge sort는 이렇게 또다른 공간 할당을 
    // 요구하기에, 공간 점유율이 상대적으로 클 수 있다(현실적으로 우려할만한 
    // 정도는 아니다).
    let mut res = Vec::with_capacity(v.len());
    let b = v.split_off(v.len() / 2);
    let a = merge_sort(v);
    let b = merge_sort(b);

    let mut a_it = a.into_iter();
    let mut b_it = b.into_iter();
    let mut a_peek = a_it.next();
    let mut b_peek = b_it.next();

    loop {
        match a_peek {
            Some(ref a_val) => match b_peek {
                Some(ref b_val) => {
                    // 오름차순 정렬이므로 큰 값을 뒤로 보내야한다. 작은 값을 먼저 push한다.
                    if a_val > b_val {
                        // Option.take()는 self를 자신의 inner value를 반환하고 자신을 None으로 바꾼다.
                        // 여기서 *_peek.take()를 호출하지 않으면 res.push에서 copy가 발생하고, 이렇게 되면
                        // 컴파일러가 이미 정렬이 완료된 값임을 알지 못한다. take를 사용해서 값을 취하고 그 
                        // 자리에 None을 둠으로써 정렬된 값의 완료를 표시한다.                        
                        res.push(b_peek.take().unwrap());
                        // 루프의 다음 회차 실행을 위해 b_peek에 next value를 세팅한다.
                        b_peek = b_it.next();
                    } else {
                        res.push(a_peek.take().unwrap());
                        a_peek = a_it.next();                        
                    }
                },
                None => {
                    // 여기에 도달했음은, 위의 값 비교에서 b가 더 컸음을 의미한다(take를 통해 None이 되었으므로).
                    // b의 값은 이미 push했으니, 여기서는 남아있는 a를 push한다.
                    res.push(a_peek.take().unwrap());
                    // 원소가 2개 이상인 a_it은 처음부터 정렬된 상태이므로, a에 남아있는 값은 어차피 위에서 push한
                    // a_peek의 값보다 크다. 때문에 남은 값은 뒤에 그대로 이어붙인다.
                    res.extend(a_it);
                    return res;
                },
            }
            None => {
                if let Some(b_val) = b_peek {
                    res.push(b_val);
                }
                res.extend(b_it);
                return res;            
            },
        }
    }
}

/// 피봇값을 고르고, 피봇값보다 작은 값은 왼쪽으로, 큰 값은 오른쪽으로 위치를 이동시키고,
/// 피봇값 위치를 반환한다.
#[allow(dead_code)]
pub fn pivot<T: PartialOrd>(v: &mut [T]) -> usize {
    let mut p = rand::rand_val(v.len());
    v.swap(p, 0);
    p = 0;
    for i in 1..v.len() {
        if v[i] < v[p] {
            v.swap(p + 1, i);
            v.swap(p, p + 1);
            p += 1;
        }
    }    
    p
}

/// 피봇값 위치를 기준으로 둘로 쪼개고, 쪼개진 둘에 대해 같은 연산을 반복한다.
#[allow(dead_code)]
pub fn quick_sort<T: PartialOrd + Debug>(v: &mut [T]) {
    if v.len() <= 1 {
        return;
    }
    let p = pivot(v);
    println!("pivot={}", p);
    println!("{:?}", v);

    let (a, b) = v.split_at_mut(p);
    quick_sort(a);
    quick_sort(&mut b[1..]);
}

/// Send trait을 unsafe로 구현하기 위한 struct.
#[allow(dead_code)]
struct RawSend<T>(*mut [T]);

/// unsafe Send 구현.
unsafe impl<T> Send for RawSend<T> {}

#[allow(dead_code)]
pub fn threaded_quick_sort<T: 'static + PartialOrd + Debug>(v: &mut [T]) {
    if v.len() <= 1 {
        return;
    }
    let p = pivot(v);
    println!("pivot={}", p);
    println!("{:?}", v);

    let (a, b) = v.split_at_mut(p);

    // raw pointer 생성.
    let raw_a: *mut [T] = a as *mut [T];
    // 클로저에 넘겨지는 변수의 Send trait 구현 요건을 만족시키기 위해 RawSend로 wrapping.
    let raw_s = RawSend(raw_a);

    unsafe {
        let handle = std::thread::spawn(move || {
            // raw pointer 역참조.
            threaded_quick_sort(&mut *raw_s.0);
        });

        threaded_quick_sort(&mut b[1..]);
        
        handle.join().ok();
    }
}

/// rayon의 work-stealing을 활용한다.
#[allow(dead_code)]
pub fn quick_sort_rayon<T: PartialOrd + Debug + Send>(v: &mut [T]) {
    if v.len() <= 1 {
        return;
    }
    let p = pivot(v);
    println!("pivot={}", p);
    println!("{:?}", v);

    let (a, b) = v.split_at_mut(p);
    // 두 번째 함수를 큐에 넣고 첫 번째 함수를 실행한다.
    // 다른 쓰레드가 준비가 되면 두 번째 함수를 실행한다(work-stealing).
    rayon::join(|| quick_sort_rayon(a), || quick_sort_rayon(&mut b[1..]));
}