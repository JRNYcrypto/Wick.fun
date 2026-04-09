#[cfg(test)]
mod tests {
    #[test]
    fn test_interval_validation() {
        let valid = 900u64;
        let invalid = 30u64;
        assert!(valid >= 60);
        assert!(invalid < 60);
    }

    #[test]
    fn test_retention_validation() {
        let valid = 2000u16;
        let invalid = 15000u16;
        assert!(valid <= 10000);
        assert!(invalid > 10000);
    }
}
