<div align="center">
    <br/>
    <img
        alt="rust-influxdb"
        src="https://i.imgur.com/4k7l8XJ.png"
        width=250px />
    <br/>
    <br/>
    <strong>Unofficial InfluxDB Driver for Rust</strong>
</div>
<p align="center">
    <a href="https://crates.io/crates/rinflux">
        <img src="https://img.shields.io/crates/v/influxdb.svg"/>
    </a>
    <a href="https://github.com/workfoxes/rinflux/actions/workflows/rust.yml">
        <img src="https://github.com/influxdb-rs/influxdb-rust/actions/workflows/rust.yml/badge.svg" alt='Build Status' />
    </a>
    <a href="https://workfoxes.github.io/rinflux/tarpaulin-report.html">
        <img src="https://workfoxes.github.io/rinflux/coverage.svg" alt="Coverage Report" />
    </a>
    <a href="https://docs.rs/rinflux">
        <img src="https://docs.rs/rinflux/badge.svg" alt='Documentation Status' />
    </a>
    <a href="https://www.rust-lang.org/en-US/">
        <img src="https://img.shields.io/badge/Made%20with-Rust-orange.svg" alt='Build with Rust' />
    </a>
    <a href="https://blog.rust-lang.org/2021/06/17/Rust-1.53.0.html">
        <img src="https://img.shields.io/badge/rustc-1.53+-yellow.svg" alt='Minimum Rust Version' />
    </a>
    <a href="https://github.com/workfoxes/rinflux/actions/workflows/release-drafter.yml">
        <img src="https://github.com/workfoxes/rinflux/actions/workflows/release-drafter.yml/badge.svg"/>
    </a>
</p>

This library is a work in progress. This means a feature you might need is not implemented
yet or could be handled better.

Pull requests are always welcome. See [Contributing](https://github.com/workfoxes/influxdb/blob/main/CONTRIBUTING.md) and [Code of Conduct](https://github.com/workfoxes/influxdb/blob/main/CODE_OF_CONDUCT.md). For a list of past changes, see [CHANGELOG.md](https://github.com/workfoxes/influxdb/blob/main/CHANGELOG.md).

### Currently, Supported Features

-   Reading and Writing to InfluxDB
-   Optional Serde Support for Deserialization
-   Running multiple queries in one request (e.g. `SELECT * FROM weather_berlin; SELECT * FROM weather_london`)
-   Authenticated and Unauthenticated Connections
-   `async`/`await` support
-   `#[derive(InfluxDbWriteable)]` Derive Macro for Writing / Reading into Structs
-   `GROUP BY` support
-   Tokio and async-std support (see example below) or [available backends](https://github.com/influxdb-rs/influxdb-rust/blob/main/influxdb/Cargo.toml)
-   Swappable HTTP backends ([see below](#Choice-of-HTTP-backend))

## Quickstart

Add the following to your `Cargo.toml`

```toml
rinflux = { version = "0.1.0", git = "https://github.com/workfoxes/rinflux.git" }
```

For an example with using Serde deserialization, please refer to [serde_integration](crate::integrations::serde_integration)

```rust
use rinflux::{Client, Query, Timestamp, ReadQuery};
use rinflux::InfluxDbWriteable;
use chrono::{DateTime, Utc};

#[tokio::main]
// or #[async_std::main] if you prefer
async fn main() {
    // Connect to db `test` on `http://localhost:8086`
    let client = Client::new("http://localhost:8086", "test");

    #[derive(InfluxDbWriteable)]
    struct WeatherReading {
        time: DateTime<Utc>,
        humidity: i32,
        #[influxdb(tag)] wind_direction: String,
    }

    // Let's write some data into a measurement called `weather`
    let weather_reading = WeatherReading {
        time: Timestamp::Hours(1).into(),
        humidity: 30,
        wind_direction: String::from("north"),
    };

    let write_result = client
        .query(weather_reading.into_query("weather"))
        .await;
    assert!(write_result.is_ok(), "Write result was not okay");

    // Let's see if the data we wrote is there
    let read_query = ReadQuery::new("SELECT * FROM weather");

    let read_result = client.query(read_query).await;
    assert!(read_result.is_ok(), "Read result was not ok");
    println!("{}", read_result.unwrap());
}
```

## License

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

@ 2022 Workfoxes and [contributors](https://github.com/workfoxes/rinflux/graphs/contributors).
