use extendr_api::prelude::*;
use phonenumber::Mode;
use serde_json;
use std::str::FromStr;

fn strip_hyphens(input: &str) -> String {
    input.replace("-", "")
}

/// Print Rust internal phonenumber for debugging purposes
/// @param input character. a phone number to parse
/// @param country character. CLDR code
/// @return nothing just used for side effects
/// @export
#[extendr]
fn phone_debug_print(input: &str, country: &str) {
    let region = phonenumber::country::Id::from_str(country).ok();
    match phonenumber::parse(region, strip_hyphens(input)) {
        Ok(phone_number) => {
            println!("Parsed phone number successfully: {:#?}", phone_number);
        }
        Err(e) => {
            println!("Failed to parse phone number: {:?}", e);
        }
    }
}

#[extendr]
fn parse_phone_rs_international(phone: Vec<String>, country: &str) -> Vec<String> {
    let region = phonenumber::country::Id::from_str(country).ok();
    phone
        .into_iter()
        .map(
            |input| match phonenumber::parse(region, strip_hyphens(&input)) {
                Ok(number) => number.format().mode(Mode::International).to_string(),
                Err(_e) => String::new(),
            },
        )
        .collect()
}

#[extendr]
fn parse_phone_rs_national(phone: Vec<String>, country: &str) -> Vec<String> {
    let region = phonenumber::country::Id::from_str(country).ok();
    phone
        .into_iter()
        .map(
            |input| match phonenumber::parse(region, strip_hyphens(&input)) {
                Ok(number) => number.format().mode(Mode::National).to_string(),
                Err(_e) => String::new(),
            },
        )
        .collect()
}

#[extendr]
fn parse_phone_rs_rfc3966(phone: Vec<String>, country: &str) -> Vec<String> {
    let region = phonenumber::country::Id::from_str(country).ok();
    phone
        .into_iter()
        .map(
            |input| match phonenumber::parse(region, strip_hyphens(&input)) {
                Ok(number) => number.format().mode(Mode::Rfc3966).to_string(),
                Err(_e) => String::new(),
            },
        )
        .collect()
}

#[extendr]
fn parse_phone_rs_e164(phone: Vec<String>, country: &str) -> Vec<String> {
    let region = phonenumber::country::Id::from_str(country).ok();
    phone
        .into_iter()
        .map(
            |input| match phonenumber::parse(region, strip_hyphens(&input)) {
                Ok(number) => number.format().mode(Mode::E164).to_string(),
                Err(_e) => String::new(),
            },
        )
        .collect()
}

#[extendr]
fn is_valid_rs(phone: Vec<String>, country: &str) -> Vec<bool> {
    let region = phonenumber::country::Id::from_str(country).ok();
    phone
        .iter()
        .map(
            |input| match phonenumber::parse(region, strip_hyphens(input)) {
                Ok(value) => phonenumber::is_valid(&value),
                Err(_e) => false,
            },
        )
        .collect()
}

#[extendr]
fn phone_types_rs(phone: Vec<String>, country: &str) -> Vec<String> {
    let region = phonenumber::country::Id::from_str(country).ok();
    phone
        .into_iter()
        .map(
            |input| match phonenumber::parse(region, strip_hyphens(&input)) {
                Ok(value) => {
                    let ptype = value.number_type(&phonenumber::metadata::DATABASE);
                    serde_json::to_string(&ptype)
                        .unwrap()
                        .trim_matches('"')
                        .to_string()
                }
                Err(_e) => String::new(),
            },
        )
        .collect()
}

#[extendr]
fn phone_country_codes_rs(phone: Vec<String>) -> Vec<u16> {
    phone
        .into_iter()
        .map(
            |input| match phonenumber::parse(None, strip_hyphens(&input)) {
                Ok(number) => number.code().value(),
                Err(_e) => 9999,
            },
        )
        .collect()
}

extendr_module! {
    mod dialrs;
    fn is_valid_rs;
    fn phone_types_rs;
    fn parse_phone_rs_international;
    fn parse_phone_rs_national;
    fn parse_phone_rs_rfc3966;
    fn parse_phone_rs_e164;
    fn phone_country_codes_rs;
    fn phone_debug_print;
}
