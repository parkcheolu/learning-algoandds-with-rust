use std::sync::Mutex;

lazy_static::lazy_static! {
    static ref RG: Mutex<RandGen> = Mutex::new(RandGen::new(34052));
}

pub fn rand_val(max: usize) -> usize {
    RG.lock().unwrap().next_val(max)
}

pub struct RandGen {
    curr: usize,
    mul: usize,
    inc: usize,
    modulo: usize,
}

impl RandGen {
    pub fn new(curr: usize) -> Self {
        RandGen {
            curr,
            mul: 56394237,
            inc: 346423491,
            modulo: 23254544563,
        }
    }

    pub fn next_val(&mut self, max: usize) -> usize {
        self.curr = (self.curr * self.mul + self.inc) % self.modulo;
        self.curr % max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rands_print() {
        let mut r = RandGen::new(12);
        for _ in 0..100 {
            println!("--{}", r.next_val(100));
        }
    }
}