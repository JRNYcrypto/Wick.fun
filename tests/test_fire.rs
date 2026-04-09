#[cfg(test)]
mod tests {
    #[test]
    fn test_timer_elapsed() {
        let last_fired = 1_700_000_000i64;
        let now = last_fired + 1000;
        let interval = 900i64;
        assert!(now - last_fired >= interval);
    }

    #[test]
    fn test_timer_not_elapsed() {
        let last_fired = 1_700_000_000i64;
        let now = last_fired + 500;
        let interval = 900i64;
        assert!(now - last_fired < interval);
    }

    #[test]
    fn test_retention_math() {
        let tokens_bought = 1_000_000u64;
        let retention_bps = 2000u64;
        let sell_ratio = 10000 - retention_bps;
        let tokens_sold = tokens_bought * sell_ratio / 10000;
        let tokens_retained = tokens_bought - tokens_sold;

        assert_eq!(tokens_sold, 800_000);
        assert_eq!(tokens_retained, 200_000);
    }
}
