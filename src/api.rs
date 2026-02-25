use yahoo_finance_api as yahoo;

pub async fn fetch_market_data(ticker: &str) -> Option<(f64, f64)> {
    let provider = yahoo::YahooConnector::new().ok()?;
    let full_ticker = format!("{}.SA", ticker.to_uppercase().trim());
    
    let response = provider.get_quote_range(&full_ticker, "1d", "6mo").await.ok()?;
    let quotes = response.quotes().ok()?;
    
    if quotes.len() < 2 { return None; }

    let current_price = quotes.last()?.close;

    // Volatilidade Histórica Logarítmica
    let mut returns = Vec::new();
    for i in 1..quotes.len() {
        let prev = quotes[i-1].close;
        let curr = quotes[i].close;
        if prev > 0.0 {
            returns.push((curr / prev).ln());
        }
    }

    let mean = returns.iter().sum::<f64>() / returns.len() as f64;
    let variance = returns.iter().map(|r| (r - mean).powi(2)).sum::<f64>() / (returns.len() - 1) as f64;
    let std_dev = variance.sqrt();
    let vol_anualizada = std_dev * (252.0_f64).sqrt() * 100.0;

    Some((current_price, vol_anualizada))
}

pub async fn fetch_selic() -> Option<f64> {
    let provider = yahoo::YahooConnector::new().ok()?;
    match provider.get_quote_range("BRLSELIC=B", "1d", "1d").await {
        Ok(response) => {
            let quotes = response.quotes().unwrap_or_default();
            quotes.last().map(|q| q.close)
        }
        Err(_) => Some(10.75),
    }
}