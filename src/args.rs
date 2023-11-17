use std::sync::OnceLock;

use clap::{Args, Parser, Subcommand};

#[derive(Subcommand)]
pub enum ConversionOptions {
    /// Convert from one weight unit to metric
    Weight {
        #[command(flatten)]
        value: WeightOptions,
    },

    /// Convert from one time unit to metric
    Time {
        #[command(flatten)]
        value: TimeOptions,
    },

    /// Convert from one length unit to metric
    Length {
        #[command(flatten)]
        value: LengthOptions,
    },
}

#[derive(Args)]
#[group(required = true, multiple = false)]
pub struct WeightOptions {
    /// Convert lbs to kilograms
    #[arg(short, long, group = "weight")]
    pub lbs: Option<f64>,

    /// Convert pounds to kilograms
    #[arg(short, long, group = "weight")]
    pub pounds: Option<f64>,
}

#[derive(Args)]
#[group(required = true, multiple = false)]
pub struct TimeOptions {
    /// Converts GMT +0 time to Sao Paulo time
    #[arg(short, long, group = "time")]
    pub gmt: Option<String>,

    /// Converts UTC +0 time to Sao Paulo time
    #[arg(short, long, group = "time")]
    pub utc: Option<String>,
}

#[derive(Args)]
#[group(required = true, multiple = false)]
pub struct LengthOptions {
    /// Convert miles to kilometers
    #[arg(short, long, group = "length")]
    pub miles: Option<f64>,

    /// Convert feet to meters
    #[arg(short, long, group = "length")]
    pub feet: Option<f64>,

    /// Convert inches to centimeters
    #[arg(short, long, group = "length")]
    pub inches: Option<f64>,

    /// Convert yards to meters
    #[arg(short, long, group = "length")]
    pub yards: Option<f64>,
}

#[derive(Parser)]
pub struct AppArguments {
    /// The conversion options
    #[command(subcommand)]
    pub conversion_options: ConversionOptions,

    /// The precision of the output, defaults to 2
    #[arg(short, long, default_value = "2")]
    pub precision: Option<u8>,
}

pub fn get_args() -> &'static AppArguments {
    static INSTANCE: OnceLock<AppArguments> = OnceLock::new();
    INSTANCE.get_or_init(AppArguments::parse)
}
