use std::fs;
use std::net::SocketAddr;

use serde::Deserialize;
use tracing::Level;
use tracing_appender::{
    non_blocking::WorkerGuard,
    rolling::{RollingFileAppender, Rotation},
};
use tracing_subscriber::{fmt::writer::MakeWriterExt, layer::SubscriberExt, Layer};

#[derive(Deserialize, Debug)]
pub(crate) struct Config {
    pub(crate) http: Http,
    pub(crate) tracing: Tracing,
    pub(crate) database: Database,
}

impl Config {
    pub(crate) fn new(filename: &str) -> anyhow::Result<Self> {
        let contents = fs::read_to_string(filename)?;
        let cfg = toml::from_str(&contents)?;
        Ok(cfg)
    }
}

#[derive(Deserialize, Debug)]
pub(crate) struct Http {
    address: String,
    port: u16,
}

#[derive(Deserialize, Debug)]
pub(crate) struct Tracing {
    pub(crate) level: String,
    pub(crate) rolling_file: RollingFile,
}

#[derive(Deserialize, Debug)]
pub(crate) struct Database {
    pub(crate) url: String,
    pub(crate) max_connection: u32,
}

impl Http {
    pub(crate) fn socketaddr(&self) -> anyhow::Result<SocketAddr> {
        let str = format!("{}:{}", self.address, self.port);
        let sock = str.parse::<SocketAddr>()?;
        Ok(sock)
    }
}

impl Tracing {
    pub(crate) fn init(&self) -> anyhow::Result<WorkerGuard> {
        let _app_name = module_path!().split("::").next().unwrap().to_owned();

        let rotation = match self.rolling_file.rotation.as_str() {
            "MINUTELY" => Rotation::MINUTELY,
            "HOURLY" => Rotation::HOURLY,
            "NEVER" => Rotation::NEVER,
            _ => Rotation::DAILY,
        };

        let level = match self.level.as_str() {
            "TRACE" => Level::TRACE,
            "DEBUG" => Level::DEBUG,
            "INFO" => Level::INFO,
            "WARN" => Level::WARN,
            "ERROR" => Level::ERROR,
            _ => Level::TRACE,
        };

        let (writer, guard) = tracing_appender::non_blocking(RollingFileAppender::new(
            rotation,
            self.rolling_file.directory.as_str(),
            self.rolling_file.prefix.as_str(),
        ));

        let flayer = tracing_subscriber::fmt::layer()
            .with_file(false)
            .with_line_number(false)
            .with_ansi(false)
            .with_target(true)
            .with_writer(writer.with_max_level(level).with_filter(move |_meta| {
                //meta.target().starts_with(app_name.as_str())
                true
            }))
            .and_then(tracing_subscriber::fmt::layer().pretty());

        tracing::subscriber::set_global_default(tracing_subscriber::registry().with(flayer))?;

        Ok(guard)
    }
}

#[derive(Deserialize, Debug)]
pub(crate) struct RollingFile {
    pub(crate) directory: String,
    pub(crate) prefix: String,
    pub(crate) rotation: String,
}
