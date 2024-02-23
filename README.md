
<!-- README.md is generated from README.Rmd. Please edit that file -->

# dialrs <img src="man/figures/logo.png" align="right" height="139" alt="" />

<!-- badges: start -->

[![R-CMD-check](https://github.com/schochastics/dialrs/actions/workflows/R-CMD-check.yaml/badge.svg)](https://github.com/schochastics/dialrs/actions/workflows/R-CMD-check.yaml)
[![CRAN
status](https://www.r-pkg.org/badges/version/dialrs)](https://CRAN.R-project.org/package=dialrs)
<!-- badges: end -->

dialrs is an experimental R package to parse phone numbers using the
Rust crate [phonenumber](https://crates.io/crates/phonenumber).

## Installation

You can install the development version of dialrs like so:

``` r
remotes::install_github("schochastics/dialrs")
pak::pak("schochastics/dialrs")
```

## Example

``` r
library(dialrs)
numbers <- c("+1 202-555-0173", "+33-1-22-33-44-55", "1 800 FLOWERS")
phone_is_valid(numbers)
#> [1]  TRUE  TRUE FALSE
phone_type(numbers)
#> [1] "fixed_line_or_mobile" "fixed_line"           NA
phone_country_code(numbers)
#> [1]  1 33 NA
phone_iso_code(numbers)
#> [1] "CA" "FR" NA
phone_parse(numbers, format = "International")
#> [1] "+1 202-555-0173"   "+33 1 22 33 44 55" NA
phone_parse(numbers, format = "RFC3966")
#> [1] "tel:+1-202-555-0173"   "tel:+33-1-22-33-44-55" NA
phone_parse(numbers, format = "E.164")
#> [1] "+12025550173" "+33122334455" NA
```

## Note

There is already an R package for that, called
[dialr](https://github.com/socialresearchcentre/dialr), which wraps
Google’s [libphonenumber](https://github.com/google/libphonenumber) Java
library.
