use owo_colors::OwoColorize;

use crate::conversions::logic;

pub enum ConversionType<'a> {
    Gmt(&'a str),
    Utc(&'a str),
}

struct SelectedConversionOption<'a> {
    value: &'a str,
    type_: &'a str,
    conversion: fn(&str) -> Result<String, Box<dyn std::error::Error>>,
}

pub fn print_time_conversion(conversion_type: &ConversionType) {
    let SelectedConversionOption {
        value,
        type_,
        conversion,
    } = match conversion_type {
        ConversionType::Gmt(val) => SelectedConversionOption {
            value: val,
            type_: "GMT",
            conversion: logic::gmt_to_sao_paulo,
        },
        ConversionType::Utc(val) => SelectedConversionOption {
            value: val,
            type_: "UTC",
            conversion: logic::utc_to_sao_paulo,
        },
    };

    println!(
        "{val} {type_}+0 is {res} in Sao Paulo time",
        val = value.bright_yellow(),
        type_ = type_,
        res = conversion(value)
            .unwrap_or_else(|_| {
                eprintln!("Could not convert {} to Sao Paulo time", value.bright_red());
                eprintln!(
                    "Please provide a valid {} time {}",
                    type_,
                    "(HH:MM:SS)".bright_yellow()
                );
                std::process::exit(1);
            })
            .bright_green()
    );
}
