#[cfg(test)]
mod test {
    fn fibonacci(n: i32) -> i32 {
        if n <= 1 {
            return 1;
        }
        fibonacci(n - 1) + fibonacci(n - 2)
    }
    fn fibonacci_iter(n: i32) -> i32 {
        let mut a = 1;
        let mut b = 1;
        let mut res = 1;
        for _ in 1..n {
            res = a + b;
            a = b;
            b = res;
        }
        res
    }
    fn fibonacci_dynamic(n: i32) -> (i32, i32) {
        if n == 0 {
            return (1, 0);
        }
        let (a, b) = fibonacci_dynamic(n - 1);
        (a + b, a)
    }

    #[test]
    fn test_fibonacci() {
        for i in 0..10 {
            println!(
                "navie = {},iter={},dynamic={}",
                fibonacci(i),
                fibonacci_iter(i),
                fibonacci_dynamic(i).0
            )
        }
    }
}
