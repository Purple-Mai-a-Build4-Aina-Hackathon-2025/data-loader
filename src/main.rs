use std::io::{self, Read};

use clap::Parser;
use cli::CliArgs;
use csv::Reader;
use dotenvy::dotenv;
use serde::Deserialize;
use sqlx::{query, types::BigDecimal, PgPool};

pub mod cli;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    color_eyre::install()?;
    let args = CliArgs::parse();
    dotenv()?;
    let conn = if let Some(conn) = args.connection {
        conn
    } else {
        dotenvy::var("DATABASE_URL")?
    };
    let pool = PgPool::connect(&conn).await?;

    if let Some(file) = args.file {
        handle(csv::Reader::from_path(&file)?, pool).await?
    } else {
        handle(csv::Reader::from_reader(io::stdin()), pool).await?
    };
    Ok(())
}
async fn handle<T>(mut rdr: Reader<T>, pool: PgPool) -> eyre::Result<()>
where
    T: Read,
{
    for result in rdr.deserialize() {
        let r: Record = result?;
        println!("{:?}", r);
        query!("
        INSERT INTO nutrient_reading (
            site_id, sensor_id, tec, ph, sulfur, phosphorous, olsen_p, calcium,
            magnesium, potassium, sodium, boron, iron, manganese, copper, zinc, aluminum, total_carbon_percentage,
            total_nitrogen_percentage, nitrate, ammonium, soil_health_score)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22)",
        1, 1, r.tec, r.ph, r.sulfur, r.phosphorus, r.olsen_p, r.calcium, r.magnesium, r.potassium, r.sodium, r.boron, r.iron, r.manganese,
        r.copper, r.zinc, r.aluminum, r.total_carbon, r.total_nitrogen, r.nitrate, r.ammonium, r.soil_health_score)
            .execute(&pool)
            .await?;
    }
    Ok(())
}

#[derive(Debug, Deserialize)]
struct Record {
    #[serde(rename = "TEC")]
    tec: BigDecimal,
    #[serde(rename = "pH")]
    ph: BigDecimal,
    #[serde(rename = "Sulfur")]
    sulfur: i32,
    #[serde(rename = "Phosphorus")]
    phosphorus: i32,
    #[serde(rename = "Olsen P")]
    olsen_p: i32,
    #[serde(rename = "Calcium")]
    calcium: i32,
    #[serde(rename = "Magnesium")]
    magnesium: i32,
    #[serde(rename = "Potassium")]
    potassium: i32,
    #[serde(rename = "Sodium")]
    sodium: i32,
    #[serde(rename = "Boron")]
    boron: BigDecimal,
    #[serde(rename = "Iron")]
    iron: i32,
    #[serde(rename = "Manganese")]
    manganese: i32,
    #[serde(rename = "Copper")]
    copper: BigDecimal,
    #[serde(rename = "Zinc")]
    zinc: BigDecimal,
    #[serde(rename = "Aluminum")]
    aluminum: i32,
    #[serde(rename = "Total Carbon %")]
    total_carbon: BigDecimal,
    #[serde(rename = "Total Nitrogen %")]
    total_nitrogen: BigDecimal,
    #[serde(rename = "Nitrate")]
    nitrate: i32,
    #[serde(rename = "Ammonium")]
    ammonium: i32,
    #[serde(rename = "Soil Health Score")]
    soil_health_score: i32,
}
