use crate::{
    args::{get_args, ConversionOptions},
    conversions::{
        print_general_conversion, print_time_conversion, GeneralConversionType, TimeConversionType,
    },
};

pub fn output_conversions() {
    let args = get_args();
    match &args.conversion_options {
        ConversionOptions::Weight { value } => {
            if let Some(lbs) = value.lbs {
                print_general_conversion(&GeneralConversionType::Lbs(lbs));
            } else if let Some(pounds) = value.pounds {
                print_general_conversion(&GeneralConversionType::Pounds(pounds));
            } else {
                panic!("Please provide a value to convert");
            }
        }
        ConversionOptions::Time { value } => {
            if let Some(ref gmt) = value.gmt {
                print_time_conversion(&TimeConversionType::Gmt(gmt));
            } else if let Some(ref utc) = value.utc {
                print_time_conversion(&TimeConversionType::Utc(utc));
            } else {
                panic!("Please provide a value to convert");
            }
        }
        ConversionOptions::Length { value } => {
            if let Some(miles) = value.miles {
                print_general_conversion(&GeneralConversionType::Miles(miles));
            } else if let Some(feet) = value.feet {
                print_general_conversion(&GeneralConversionType::Feet(feet));
            } else if let Some(inches) = value.inches {
                print_general_conversion(&GeneralConversionType::Inches(inches));
            } else if let Some(yards) = value.yards {
                print_general_conversion(&GeneralConversionType::Yards(yards));
            } else {
                panic!("Please provide a value to convert");
            }
        }
    }
}
