use owo_colors::OwoColorize;

use crate::{
    args::{self, Args},
    conversions,
};

pub fn output_conversions(args: &Args) {
    match &args.conversion_options {
        args::ConversionOptions::Weight { lbs, pounds } => {
            if let Some(lbs) = lbs {
                let precision = args.precision.unwrap_or_default();
                println!(
                    "{} lbs is {:.precision$} kg",
                    lbs.bright_yellow(),
                    conversions::lbs_to_kg(*lbs).bright_green(),
                    precision = precision as usize
                );
            } else if let Some(pounds) = pounds {
                let precision = args.precision.unwrap_or_default();
                println!(
                    "{} pounds is {:.precision$} kg",
                    pounds.bright_yellow(),
                    conversions::lbs_to_kg(*pounds).bright_green(),
                    precision = precision as usize
                );
            } else {
                println!("Please provide a value to convert");
            }
        }
        args::ConversionOptions::Time { gmt, utc } => {
            if let Some(gmt) = gmt {
                println!(
                    "{} GMT+0 is {} in Sao Paulo time",
                    gmt.bright_yellow(),
                    conversions::gmt_to_sao_paulo(gmt)
                        .unwrap_or_else(|_| {
                            eprintln!("Could not convert {} to Sao Paulo time", gmt.bright_red());
                            eprintln!(
                                "Please provide a valid GMT time {}",
                                "(HH:MM:SS)".bright_yellow()
                            );
                            std::process::exit(1);
                        })
                        .bright_green()
                );
            } else if let Some(utc) = utc {
                println!(
                    "{} UTC+0 is {} in Sao Paulo time",
                    utc.bright_yellow(),
                    conversions::gmt_to_sao_paulo(utc)
                        .unwrap_or_else(|_| {
                            eprintln!("Could not convert {} to Sao Paulo time", utc.bright_red());
                            eprintln!(
                                "Please provide a valid UTC time {}",
                                "(HH:MM:SS)".bright_yellow()
                            );
                            std::process::exit(1);
                        })
                        .bright_green()
                );
            } else {
                println!("Please provide a value to convert");
            }
        }
        args::ConversionOptions::Length {
            miles,
            feet,
            inches,
            yards,
        } => {
            if let Some(miles) = miles {
                let precision = args.precision.unwrap_or_default();
                println!(
                    "{} miles is {:.precision$} km",
                    miles.bright_yellow(),
                    conversions::miles_to_km(*miles).bright_green(),
                    precision = precision as usize
                );
            } else if let Some(feet) = feet {
                let precision = args.precision.unwrap_or_default();
                println!(
                    "{} feet is {:.precision$} meters",
                    feet.bright_yellow(),
                    conversions::feet_to_meters(*feet).bright_green(),
                    precision = precision as usize
                );
            } else if let Some(inches) = inches {
                let precision = args.precision.unwrap_or_default();
                println!(
                    "{} inches is {:.precision$} centimeters",
                    inches.bright_yellow(),
                    conversions::inches_to_centimeters(*inches).bright_green(),
                    precision = precision as usize
                );
            } else if let Some(yards) = yards {
                let precision = args.precision.unwrap_or_default();
                println!(
                    "{} yards is {:.precision$} meters",
                    yards.bright_yellow(),
                    conversions::yards_to_meters(*yards).bright_green(),
                    precision = precision as usize
                );
            } else {
                println!("Please provide a value to convert");
            }
        }
    }
}
