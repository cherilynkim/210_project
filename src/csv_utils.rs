
use csv::ReaderBuilder;
use serde::Deserialize;
use std::error::Error;

#[derive(Debug, Deserialize)]
pub struct Transaction {
    pub user_id: String,
    pub product_id: String,
    pub category: String,
    pub final_price: f64,
}

pub fn clean_and_load_csv(file_path: &str) -> Result<Vec<Transaction>, Box<dyn Error>> {
    let mut reader = ReaderBuilder::new().from_path(file_path)?;
    let mut transactions = Vec::new();

    for result in reader.deserialize() {
        let record: Transaction = result?;
        if record.final_price > 0.0 {
            transactions.push(record);
        }
    }

    Ok(transactions)
}