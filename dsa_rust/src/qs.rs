use std::{fmt::Debug, sync::Mutex};


pub struct RandGen {
    curr: usize,
    mul: usize,
    inc: usize,
    modulo: usize
}

impl RandGen {
    pub fn new(curr: usize) -> Self {
        RandGen {
            curr,
            mul: 1292512,
            inc: 14041294,
            modulo: 12902402829
        }
    }

    pub fn next_v(&mut self) -> usize {
        self.curr = (self.curr * self.mul + self.inc) % self.modulo;
        self.curr

    }
}

fn pivot<T: PartialOrd>(v: &mut [T]) -> usize {
    let mut pivot = 0;
    for i in 1..v.len() {
        if v[i] < v[pivot] {
            v.swap(pivot + 1, i);
            v.swap(pivot,pivot+1);
            pivot += 1;
        }
    }
    pivot
}

fn quick_sort<T: PartialOrd+Debug+ Send>(v: &mut[T]) {
    if v.len() <= 1 {
        return;
    }
    let p = pivot(v);
    let (a,b) = v.split_at_mut(p);
    quick_sort(a);
    quick_sort(&mut b[1..]);
}

fn main() {
    let mut v = vec![3,4,9,2,8];
    quick_sort(&mut v);
    println!("{:?}",v);
}