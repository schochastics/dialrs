fake_phone <- c(
    "+1 202-555-0173", # US number with space and dash
    "+442071838750", # UK number, standard
    "+33-1-22-33-44-55", # France number with dashes
    "+49891234567", # Germany number, no separator
    "+819012345678", # Japan number, long
    "+61234567890", # Australia number, standard
    "+5511955551234", # Brazil number, mobile
    "+8613800138000", # China number, mobile
    "+7 499 123-45-67", # Russia number with spaces and dash
    "+27123456789", # South Africa, standard
    "+911234567890", # India, long
    "+39 02 1234 1234", # Italy, with spaces
    "+34555555555", # Spain, mobile
    "+46701234567", # Sweden, mobile
    "+48221234567", # Poland, standard
    "+302101234567", # Greece, standard
    "+902123456789", # Turkey, standard
    "+971501234567", # UAE, mobile
    "+201001234567", # Egypt, mobile
    "+525512345678", # Mexico, mobile
    "+31 20 123 4567", # Netherlands, with spaces
    "+41441234567", # Switzerland, standard
    "+4732123456", # Norway, standard
    "+358 20 1234567", # Finland, with space
    "+3721234567", # Estonia, standard
    "+64211234567", # New Zealand, mobile
    "+601123456789", # Malaysia, mobile
    "+6561234567", # Singapore, standard
    "+2348012345678", # Nigeria, mobile
    "+380501234567", # Ukraine, mobile
    "+541112345678", # Argentina, standard
    "+56212345678", # Chile, standard
    "+573123456789", # Colombia, mobile
    "+30210", # Greece, too short
    "+123", # Invalid country code
    "1234567890", # No country code
    "+9991234567", # Unassigned country code
    "+1 800 FLOWERS", # Alphanumeric
    "+447911123456", # UK, mobile
    "+448001234567", # UK, free phone
    "+441632960961", # UK, standard
    "+12024561111", # US, New York
    "+12125551212", # US, fake number
    "+358401234567", # Finland, mobile
    "+7 800 123 4567", # Russia, free phone
    "+85212345678", # Hong Kong, standard
    "+27831234567", # South Africa, mobile
    "+918032761234", # India, standard
    "+551143218999", # Brazil, Sao Paulo
    "+33123456789", # France, standard
    "+1-800-123-4567", # US toll-free
    "+44 20 7946 0958", # UK, London
    "+49-89-636-48018", # Germany, Munich
    "+81-3-3477-0111", # Japan, Tokyo
    "+33 1 42 68 53 00", # France, Paris
    "+61-2-9876-5432", # Australia, Sydney
    "+86 10 6552 9988", # China, Beijing
    "+91-22-2778-4040", # India, Mumbai
    "+7 495 697-0349", # Russia, Moscow
    "+55 21 2505-6700", # Brazil, Rio de Janeiro
    "+27 21 401 9111", # South Africa, Cape Town
    "+34 91 398 43 00", # Spain, Madrid
    "+39 06 6988 4466", # Italy, Rome
    "+46 8 5792 9100", # Sweden, Stockholm
    "+48 22 594 19 99", # Poland, Warsaw
    "+1-246-436-9000", # Barbados
    "+20 2 3377 3222", # Egypt, Cairo
    "+51 1 203-5000", # Peru, Lima
    "+64 9-306 3000", # New Zealand, Auckland
    "+90 212 444 0 145", # Turkey, Istanbul
    "+358 9 310 1691", # Finland, Helsinki
    "+82 2 2001-0114", # South Korea, Seoul
    "+62 21 5085 1975", # Indonesia, Jakarta
    "+974 4499 7499", # Qatar, Doha
    "+966 11 488 3800", # Saudi Arabia, Riyadh
    "+31 20 794 0800", # Netherlands, Amsterdam
    "+41 44 212 44 88", # Switzerland, Zurich
    "+54 11 4319 1111", # Argentina, Buenos Aires
    "+603-2176 8555", # Malaysia, Kuala Lumpur
    "+52 55 5227 4600", # Mexico, Mexico City
    "+973 1723 8000", # Bahrain, Manama
    "+375 17 203-34-45", # Belarus, Minsk
    "+47 22 44 85 50", # Norway, Oslo
    "+43 1 50110-0", # Austria, Vienna
    "+359 2 981 981", # Bulgaria, Sofia
    "+385 1 4563 111", # Croatia, Zagreb
    "+357 22 867000", # Cyprus, Nicosia
    "+420 224 491 111", # Czech Republic, Prague
    "+372 612 3456", # Estonia, Tallinn
    "+30 21 0330 1000", # Greece, Athens
    "+36 1 3 111 111", # Hungary, Budapest
    "+354 545 1100", # Iceland, Reykjavik
    "+353 1 222 2222", # Ireland, Dublin
    "+371 6709 7000", # Latvia, Riga
    "+370 5 266 6666", # Lithuania, Vilnius
    "+352 22 999 999", # Luxembourg
    "+356 2124 1234", # Malta, Valletta
    "+377 98 98 88 88", # Monaco
    "+64 4-472 2249", # New Zealand, Wellington
    "+507 507-7200" # Panama, Panama City
)
usethis::use_data(fake_phone, overwrite = TRUE)

# library(rvest)
# library(dplyr)
# url <- "https://countrycode.org/"
# doc <- read_html(url)
# countries <- html_table(doc)[[1]] |>
#     janitor::clean_names() |>
#     select(country, country_code, iso_codes) |>
#     tidyr::separate_wider_delim(iso_codes, " / ", names = c("iso2", "iso3")) |>
#     as.data.frame()

# usethis::use_data(countries, internal = TRUE, overwrite = TRUE)
