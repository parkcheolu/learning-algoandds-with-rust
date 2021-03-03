#[derive(Debug)]
pub struct BinTree<T>(Option<Box<BinData<T>>>);
#[derive(Debug)]
pub struct BinData<T> {
    data: T,
    left: BinTree<T>,
    right: BinTree<T>,
}

impl<T> BinTree<T> {
    pub fn new() -> Self {
        BinTree(None)
    }
}

impl<T:PartialOrd> BinTree<T> {
    pub fn add_sorted(&mut self, data: T) {
        match self.0 {
            Some(ref mut bin_data) => {
                if data < bin_data.data {
                    bin_data.left.add_sorted(data);
                } else {
                    bin_data.right.add_sorted(data);
                }
            }
            None => {
                self.0 = Some(Box::new(BinData {
                    data,
                    left: BinTree::new(),
                    right: BinTree::new(),
                }))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut bt = BinTree::new();
        bt.add_sorted(10);
        bt.add_sorted(13);
        bt.add_sorted(11);
        bt.add_sorted(5);
        bt.add_sorted(4);
        bt.add_sorted(9);
        println!("{:?}", bt);
    }
}