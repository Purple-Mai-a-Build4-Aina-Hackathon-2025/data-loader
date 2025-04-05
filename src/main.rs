use std::io::{self, Read};

use clap::Parser;
use cli::CliArgs;
use csv::Reader;
use serde::Deserialize;

pub mod cli;
fn main() -> eyre::Result<()> {
    let args = CliArgs::parse();
    if let Some(file) = args.file {
        handle(csv::Reader::from_path(&file)?)?
    } else {
        handle(csv::Reader::from_reader(io::stdin()))?
    };
    Ok(())
}
fn handle<T>(mut rdr: Reader<T>) -> eyre::Result<()>
where
    T: Read,
{
    for result in rdr.deserialize() {
        let record: Record = result?;
        println!("{:?}", record);
    }
    Ok(())
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Record {
    sample_id: String,
    #[serde(rename = "TEC")]
    tec: f64,
    #[serde(rename = "pH")]
    ph: f64,
    #[serde(rename = "EC")]
    ec: f64,
    #[serde(rename = "Sulfur")]
    sulfur: u64,
    #[serde(rename = "Phosphorus")]
    phosphorus: u64,
    #[serde(rename = "Olsen P")]
    olsen_p: u64,
    #[serde(rename = "Calcium")]
    calcium: u64,
    #[serde(rename = "Magnesium")]
    magnesium: u64,
    #[serde(rename = "Potassium")]
    potassium: u64,
    #[serde(rename = "Sodium")]
    sodium: u64,
    #[serde(rename = "Boron")]
    boron: f64,
    #[serde(rename = "Iron")]
    iron: u64,
    #[serde(rename = "Manganese")]
    manganese: u64,
    #[serde(rename = "Copper")]
    copper: f64,
    #[serde(rename = "Zinc")]
    zinc: f64,
    #[serde(rename = "Aluminum")]
    aluminum: f64,
    #[serde(rename = "Total Carbon %")]
    total_carbon: f64,
    #[serde(rename = "Total Nitrogen %")]
    total_nitrogen: f64,
    #[serde(rename = "Nitrate")]
    nitrate: u64,
    #[serde(rename = "Ammonium")]
    ammonium: u64,
    #[serde(rename = "Soil Health Score")]
    soil_health_score: u64,
}
