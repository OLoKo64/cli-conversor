use chrono::Timelike;
use std::error::Error;

pub fn lbs_to_kg(lbs: f64) -> f64 {
    lbs * 0.453_592_4
}

pub fn miles_to_km(miles: f64) -> f64 {
    miles * 1.609_344
}

pub fn feet_to_meters(feet: f64) -> f64 {
    feet * 0.304_8
}

pub fn inches_to_centimeters(inches: f64) -> f64 {
    inches * 2.54
}

pub fn yards_to_meters(yards: f64) -> f64 {
    yards * 0.914_4
}

pub fn gmt_to_sao_paulo(gmt: &str) -> Result<String, Box<dyn Error>> {
    let time = chrono::NaiveTime::parse_from_str(gmt, "%H:%M:%S")?;
    let sao_paulo_time =
        chrono::NaiveTime::from_hms_opt(time.hour() + 3, time.minute(), time.second());
    Ok(sao_paulo_time
        .ok_or("Could not add +3 time to supplied time")?
        .format("%H:%M:%S")
        .to_string())
}
