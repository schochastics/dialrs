test_that("phone_is_valid works", {
    expect_true(phone_is_valid("+1 202-555-0173"))
    expect_true(phone_is_valid("+1 800 FLOWERS"))
    expect_false(phone_is_valid("+49160123456"))
    expect_error(phone_is_valid(NULL))
    expect_error(phone_is_valid(NA_character_))
})

test_that("phone_type works", {
    expected <- c("unknown", "mobile", "mobile", "fixed_line_or_mobile")
    actual <- phone_type(c(0123, "0404 753 123", "61410123817", "+12015550123"), "AU")
    expect_true(all(expected == actual))
    expect_error(phone_type(NULL))
    expect_error(phone_type(NA_character_))
})

test_that("phone_parse works", {
    expect_equal(phone_parse("+1 202-555-0173", format = "International"), "+1 202-555-0173")
    expect_equal(phone_parse("+1 202-555-0173", format = "National"), "(202) 555-0173")
    expect_equal(phone_parse("+1 202-555-0173", format = "RFC3966"), "tel:+1-202-555-0173")
    expect_equal(phone_parse("+1 202-555-0173", format = "E.164"), "+12025550173")
    expect_error(phone_parse("+1 202-555-0173", format = "Unknown"))
})

test_that("country codes work", {
    expect_equal(phone_country_code("+1 202-555-0173"), 1)
    expect_equal(phone_country_code("+49 16012345678"), 49)
    # expect_equal(phone_iso_code("+1 202-555-0173", type = "iso2"), "CA")
    # expect_equal(phone_iso_code("+1 202-555-0173", type = "iso3"), "CAN")
    # expect_equal(phone_iso_code("+1 202-555-0173", type = "country"), "Canada")
})
