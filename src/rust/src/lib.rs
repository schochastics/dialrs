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
fn parse_phone_r(input: &str, country: &str) -> Robj {
    match phonenumber::parse(None, input) {
        Ok(phone_number) => list!(
            code = phone_number.code().value(),
            national = phone_number.national().value(),
            zeros = phone_number.national().zeros()
        )
        .into(),
        Err(e) => list!(code = "", national = "", zeros = "").into(),
    }
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

extendr_module! {
    mod dialrs;
    fn is_valid;
    fn parse_phone_print;
    fn parse_phone_r;
}
