use clap::Parser;

#[derive(clap::Subcommand)]
pub enum ConversionOptions {
    /// Convert from one weight unit to metric
    Weight {
        /// Convert lbs to kilograms
        #[clap(short, long)]
        lbs: Option<f64>,

        /// Convert pounds to kilograms
        #[clap(short, long)]
        pounds: Option<f64>,
    },

    /// Convert from one time unit to metric
    Time {
        /// Converts GMT +0 time to Sao Paulo time
        #[clap(short, long)]
        gmt: Option<String>,

        /// Converts UTC +0 time to Sao Paulo time
        #[clap(short, long)]
        utc: Option<String>,
    },

    /// Convert from one length unit to metric
    Length {
        /// Convert miles to kilometers
        #[clap(short, long)]
        miles: Option<f64>,

        /// Convert feet to meters
        #[clap(short, long)]
        feet: Option<f64>,

        /// Convert inches to centimeters
        #[clap(short, long)]
        inches: Option<f64>,

        /// Convert yards to meters
        #[clap(short, long)]
        yards: Option<f64>,
    },
}

#[derive(clap::Parser)]
pub struct Args {
    /// The conversion options
    #[command(subcommand)]
    pub conversion_options: ConversionOptions,

    /// The precision of the output, defaults to 2
    #[clap(short, long, default_value = "2")]
    pub precision: Option<u8>,
}

pub fn parse_args() -> Args {
    Args::parse()
}
