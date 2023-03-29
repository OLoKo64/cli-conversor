use clap::{Parser, Subcommand};

#[derive(Subcommand)]
pub enum ConversionOptions {
    /// Convert from one weight unit to metric
    Weight {
        /// Convert lbs to kilograms
        #[arg(short, long)]
        lbs: Option<f64>,

        /// Convert pounds to kilograms
        #[arg(short, long)]
        pounds: Option<f64>,
    },

    /// Convert from one time unit to metric
    Time {
        /// Converts GMT +0 time to Sao Paulo time
        #[arg(short, long)]
        gmt: Option<String>,

        /// Converts UTC +0 time to Sao Paulo time
        #[arg(short, long)]
        utc: Option<String>,
    },

    /// Convert from one length unit to metric
    Length {
        /// Convert miles to kilometers
        #[arg(short, long)]
        miles: Option<f64>,

        /// Convert feet to meters
        #[arg(short, long)]
        feet: Option<f64>,

        /// Convert inches to centimeters
        #[arg(short, long)]
        inches: Option<f64>,

        /// Convert yards to meters
        #[arg(short, long)]
        yards: Option<f64>,
    },
}

#[derive(Parser)]
pub struct Args {
    /// The conversion options
    #[command(subcommand)]
    pub conversion_options: ConversionOptions,

    /// The precision of the output, defaults to 2
    #[arg(short, long, default_value = "2")]
    pub precision: Option<u8>,
}

pub fn parse_args() -> Args {
    Args::parse()
}
