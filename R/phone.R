#' Check if strings are valid phone numbers
#' @param phone character vector of phone numbers
#' @param country CLDR country code. If empty, tries to parse it from `phone`
#' @return TRUE if valid phone number, FALSE otherwise
#' @examples
#' phone_is_valid(fake_phone)
#' @export
phone_is_valid <- function(phone, country = "") {
    is_valid_rs(phone, country)
}


#' Return type of phone number
#' @inheritParams phone_is_valid
#' @return character vector of phone number types
#' @examples
#' phone_type(fake_phone)
#' @export
phone_type <- function(phone, country = "") {
    res <- phone_types_rs(phone, country)
    res[res == ""] <- NA_character_
    res
}

#' Parse phone numbers into specific format
#' @inheritParams phone_is_valid
#' @param format character. one of "International", "National", "RFC3966", "E.164"
#' @return character vector of parsed and formatted phone numbers
#' phone_parse(fake_phone,format = "RFC3966")
#' phone_parse("016012345678",country = "DE",format = "International")
#' @export
phone_parse <- function(phone, country = "", format) {
    format <- match.arg(format, c("International", "National", "RFC3966", "E.164"))
    if (format == "International") {
        res <- parse_phone_rs_international(phone, country)
    } else if (format == "National") {
        res <- parse_phone_rs_national(phone, country)
    } else if (format == "RFC3966") {
        res <- parse_phone_rs_rfc3966(phone, country)
    } else if (format == "E.164") {
        res <- parse_phone_rs_e164(phone, country)
    }
    res[res == ""] <- NA_character_
    res
}

#' Extract country code from phone numbers
#' @inheritParams phone_is_valid
#' @return integer vector of country codes
#' phone_country_code("+4916012345678")
#' @export
phone_country_code <- function(phone) {
    res <- phone_country_codes_rs(phone)
    res[res == 9999] <- NA_integer_
    res
}


#' Extract country from phone numbers in different formats
#' @inheritParams phone_is_valid
#' @param type character. one of "iso2", "iso3", or "country"
#' @return character vector of country codes
#' phone_iso_code("+4916012345678")
# phone_region <- function(phone, type = "iso2") {
#     res <- phone_country_codes_rs(phone)
#     res[res == 9999] <- NA_integer_
#     type <- match.arg(type, c("iso2", "iso3", "country"))
#     countries[[type]][match(res, countries$country_code)]
# }
