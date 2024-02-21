use extendr_api::prelude::*;
use phonenumber::{Mode, PhoneNumber};

/// Parse phone number
// fn parse_phone(input: &str) -> PhoneNumber {
// }

/// @export
#[extendr]
fn is_valid(input: &str) -> bool {
    let result = phonenumber::parse(None, input);
    match result {
        Ok(value) => phonenumber::is_valid(&value),
        Err(e) => false,
    }
}

/// @export
#[extendr]
fn extract_phone(input: &str) {
    let result = phonenumber::parse::extract(input);
    match result {
        Ok((remainder, parsed)) => {
            println!("Parsed output: {}", parsed); // Access the parsed output
            println!("Remaining input: {}", remainder); // Access the remaining input
        }
        Err(error) => println!("Error encountered: {:?}", error),
    }
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod dialrs;
    fn is_valid;
    fn extract_phone;
}
