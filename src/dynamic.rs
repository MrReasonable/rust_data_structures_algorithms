pub fn fibonacci(n: i32) -> i32 {
    if n <= 1 {
        return 1;
    }
    fibonacci(n - 1) + fibonacci(n - 2)
}

pub fn fibonacci_iter(n: i32) -> i32 {
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

pub fn fibonacci_dynamic(n: i32) -> (i32, i32) {
    if n == 0 {
        return (1, 0);
    }

    let (a, b) = fibonacci_dynamic(n - 1);
    (a + b, a)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_fibonacci() {
        assert_eq!(fibonacci(0), 1);
        assert_eq!(fibonacci(1), 1);
        assert_eq!(fibonacci(2), 2);
        assert_eq!(fibonacci(3), 3);
        assert_eq!(fibonacci(4), 5);
        assert_eq!(fibonacci(5), 8);
    }

    #[test]
    pub fn test_fibonacci_iter() {
        assert_eq!(fibonacci_iter(0), 1);
        assert_eq!(fibonacci_iter(1), 1);
        assert_eq!(fibonacci_iter(2), 2);
        assert_eq!(fibonacci_iter(3), 3);
        assert_eq!(fibonacci_iter(4), 5);
        assert_eq!(fibonacci_iter(5), 8);
    }

    #[test]
    pub fn test_fibonacci_dynamic() {
        assert_eq!(fibonacci_dynamic(0).0, 1);
        assert_eq!(fibonacci_dynamic(1).0, 1);
        assert_eq!(fibonacci_dynamic(2).0, 2);
        assert_eq!(fibonacci_dynamic(3).0, 3);
        assert_eq!(fibonacci_dynamic(4).0, 5);
        assert_eq!(fibonacci_dynamic(5).0, 8);
    }
}
