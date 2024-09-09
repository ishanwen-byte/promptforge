pub trait IsEven {
    fn is_even(&self) -> bool;
}

impl<T> IsEven for T
where
    T: std::ops::Rem<Output = T> + From<u8> + PartialEq + Copy,
{
    fn is_even(&self) -> bool {
        *self % T::from(2) == T::from(0)
    }
}

#[cfg(test)]
mod tests {
    use super::IsEven;

    #[test]
    fn test_is_even_i32() {
        assert!(4i32.is_even());
        assert!(!5i32.is_even());
        assert!(0i32.is_even());
        assert!((-2i32).is_even());
        assert!(!(-3i32).is_even());
    }

    #[test]
    fn test_is_even_u32() {
        assert!(4u32.is_even());
        assert!(!5u32.is_even());
        assert!(0u32.is_even());
    }

    #[test]
    fn test_is_even_i64() {
        assert!(4i64.is_even());
        assert!(!5i64.is_even());
        assert!(0i64.is_even());
        assert!((-2i64).is_even());
        assert!(!(-3i64).is_even());
    }

    #[test]
    fn test_is_even_u64() {
        assert!(4u64.is_even());
        assert!(!5u64.is_even());
        assert!(0u64.is_even());
    }
}
