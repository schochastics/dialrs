#' Check if strings are valid phone numbers
#' @param phone character vector of phone numbers
#' @param country CLDR country code. If empty, tries to parse it from `phone`
#' @return TRUE if valid phone number, FALSE otherwise
#' @export
phone_is_valid <- function(phone, country = "") {
    is_valid_rs(phone, country)
}


#' Return type of phone number
#' @inheritParams phone_is_valid
#' @return character vector of phone number types
#' @export
phone_type <- function(phone, country = "") {
    res <- phone_types_rs(phone, country)
    res[res == ""] <- NA_character_
    res
}

#' Parse phone numbers into specific format
#' @inheritParams phone_is_valid
#' @format character. one of "International", "National", "RFC3966", "E.164"
#' @return character vector of parsed and formatted phone numbers
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
