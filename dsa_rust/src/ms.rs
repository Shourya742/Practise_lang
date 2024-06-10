use std::fmt::Debug;



pub fn merge_sort<T: PartialOrd + Debug + Copy>(mut v: Vec<T>) -> Vec<T> {
    if v.len() <= 1 {
        return v;
    }

    let mut result: Vec<T> = Vec::with_capacity(v.len());

    let mid = v.len()/2;
    let right_half = v.split_off(mid);
    let left_half = v;
    
    let right_half = merge_sort(right_half);
    let left_half = merge_sort(left_half);
 

    let mut left_iterator = left_half.into_iter();
    let mut right_iterator = right_half.into_iter();

    let mut left_peek = left_iterator.next();
    let mut right_peek = right_iterator.next();
   
    loop {
        match left_peek {
            Some(left_value) => {
                if let Some(right_value) = right_peek {
                    if right_value > left_value {
                        result.push(left_value);
                        left_peek = left_iterator.next();
                    } else {
                        result.push(right_value);
                        right_peek = right_iterator.next();
                    }
                } else {
                    result.push(left_value);
                    result.extend(left_iterator);
                    break;
                }
            },
            None => {
                if let Some(right_value) = right_peek {
                    result.push(right_value);
                    result.extend(right_iterator);
                    break;
                }
            }
        }
    }

    println!(" {:?}",result);
    return result;
}

fn main() {
    let v = vec![1,4,3,2,8];
    merge_sort(v);
}