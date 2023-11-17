use owo_colors::OwoColorize;

use crate::{args::get_args, conversions};

pub enum ConversionType {
    Lbs(f64),
    Pounds(f64),

    Miles(f64),
    Feet(f64),
    Inches(f64),
    Yards(f64),
}

struct SelectedConversionOption<'a> {
    value: f64,
    lhs: &'a str,
    rhs: &'a str,
    conversion: fn(f64) -> f64,
}

pub fn print_general_conversion(conversion_type: &ConversionType) {
    let precision = get_args().precision.unwrap_or_default();
    let SelectedConversionOption {
        value,
        lhs,
        rhs,
        conversion,
    } = match conversion_type {
        ConversionType::Miles(val) => SelectedConversionOption {
            value: *val,
            lhs: "miles",
            rhs: "kilometers",
            conversion: conversions::miles_to_km,
        },
        ConversionType::Feet(val) => SelectedConversionOption {
            value: *val,
            lhs: "feet",
            rhs: "meters",
            conversion: conversions::feet_to_meters,
        },
        ConversionType::Inches(val) => SelectedConversionOption {
            value: *val,
            lhs: "inches",
            rhs: "centimeters",
            conversion: conversions::inches_to_centimeters,
        },
        ConversionType::Yards(val) => SelectedConversionOption {
            value: *val,
            lhs: "yards",
            rhs: "meters",
            conversion: conversions::yards_to_meters,
        },
        ConversionType::Lbs(val) => SelectedConversionOption {
            value: *val,
            lhs: "lbs",
            rhs: "kilograms",
            conversion: conversions::lbs_to_kg,
        },
        ConversionType::Pounds(val) => SelectedConversionOption {
            value: *val,
            lhs: "pounds",
            rhs: "kilograms",
            conversion: conversions::lbs_to_kg,
        },
    };

    println!(
        "{val} {lhs} is {res:.precision$} {rhs}",
        val = value.bright_yellow(),
        lhs = lhs,
        precision = precision as usize,
        res = conversion(value).bright_green(),
        rhs = rhs,
    );
}
