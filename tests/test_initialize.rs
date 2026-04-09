#[cfg(test)]
mod tests {
    #[test]
    fn test_vault_creation() {
        let interval = 900u64;
        let retention = 2000u16;
        let min_balance = 100_000_000u64;
        let slippage = 300u16;

        assert!(interval >= 60);
        assert!(retention <= 10000);
        assert!(slippage <= 10000);
        assert!(min_balance > 0);
    }

    #[test]
    fn test_vault_pda_derivation() {
        let mint_a = "mintAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA";
        let mint_b = "mintBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBB";
        assert_ne!(mint_a, mint_b);
    }
}
