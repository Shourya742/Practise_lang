#[cfg(test)]
mod test {
    use std::fmt::Debug;

    pub fn merge_sort<T: PartialOrd + Debug>(mut v: Vec<T>) -> Vec<T> {
        // sort the left half,
        // sort the right half,
        // bring the sorted halfs together
        println!("MS:{:?}", v);
        if v.len() <= 1 {
            return v;
        }

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
                        if b_val < a_val {
                            res.push(b_peek.take().unwrap());
                            b_peek = b_it.next();
                        } else {
                            res.push(a_peek.take().unwrap());
                            a_peek = a_it.next();
                        }
                    }
                    None => {
                        res.push(a_peek.take().unwrap());
                        res.extend(a_it);
                        return res;
                    }
                },
                None => {
                    if let Some(ref b_val) = b_peek {
                        res.push(b_peek.take().unwrap());
                    }
                    res.extend(b_it);
                    return res;
                }
            }
        }
    }

    #[test]
    fn test_merge_sort() {
        let v = vec![4, 3, 6, 1, 8, 11, 13];
        let v = merge_sort(v);
        assert_eq!(v, vec![1, 3, 4, 6, 8, 11, 13])
    }
}
