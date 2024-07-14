use std::error::Error;
use std::time::Duration;
use reqwest::blocking::Client;
use serde::Deserialize;

#[derive(Deserialize)]
struct EthPriceResponse {
    ethereum: CurrencyData,
}

#[derive(Deserialize)]
struct CurrencyData {
    usd: f64,
}

#[derive(Deserialize)]
struct ValidatorStats {
    data: ValidatorData,
}

#[derive(Deserialize)]
struct ValidatorData {
    performance1d: f64,
    performance7d: f64,
    balance: f64,
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("🔍 Beacon Oracle Tool — Ethereum staking analytics");

    let client = Client::builder()
        .timeout(Duration::from_secs(10))
        .build()?;

    // 1. Получаем цену ETH в USD
    let price_url = "https://api.coingecko.com/api/v3/simple/price?ids=ethereum&vs_currencies=usd";
    let price_resp: EthPriceResponse = client.get(price_url).send()?.json()?;
    let eth_price = price_resp.ethereum.usd;
    println!("💰 ETH Price: ${:.2}", eth_price);

    // 2. Получаем статистику по конкретному валидатору (пример валидатора)
    let validator_index = 123456; // <-- сюда можно подставить любой index валидатора
    let validator_url = format!("https://beaconcha.in/api/v1/validator/{}", validator_index);
    let stats_resp: ValidatorStats = client.get(&validator_url).send()?.json()?;

    println!(
        "📊 Validator #{} — Balance: {:.4} ETH | Perf 1d: {:.2}% | Perf 7d: {:.2}%",
        validator_index,
        stats_resp.data.balance / 1e9, // Gwei -> ETH
        stats_resp.data.performance1d,
        stats_resp.data.performance7d
    );

    // 3. Расчёт потенциальной годовой доходности
    let avg_perf = (stats_resp.data.performance7d + stats_resp.data.performance1d) / 2.0;
    let yearly_yield_eth = stats_resp.data.balance / 1e9 * avg_perf / 100.0 * 365.0 / 7.0;
    let yearly_yield_usd = yearly_yield_eth * eth_price;

    println!(
        "📈 Potential yearly yield: {:.4} ETH (${:.2})",
        yearly_yield_eth, yearly_yield_usd
    );

    Ok(())
}
