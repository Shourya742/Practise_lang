#[cfg(test)]
mod test {
    use std::fmt::Debug;

    struct RandGen {
        curr: usize,
        mul: usize,
        inc: usize,
        module: usize,
    }

    impl RandGen {
        pub fn new(curr: usize) -> Self {
            RandGen {
                curr: curr,
                mul: 56394237,
                inc: 346423491,
                module: 23254544563,
            }
        }
        pub fn next_v(&mut self, max: usize) -> usize {
            self.curr = (self.curr * self.mul + self.inc) % self.module;
            self.curr % max
        }
    }

    fn pivot<T: PartialOrd>(v: &mut [T]) -> usize {
        let mut p = 0;
        for i in 1..v.len() {
            if v[i] < v[p] {
                v.swap(p + 1, i);
                v.swap(p, p + 1);
                p += 1;
            }
        }
        p
    }

    fn quick_sort<T: PartialOrd + Debug>(v: &mut [T]) {
        if v.len() <= 1 {
            return;
        }
        let p = pivot(v);
        println!("{:?}", v);
        let (a, b) = v.split_at_mut(p);
        quick_sort(a);
        quick_sort(&mut b[1..]);
    }

    struct RawSend<T>(*mut [T]);
    unsafe impl<T> Send for RawSend<T> {}

    // fn threaded_quick_sort<T: 'static + PartialOrd + Debug + Send>(v: &mut [T]) {
    //     if v.len() <= 1 {
    //         return;
    //     }
    //     let p = pivot(v);
    //     println!("{:?}", v);
    //     let (a, b) = v.split_at_mut(p);
    //     let raw_a = a as *mut [T];
    //     let raw_s = RawSend(raw_a);
    //     unsafe {
    //         let handle = std::thread::spawn(move || {
    //             threaded_quick_sort(&mut *raw_s.0);
    //         });
    //         threaded_quick_sort(&mut b[1..]);
    //         handle.join().ok();
    //     }
    // }

    #[test]
    fn test_quick_sort() {
        let mut v = vec![4, 6, 1, 19, 8, 11, 13, 3];
        quick_sort(&mut v);
        assert_eq!(v, vec![1, 3, 4, 6, 8, 11, 13, 19]);
    }

    #[test]
    fn test_pivot() {
        let mut v = vec![4, 6, 1, 19, 8, 11, 13, 3];
        let p = pivot(&mut v);
        for x in 0..v.len() {
            assert!((v[x] < v[p]) == (x < p))
        }
    }

    #[test]
    fn test_rands_printout() {
        let mut r = RandGen::new(12);
        for _ in 0..100 {
            println!("---{}---", r.next_v(100));
        }
    }
}
