use std::fmt::Debug;
#[derive(Debug)]
pub struct BinTree<T>(Option<Box<BinData<T>>>);
#[derive(Debug)]
pub struct BinData<T> {
    data: T,
    h: i8,
    left: BinTree<T>,
    right: BinTree<T>,
}

impl<T> BinData<T> {
    pub fn rot_left(mut self) -> Box<Self> {
        // 반환값은 right node.
        let mut res = match self.right.0.take() {
            Some(res) => res,
            None => return Box::new(self),
        };

        // right node의 left를 시작 node의 right로 이동.
        self.right = BinTree(res.left.0.take());
        self.right.set_height();
        res.left = BinTree(Some(Box::new(self)));
        res.left.set_height();
        res.h = 1 + std::cmp::max(res.left.height(), res.right.height());
        res
    }

    pub fn rot_right(mut self) -> Box<Self> {
        let mut res = match self.left.0.take() {
            Some(res) => res,
            None => return Box::new(self),
        };

        self.left = BinTree(res.right.0.take());
        self.left.set_height();
        res.right = BinTree(Some(Box::new(self)));
        res.right.set_height();
        res.h = 1 + std::cmp::max(res.left.height(), res.right.height());
        res
    }
}

impl<T> BinTree<T> {
    pub fn new() -> Self {
        BinTree(None)
    }

    pub fn height(&self) -> i8 {
        match self.0.as_ref() {
            Some(t) => t.h,
            None => 0,
        }
    }

    pub fn set_height(&mut self) {
        if let Some(t) = self.0.as_deref_mut() {
            t.h = 1 + std::cmp::max(t.left.height(), t.right.height());
        }
    }

    pub fn rot_left(&mut self) {
        self.0 = self.0.take().map(|v| v.rot_left());
    }

    pub fn rot_right(&mut self) {
        self.0 = self.0.take().map(|v| v.rot_right());
    }
}

impl<T:PartialOrd> BinTree<T> {
    pub fn add_sorted(&mut self, data: T) {
        let rot_dir = match self.0 {
            Some(ref mut bin_data) => {
                if data < bin_data.data {
                    bin_data.left.add_sorted(data);
                    if bin_data.left.height() - bin_data.right.height() > 1 {
                        1
                    } else {
                        0
                    }
                } else {
                    bin_data.right.add_sorted(data);
                    if bin_data.right.height() - bin_data.left.height() > 1 {
                        -1
                    } else {
                        0
                    }
                }
            }
            None => {
                self.0 = Some(Box::new(BinData {
                    data,
                    h: 0,
                    left: BinTree::new(),
                    right: BinTree::new(),
                }));
                0
            }
        };

        match rot_dir {
            1 => self.rot_right(),
            -1 => self.rot_left(),
            _ => self.set_height(),
        }
    }
}

impl<T: Debug> BinTree<T> {
    pub fn print_lfirst(&self, dp: i32) {
        if let Some(bd) = self.0.as_ref() {
            bd.left.print_lfirst(dp + 1);
            let mut spc = String::new();
            for _ in 0..dp {
                spc.push('.');
            }
            println!("{}:{}{:?}", bd.h, spc, bd.data);
            bd.right.print_lfirst(dp + 1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut bt = BinTree::new();
        bt.add_sorted(4);
        bt.add_sorted(5);
        bt.add_sorted(6);
        bt.add_sorted(10);
        bt.add_sorted(1);
        bt.add_sorted(94);
        bt.add_sorted(54);
        bt.add_sorted(3);
        bt.print_lfirst(0);

        println!("-------------------------------------------");

        bt.rot_left();
        bt.print_lfirst(0);
    }
}