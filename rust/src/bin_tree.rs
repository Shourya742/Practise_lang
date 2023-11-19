#[cfg(test)]
mod test {

    use std::fmt::Debug;

    #[derive(Debug)]
    struct BinTree<T>(Option<Box<BinData<T>>>);

    #[derive(Debug)]
    struct BinData<T> {
        data: T,
        left: BinTree<T>,
        right: BinTree<T>,
    }

    impl<T> BinTree<T> {
        fn new() -> Self {
            BinTree(None)
        }
    }

    impl<T: PartialOrd> BinTree<T> {
        fn add_sorted(&mut self, data: T) {
            match self.0 {
                Some(ref mut bd) => {
                    if data < bd.data {
                        bd.left.add_sorted(data);
                    } else {
                        bd.right.add_sorted(data);
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

    impl<T: Debug> BinTree<T> {
        fn print_lfirst(&self, dp: i32) {
            if let Some(ref bd) = self.0 {
                bd.left.print_lfirst(dp + 1);
                let mut spc = String::new();
                for _ in 0..dp {
                    spc.push('.');
                }
                println!("{} {:?}", spc, bd.data);
                bd.right.print_lfirst(dp + 1);
            }
        }
    }

    #[test]
    fn test_bin_tree() {
        let mut t = BinTree::new();
        t.add_sorted(1);
        t.add_sorted(2);
        t.add_sorted(5);
        t.add_sorted(8);
        t.add_sorted(4);
        t.add_sorted(19);
        println!("{:?}", t);
        t.print_lfirst(0);
    }
}
