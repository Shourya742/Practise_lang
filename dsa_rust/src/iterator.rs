

#[derive(Clone)]
pub struct  Stepper {
    curr: i32,
    step: i32,
    max: i32
}

impl Iterator for Stepper {
    
    type Item = i32;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.curr > self.max {
            return None;
        }
        let res = self.curr;
        self.curr += self.step;
        Some(res)
    } 
}

fn main() {
    let mut st = Stepper{curr: 2, step: 3, max: 15};
    let mut st2 = st.clone();

    loop {
        match st.next() {
            Some(v) => println!("loop: {}", v),
            None => break
        }
    }

    while let Some(v) = st2.next() {
        println!("I am here: {:?}",v);
    }
}
