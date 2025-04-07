use core::panic;
use std::{
    fs::read_to_string,
    io::{self, Read},
    str::FromStr,
};

use chrono::{TimeDelta, Utc};
use clap::Parser;
use cli::CliArgs;
use dotenvy::dotenv;
use semver::{Version, VersionReq};
use serde::Deserialize;
use serde_json::Value;
use sqlx::{query, PgPool};

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

    let unknown: Unknown = if let Some(file) = args.file {
        serde_json::from_str(&read_to_string(file)?)?
    } else {
        let mut buf = String::new();
        io::stdin().lock().read_to_string(&mut buf)?;
        serde_json::from_str(&buf)?
    };
    let sensor = if let Some(sensor) = query!(
        "SELECT id FROM \"Sensor\" WHERE serial_id = $1",
        unknown.serial_id
    )
    .fetch_optional(&pool)
    .await?
    {
        sensor
    } else {
        panic!("Sensor with serial {} not found", unknown.serial_id)
    }
    .id;

    let version = Version::from_str(&unknown.version)?;
    match unknown.kind {
        SensorKind::Temperature => {
            let req = VersionReq::parse("<=0.0.1")?;
            if !req.matches(&version) {
                panic!("Cannot handle version")
            }

            // then insert the temperature
            let data: Vec<TemperatureReading> = serde_json::from_value(Value::Array(unknown.data))?;

            for data in data {
                let timestamp = Utc::now()
                    .checked_add_signed(TimeDelta::seconds(data.timestamp))
                    .expect("adding shouldnt fail")
                    .naive_utc();
                query!(
                    "INSERT INTO \"Metric\" (value, timestamp, type, \"sensorID\")
                    VALUES ($1, $2, 'temp', $3)",
                    data.temperature,
                    timestamp,
                    sensor
                )
                .execute(&pool)
                .await?;
            }
        }
    };

    Ok(())
}

#[derive(Debug, Deserialize)]
struct Unknown {
    version: String,
    #[serde(rename = "type")]
    kind: SensorKind,
    serial_id: String,
    data: Vec<Value>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
enum SensorKind {
    Temperature,
}

#[derive(Debug, Deserialize)]
struct TemperatureReading {
    reading: u32,
    timestamp: i64,
    temperature: f64,
}
