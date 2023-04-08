use chrono::Utc;
use futures::prelude::*;
use influxdb2::Client;
use influxdb2_derive::WriteDataPoint;

#[derive(Default, WriteDataPoint)]
#[measurement = "cpu_load_short"]
struct CpuLoadShort {
    #[influxdb(tag)]
    host: Option<String>,
    #[influxdb(tag)]
    region: Option<String>,
    #[influxdb(field)]
    value: f64,
    #[influxdb(timestamp)]
    time: i64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let host = std::env::var("host").unwrap();
    let org = std::env::var("org").unwrap();
    let token = std::env::var("token").unwrap();
    let bucket = std::env::var("bucket").unwrap();
    let client = Client::new(host, org, token);

    let points = vec![
        CpuLoadShort {
            host: Some("server01".to_owned()),
            region: Some("us-west".to_owned()),
            value: 0.64,
            time: Utc::now().timestamp_nanos(),
        },
        CpuLoadShort {
            host: Some("server02".to_owned()),
            region: Some("us-east".to_owned()),
            value: 0.63,
            time: Utc::now().timestamp_nanos(),
        },
    ];

    client.write(bucket.as_str(), stream::iter(points)).await?;

    Ok(())
}
