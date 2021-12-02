// ------------------------------------------------------------------------------
// Copyright 2021 Uwe Arzt, mail@uwe-arzt.de
// SPDX-License-Identifier: Apache-2.0
// ------------------------------------------------------------------------------

use futures::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let org = "testorg";
    let bucket = "testbucket";
    let influx_url = "http://localhost:8086";
    let token = "y2SkbXbibQ4EMMXpp-hM_KPVjPnmTLfLe1KoB_g4lKEl5cVJDnuQhvAk3nfLZd8phsg5JHtc5syyWP7KCDAo3w==";

    let client = influxdb2::Client::new(influx_url, org, token);

    let points = vec![
        influxdb2::models::DataPoint::builder("temp")
            .tag("room", "serverraum")
            .field("value", 21.1)
            .build()?,
    ];

    client.write(bucket, stream::iter(points)).await?;

    Ok(())
}
