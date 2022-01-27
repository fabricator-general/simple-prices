use bimap::BiMap;
use clap::Parser;
use colored::*;
use std::{thread, time};
use std::collections::BTreeMap;

#[derive(Parser)]
struct Cli {
    /// Token symbols, comma delimited
    token_symbols: String,
}

type SymbolsAndIds<'a> = BiMap<&'a str, &'a str>;

fn assemble_api_url(token_symbols_and_ids: &SymbolsAndIds, user_input_ids: &String) -> String {
    let mut v = vec![];
    for s in user_input_ids.split(",") {
        let name_result = token_symbols_and_ids.get_by_left(s.trim().to_lowercase().as_str());
        match name_result {
            Some(name) => v.push(*name),
            None => continue,
        }
    }
    v.join(",")
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    // symbols/api id lookup
    let mut token_symbols_and_ids: SymbolsAndIds = BiMap::new();
    token_symbols_and_ids.insert("btc", "bitcoin");
    token_symbols_and_ids.insert("eth", "ethereum");
    token_symbols_and_ids.insert("sol", "solana");
    token_symbols_and_ids.insert("avax", "avalanche-2");
    token_symbols_and_ids.insert("luna", "terra-luna");

    // parse cmd input
    let args = Cli::parse();
    let user_input_ids = assemble_api_url(&token_symbols_and_ids, &args.token_symbols);
    let url = format!(
        "https://api.coingecko.com/api/v3/simple/price?ids={}&vs_currencies=usd&include_24hr_change=true",
        user_input_ids
    );

    loop {
        let resp = reqwest::get(&url)
            .await?
            .json::<BTreeMap<String, BTreeMap<String, f64>>>()
            .await?;

        for (key, value) in resp {
            let symbol_result = token_symbols_and_ids.get_by_right(key.as_str());
            let symbol = match symbol_result {
                Some(v) => *v,
                None => continue,
            };

            let usd = match value.get("usd") {
                Some(v) => v,
                None => continue,
            };

            let change = match value.get("usd_24h_change") {
                Some(v) => v,
                None => continue,
            };

            let formatted_price = format!(
                "{} {} {}\n",
                format!("{}", symbol.to_uppercase()),
                format!("{}", usd).truecolor(251, 139, 30).bold(),
                if change.is_sign_negative() {
                    format!("{:.2}%", change).red().bold()
                } else {
                    format!("{:.2}%", change).green().bold()
                }
            );

            print!("{}", formatted_price);
        }
        println!();

        thread::sleep(time::Duration::from_millis(3000));
    }
}
