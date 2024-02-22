use extendr_api::prelude::*;
use phonenumber::PhoneNumber;
use serde_json;

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

// fn phone_types(inputs: Vec<String>) -> Robj {
//     let types: Vec<Robj> = inputs
//         .into_iter()
//         .map(|input| {
//             let result = phonenumber::parse(None, input);
//             match result {
//                 Ok(value) => {
//                     let ptype = value.number_type(&phonenumber::metadata::DATABASE);
//                     r!(serde_json::to_string(&ptype).unwrap().trim_matches('"'))
//                 }
//                 Err(_e) => r!(NA_STRING),
//             }
//         })
//         .collect();

//     types.into()
// }

/// @export
#[extendr]
fn phone_types(inputs: Vec<String>) -> Vec<String> {
    inputs
        .into_iter()
        .map(|input| match phonenumber::parse(None, &input) {
            Ok(value) => {
                let ptype = value.number_type(&phonenumber::metadata::DATABASE);
                serde_json::to_string(&ptype)
                    .unwrap()
                    .trim_matches('"')
                    .to_string()
            }
            Err(_e) => "NA".to_string(),
        })
        .collect()
}

extendr_module! {
    mod dialrs;
    fn is_valid;
    fn parse_phone_print;
    fn phone_to_r;
    fn phones_to_r;
    fn phone_types;
}
