use std::error::Error;
use std::env;
use yfinance_rs::{Interval, Range, Ticker, YfClient};
use yfinance_rs::core::conversions::money_to_f64;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // --- Runtime Ticker Input ---
    let args: Vec<String> = env::args().collect();
    let ticker_symbol = if args.len() > 1 {
        &args[1]
    } else {
        eprintln!("Usage: cargo run -- <TICKER_SYMBOL>");
        return Ok(());
    };

    let annual_risk_free_rate = 0.03; // 3% per year

    // --- Fetch Historical Data ---
    let client = YfClient::default();
    let ticker = Ticker::new(&client, ticker_symbol);

    println!("Fetching historical data for {}...", ticker_symbol);

    let history_data = ticker
    .history(Some(Range::Y1), Some(Interval::D1), false)
    .await?;

    let prices: Vec<f64> = history_data
    .iter()
    .map(|c| money_to_f64(&c.close))
    .collect();

    if prices.len() < 2 {
        eprintln!("Error: Not enough data points to calculate returns.");
        return Ok(());
    }

    // --- Daily Returns ---
    let daily_returns: Vec<f64> = prices
    .windows(2)
    .map(|w| (w[1] - w[0]) / w[0])
    .collect();

    // --- Metrics ---
    let mean_daily = calculate_mean(&daily_returns);
    let std_dev_daily = calculate_std_dev(&daily_returns, mean_daily);
    let sharpe_ratio = calculate_sharpe_ratio(&daily_returns, annual_risk_free_rate);

    // --- Output ---
    println!("--- Sharpe Calculator ---");
    println!("Ticker: {}", ticker_symbol);
    println!("Data points: {}", daily_returns.len());
    println!("Annual risk-free rate: {:.2}%", annual_risk_free_rate * 100.0);

    println!("\n[Daily Metrics]");
    println!("Mean daily return: {:.6}", mean_daily);
    println!("Daily volatility: {:.6}", std_dev_daily);

    println!("\n[Result]");
    println!("Annualised Sharpe ratio: {:.4}", sharpe_ratio);
    if sharpe_ratio > 1.0 {
        println!("Interpretation: Good risk-adjusted return (> 1.0).");
    } else {
        println!("Interpretation: Weak risk-adjusted return (< 1.0).");
    }

    Ok(())
}

// --- Utilities ---
fn calculate_mean(data: &[f64]) -> f64 {
    data.iter().sum::<f64>() / data.len() as f64
}

fn calculate_std_dev(data: &[f64], mean: f64) -> f64 {
    if data.len() < 2 { return 0.0; }
    let variance = data.iter().map(|&x| (x - mean).powi(2)).sum::<f64>() / (data.len() - 1) as f64;
    variance.sqrt()
}

fn calculate_sharpe_ratio(daily_returns: &[f64], annual_risk_free_rate: f64) -> f64 {
    const TRADING_DAYS_PER_YEAR: f64 = 252.0;

    let mean_daily_return = calculate_mean(daily_returns);
    let std_dev_daily = calculate_std_dev(daily_returns, mean_daily_return);
    if std_dev_daily == 0.0 { return f64::INFINITY; }

    let daily_risk_free_rate = annual_risk_free_rate / TRADING_DAYS_PER_YEAR;
    let daily_excess_return = mean_daily_return - daily_risk_free_rate;

    daily_excess_return / std_dev_daily * TRADING_DAYS_PER_YEAR.sqrt()
}
