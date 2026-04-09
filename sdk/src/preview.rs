#[derive(Debug)]
pub struct WickPreview {
    pub buy_sol: f64,
    pub tokens_bought: f64,
    pub tokens_sold: f64,
    pub tokens_retained: f64,
    pub sell_sol: f64,
    pub net_volume: f64,
    pub price_impact_bps: u16,
}

pub fn preview_wick(
    vault_balance_sol: f64,
    retention_ratio: f64,
    token_price_sol: f64,
    pool_liquidity_sol: f64,
) -> WickPreview {
    let buy_sol = vault_balance_sol;
    let tokens_bought = buy_sol / token_price_sol;
    let tokens_sold = tokens_bought * (1.0 - retention_ratio);
    let tokens_retained = tokens_bought - tokens_sold;
    let sell_sol = tokens_sold * token_price_sol;
    let net_volume = buy_sol + sell_sol;

    let price_impact = if pool_liquidity_sol > 0.0 {
        ((buy_sol / pool_liquidity_sol) * 10000.0) as u16
    } else {
        0
    };

    WickPreview {
        buy_sol,
        tokens_bought,
        tokens_sold,
        tokens_retained,
        sell_sol,
        net_volume,
        price_impact_bps: price_impact,
    }
}
