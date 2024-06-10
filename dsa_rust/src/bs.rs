
pub fn unoptimised_bubble_sort<T: PartialOrd> (v: &mut [T]) {
    for _ in 0..v.len() { 
        for i in 0..v.len()-1 {
            if v[i] > v[i+1] {
                v.swap(i, i+1)
            }
            
        }
    }
}

pub fn optimised_bubble_sort<T: PartialOrd>(v: &mut [T]) {
    for i in 0..v.len()-1 {
        let mut is_sorted = true;

        for j in 0..v.len() - 1 -i {
            if v[j] > v[j+1] {
                is_sorted = false;
                v.swap(j,j+1);
            }
        }
        if is_sorted {
            return;
        }
    }
}
fn main() {
    let mut v = vec![1,4,3,2,8];
    unoptimised_bubble_sort(&mut v);
    println!("V: {:?}", v);
    v = vec![12,41,21,94,2012,423,24];
    optimised_bubble_sort(&mut v);
    println!("V: {:?}",v);
}