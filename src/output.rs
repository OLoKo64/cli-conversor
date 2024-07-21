use owo_colors::OwoColorize;

use crate::{
    args::{self, get_args, ConversionOptions},
    conversions::{self},
};

pub fn output_conversions() {
    let printer = get_printer();

    printer.print();
}

fn get_printer() -> Box<dyn ConversionPrinter> {
    let args = args::get_args();

    match &args.conversion_options {
        ConversionOptions::Weight { value } => {
            if let Some(lbs) = value.lbs {
                Box::new(Lbs(lbs))
            } else if let Some(pounds) = value.pounds {
                Box::new(Pounds(pounds))
            } else {
                panic!("Please provide a value to convert");
            }
        }
        ConversionOptions::Time { value } => {
            if let Some(ref gmt) = value.gmt {
                Box::new(TimeGmt(gmt))
            } else if let Some(ref utc) = value.utc {
                Box::new(TimeUtc(utc))
            } else {
                panic!("Please provide a value to convert");
            }
        }
        ConversionOptions::Length { value } => {
            if let Some(miles) = value.miles {
                Box::new(Miles(miles))
            } else if let Some(feet) = value.feet {
                Box::new(Feet(feet))
            } else if let Some(inches) = value.inches {
                Box::new(Inches(inches))
            } else if let Some(yards) = value.yards {
                Box::new(Yards(yards))
            } else {
                panic!("Please provide a value to convert");
            }
        }
    }
}

trait ConversionPrinter {
    fn print(&self);
}

struct Lbs(f64);
struct Pounds(f64);
struct Miles(f64);
struct Feet(f64);
struct Inches(f64);
struct Yards(f64);
struct TimeGmt<'a>(&'a str);
struct TimeUtc<'a>(&'a str);

impl ConversionPrinter for Lbs {
    fn print(&self) {
        let precision = get_args().precision.unwrap_or_default();

        let res = conversions::lbs_to_kg(self.0);

        println!(
            "{val} lbs is {res:.precision$} kilograms",
            val = self.0,
            precision = precision as usize,
            res = res,
        );
    }
}

impl ConversionPrinter for Pounds {
    fn print(&self) {
        let precision = get_args().precision.unwrap_or_default();

        let res = conversions::lbs_to_kg(self.0);

        println!(
            "{val} pounds is {res:.precision$} kilograms",
            val = self.0,
            precision = precision as usize,
            res = res,
        );
    }
}

impl ConversionPrinter for Miles {
    fn print(&self) {
        let precision = get_args().precision.unwrap_or_default();

        let res = conversions::miles_to_km(self.0);

        println!(
            "{val} miles is {res:.precision$} kilometers",
            val = self.0,
            precision = precision as usize,
            res = res,
        );
    }
}

impl ConversionPrinter for Feet {
    fn print(&self) {
        let precision = get_args().precision.unwrap_or_default();

        let res = conversions::feet_to_meters(self.0);

        println!(
            "{val} feet is {res:.precision$} meters",
            val = self.0,
            precision = precision as usize,
            res = res,
        );
    }
}

impl ConversionPrinter for Inches {
    fn print(&self) {
        let precision = get_args().precision.unwrap_or_default();

        let res = conversions::inches_to_centimeters(self.0);

        println!(
            "{val} inches is {res:.precision$} centimeters",
            val = self.0,
            precision = precision as usize,
            res = res,
        );
    }
}

impl ConversionPrinter for Yards {
    fn print(&self) {
        let precision = get_args().precision.unwrap_or_default();

        let res = conversions::yards_to_meters(self.0);

        println!(
            "{val} yards is {res:.precision$} meters",
            val = self.0,
            precision = precision as usize,
            res = res,
        );
    }
}

impl<'a> ConversionPrinter for TimeGmt<'a> {
    fn print(&self) {
        let type_ = "GMT";

        println!(
            "{val} {type_}+0 is {res} in Sao Paulo time",
            val = self.0.bright_yellow(),
            type_ = type_,
            res = conversions::gmt_to_sao_paulo(self.0)
                .unwrap_or_else(|_| {
                    eprintln!(
                        "Could not convert {} to Sao Paulo time",
                        self.0.bright_red()
                    );
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
}

impl<'a> ConversionPrinter for TimeUtc<'a> {
    fn print(&self) {
        let type_ = "UTC";

        println!(
            "{val} {type_}+0 is {res} in Sao Paulo time",
            val = self.0.bright_yellow(),
            type_ = type_,
            res = conversions::utc_to_sao_paulo(self.0)
                .unwrap_or_else(|_| {
                    eprintln!(
                        "Could not convert {} to Sao Paulo time",
                        self.0.bright_red()
                    );
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
}
