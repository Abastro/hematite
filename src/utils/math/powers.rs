pub fn pow2(exp: u32) -> u64 {
    1u64 << exp
}

pub fn pow3(exp: u32) -> u64 {
    3u64.pow(exp)
}

pub fn is_pow2(n: u64) -> bool {
    n > 0 && n & (n - 1) == 0
}

pub fn is_pow3(n: u64) -> bool {
    if n == 0 {
        return false;
    }
    let mut n = n;
    while n % 3 == 0 {
        n /= 3;
    }
    n == 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pow2() {
        assert_eq!(pow2(0), 1);
        assert_eq!(pow2(1), 2);
        assert_eq!(pow2(10), 1024);
    }

    #[test]
    fn test_pow3() {
        assert_eq!(pow3(0), 1);
        assert_eq!(pow3(1), 3);
        assert_eq!(pow3(6), 729);
    }

    #[test]
    fn test_is_pow2() {
        assert!(is_pow2(1));
        assert!(is_pow2(2));
        assert!(is_pow2(1024));
        assert!(!is_pow2(0));
        assert!(!is_pow2(3));
        assert!(!is_pow2(6));
    }

    #[test]
    fn test_is_pow3() {
        assert!(is_pow3(1));
        assert!(is_pow3(3));
        assert!(is_pow3(27));
        assert!(is_pow3(729));
        assert!(!is_pow3(0));
        assert!(!is_pow3(2));
        assert!(!is_pow3(6));
    }
}
