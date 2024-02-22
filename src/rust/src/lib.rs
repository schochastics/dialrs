use extendr_api::prelude::*;
use phonenumber::PhoneNumber;

/// @export
#[extendr]
fn parse_phone_print(input: &str) {
    match phonenumber::parse(None, input) {
        Ok(phone_number) => {
            println!("Parsed phone number successfully: {:#?}", phone_number);
        }
        Err(e) => {
            println!("Failed to parse phone number: {:?}", e);
        }
    }
}

/// @export
#[extendr]
fn phone_to_r(input: &str, country: &str) -> Robj {
    match phonenumber::parse(None, input) {
        Ok(phone_number) => list!(
            code = phone_number.code().value(),
            national = phone_number.national().value()
        )
        .into(),
        Err(e) => list!(code = r!(NA_INTEGER), national = r!(NA_INTEGER)).into(),
    }
}

/// @export
#[extendr]
fn phones_to_r(inputs: Vec<String>) -> Vec<Robj> {
    inputs
        .into_iter()
        .map(|input| match phonenumber::parse(None, input) {
            Ok(phone_number) => list!(
                code = phone_number.code().value(),
                national = phone_number.national().value()
            )
            .into(),
            Err(_) => list!(code = r!(NA_INTEGER), national = r!(NA_INTEGER)).into(),
        })
        .collect()
}

/// @export
#[extendr]
fn is_valid(input: &str) -> bool {
    let result = phonenumber::parse(None, input);
    match result {
        Ok(value) => phonenumber::is_valid(&value),
        Err(_e) => false,
    }
}

/// @export
#[extendr]
fn phone_type(input: &str) {
    let result = phonenumber::parse(None, input);
    match result {
        Ok(value) => {
            let ptype = value.number_type(&phonenumber::metadata::DATABASE);
            println!("{:?}", ptype)
        }
        Err(_e) => println!("error"),
    }
}

extendr_module! {
    mod dialrs;
    fn is_valid;
    fn parse_phone_print;
    fn phone_to_r;
    fn phones_to_r;
    fn phone_type;
}
