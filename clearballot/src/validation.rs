pub const PHONE_NUMBER_LENGTH: usize = 10;
pub const UNIQUE_ID_LENGTH: usize = 9;
pub const MIN_NAME_LENGTH: usize = 2;
pub const MAX_NAME_LENGTH: usize = 50;
pub const MIN_ADDRESS_LENGTH: usize = 5;
pub const MAX_ADDRESS_LENGTH: usize = 500;
pub const MIN_AGE: u32 = 18;
pub const MAX_AGE: u32 = 120;

#[derive(Debug, PartialEq)]
pub enum ValidationError {
    Empty { field: String },
    TooShort { field: String, min: usize },
    TooLong { field: String, max: usize },
    NotAlphabetic { field: String },
    WrongLength { field: String, expected: usize },
    NotNumeric { field: String },
    TooYoung { age: u32 },
    TooOld { age: u32 },
}

#[derive(Debug, PartialEq)]
pub struct ValidVoter {
    pub first_name: String,
    pub last_name: String,
    pub address: String,
    pub phone_number: String,
    pub unique_id: String,
    pub age: u32,
}

pub fn validate_voter(
    first_name: &str,
    last_name: &str,
    phone_number: &str,
    address: &str,
    unique_id: &str,
    age: &str,
) -> Result<ValidVoter, ValidationError> {
    let first_name = validate_name(first_name, "First name")?;
    let last_name = validate_name(last_name, "Last name")?;
    let phone_number = validate_phone_number(phone_number)?;
    let address = validate_address(address)?;
    let unique_id = validate_unique_id(unique_id)?;
    let age = validate_age(age)?;

    Ok(ValidVoter {
        first_name,
        last_name,
        address,
        phone_number,
        unique_id,
        age,
    })
}

pub fn validate_name(name: &str, field: &str) -> Result<String, ValidationError> {
    let name = validate_length(name, field, MIN_NAME_LENGTH, MAX_NAME_LENGTH)?;

    if !name.chars().all(|c| c.is_alphabetic() || c == ' ') {
        return Err(ValidationError::NotAlphabetic {
            field: field.to_string(),
        });
    }

    Ok(name)
}

pub fn validate_address(address: &str) -> Result<String, ValidationError> {
    validate_length(address, "Address", MIN_ADDRESS_LENGTH, MAX_ADDRESS_LENGTH)
}

fn validate_length(
    value: &str,
    field: &str,
    min: usize,
    max: usize,
) -> Result<String, ValidationError> {
    let trimmed = value.trim();

    if trimmed.is_empty() {
        return Err(ValidationError::Empty {
            field: field.to_string(),
        });
    }

    let length = trimmed.chars().count();

    if length < min {
        return Err(ValidationError::TooShort {
            field: field.to_string(),
            min,
        });
    }

    if length > max {
        return Err(ValidationError::TooLong {
            field: field.to_string(),
            max,
        });
    }

    Ok(trimmed.to_string())
}

pub fn validate_phone_number(phone: &str) -> Result<String, ValidationError> {
    validate_digits(phone, "Phone number", PHONE_NUMBER_LENGTH)
}

pub fn validate_unique_id(uniqueid: &str) -> Result<String, ValidationError> {
    validate_digits(uniqueid, "Unique ID", UNIQUE_ID_LENGTH)
}

fn validate_digits(value: &str, field: &str, expected: usize) -> Result<String, ValidationError> {
    let trimmed = value.trim();

    if trimmed.is_empty() {
        return Err(ValidationError::Empty {
            field: field.to_string(),
        });
    }

    if trimmed.chars().count() != expected {
        return Err(ValidationError::WrongLength {
            field: field.to_string(),
            expected,
        });
    }

    if !trimmed.chars().all(|c| c.is_ascii_digit()) {
        return Err(ValidationError::NotNumeric {
            field: field.to_string(),
        });
    }

    Ok(trimmed.to_string())
}

pub fn validate_age(age: &str) -> Result<u32, ValidationError> {
    let trimmed = age.trim();

    if trimmed.is_empty() {
        return Err(ValidationError::Empty {
            field: "Age".to_string(),
        });
    }

    if !trimmed.chars().all(|c| c.is_ascii_digit()) {
        return Err(ValidationError::NotNumeric {
            field: "Age".to_string(),
        });
    }

    let age: u32 = trimmed.parse().map_err(|_| ValidationError::NotNumeric {
        field: "Age".to_string(),
    })?;

    if age < MIN_AGE {
        return Err(ValidationError::TooYoung { age });
    }

    if age > MAX_AGE {
        return Err(ValidationError::TooOld { age });
    }

    Ok(age)
}

#[cfg(test)]
mod tests {
    use super::*;

    // ---- validate_name ----

    #[test]
    fn accepts_a_normal_name() {
        assert_eq!(
            validate_name("  Ziyad  ", "First name"),
            Ok("Ziyad".to_string())
        );
    }

    #[test]
    fn accepts_name_with_space() {
        assert_eq!(
            validate_name("John Doe", "First name"),
            Ok("John Doe".to_string())
        );
    }

    #[test]
    fn accepts_accented_names() {
        assert_eq!(validate_name("José", "First name"), Ok("José".to_string()));
    }

    #[test]
    fn accepts_minimum_length_name() {
        assert_eq!(validate_name("Al", "First name"), Ok("Al".to_string()));
    }

    #[test]
    fn accepts_maximum_length_name() {
        let name = "A".repeat(MAX_NAME_LENGTH);
        assert_eq!(validate_name(&name, "First name"), Ok(name));
    }

    #[test]
    fn rejects_empty_name() {
        assert_eq!(
            validate_name("   ", "First name"),
            Err(ValidationError::Empty {
                field: "First name".to_string(),
            })
        );
    }

    #[test]
    fn rejects_single_character_names_even_non_ascii() {
        assert_eq!(
            validate_name("李", "First name"),
            Err(ValidationError::TooShort {
                field: "First name".to_string(),
                min: MIN_NAME_LENGTH,
            })
        );
    }

    #[test]
    fn rejects_name_over_maximum_length() {
        let name = "A".repeat(MAX_NAME_LENGTH + 1);
        assert_eq!(
            validate_name(&name, "First name"),
            Err(ValidationError::TooLong {
                field: "First name".to_string(),
                max: MAX_NAME_LENGTH,
            })
        );
    }

    #[test]
    fn rejects_digits_in_a_name() {
        assert_eq!(
            validate_name("Ziyad99", "First name"),
            Err(ValidationError::NotAlphabetic {
                field: "First name".to_string(),
            })
        );
    }

    #[test]
    fn rejects_tab_inside_a_name() {
        // Only a plain space is allowed between words, not other whitespace
        assert_eq!(
            validate_name("John\tDoe", "First name"),
            Err(ValidationError::NotAlphabetic {
                field: "First name".to_string(),
            })
        );
    }

    // ---- validate_phone_number ----

    #[test]
    fn accepts_ten_digit_phone_number() {
        assert_eq!(
            validate_phone_number(" 1234567890 "),
            Ok("1234567890".to_string())
        );
    }

    #[test]
    fn rejects_empty_phone_number() {
        assert_eq!(
            validate_phone_number(""),
            Err(ValidationError::Empty {
                field: "Phone number".to_string(),
            })
        );
    }

    #[test]
    fn rejects_phone_number_one_digit_short() {
        assert_eq!(
            validate_phone_number("123456789"),
            Err(ValidationError::WrongLength {
                field: "Phone number".to_string(),
                expected: PHONE_NUMBER_LENGTH,
            })
        );
    }

    #[test]
    fn rejects_phone_number_one_digit_long() {
        assert_eq!(
            validate_phone_number("12345678901"),
            Err(ValidationError::WrongLength {
                field: "Phone number".to_string(),
                expected: PHONE_NUMBER_LENGTH,
            })
        );
    }

    #[test]
    fn rejects_non_numeric_phone_number() {
        assert_eq!(
            validate_phone_number("abcdefghij"),
            Err(ValidationError::NotNumeric {
                field: "Phone number".to_string(),
            })
        );
    }

    #[test]
    fn rejects_formatted_phone_number() {
        assert_eq!(
            validate_phone_number("123-456-78"),
            Err(ValidationError::NotNumeric {
                field: "Phone number".to_string(),
            })
        );
    }

    // ---- validate_unique_id ----

    #[test]
    fn accepts_nine_digit_unique_id() {
        assert_eq!(validate_unique_id("123456789"), Ok("123456789".to_string()));
    }

    #[test]
    fn keeps_leading_zeros_in_unique_id() {
        assert_eq!(validate_unique_id("000123456"), Ok("000123456".to_string()));
    }

    #[test]
    fn rejects_empty_unique_id() {
        assert_eq!(
            validate_unique_id(""),
            Err(ValidationError::Empty {
                field: "Unique ID".to_string()
            })
        );
    }

    #[test]
    fn rejects_unique_id_one_digit_short() {
        assert_eq!(
            validate_unique_id("12345678"),
            Err(ValidationError::WrongLength {
                field: "Unique ID".to_string(),
                expected: UNIQUE_ID_LENGTH,
            })
        );
    }

    #[test]
    fn rejects_unique_id_one_digit_long() {
        assert_eq!(
            validate_unique_id("1234567890"),
            Err(ValidationError::WrongLength {
                field: "Unique ID".to_string(),
                expected: UNIQUE_ID_LENGTH,
            })
        );
    }

    #[test]
    fn rejects_non_numeric_unique_id() {
        assert_eq!(
            validate_unique_id("abcdefghi"),
            Err(ValidationError::NotNumeric {
                field: "Unique ID".to_string(),
            })
        );
    }

    #[test]
    fn rejects_full_width_digits_in_unique_id() {
        // Nine characters, and char::is_numeric() would accept them
        assert_eq!(
            validate_unique_id("１２３４５６７８９"),
            Err(ValidationError::NotNumeric {
                field: "Unique ID".to_string(),
            })
        );
    }

    // ---- validate_age ----

    #[test]
    fn accepts_minimum_age() {
        assert_eq!(validate_age("18"), Ok(18));
    }

    #[test]
    fn accepts_maximum_age() {
        assert_eq!(validate_age("120"), Ok(120));
    }

    #[test]
    fn accepts_age_with_surrounding_whitespace() {
        assert_eq!(validate_age(" 25 "), Ok(25));
    }

    #[test]
    fn accepts_age_with_leading_zero() {
        assert_eq!(validate_age("018"), Ok(18));
    }

    #[test]
    fn rejects_one_below_minimum() {
        assert_eq!(
            validate_age("17"),
            Err(ValidationError::TooYoung { age: 17 })
        );
    }

    #[test]
    fn rejects_one_above_maximum() {
        assert_eq!(
            validate_age("121"),
            Err(ValidationError::TooOld { age: 121 })
        );
    }

    #[test]
    fn rejects_empty_age() {
        assert_eq!(
            validate_age(""),
            Err(ValidationError::Empty {
                field: "Age".to_string(),
            })
        );
    }

    #[test]
    fn rejects_non_numeric_age() {
        assert_eq!(
            validate_age("abc"),
            Err(ValidationError::NotNumeric {
                field: "Age".to_string(),
            })
        );
    }

    #[test]
    fn rejects_age_with_plus_sign() {
        // parse::<u32>() accepts "+18", so only the digit check catches this
        assert_eq!(
            validate_age("+18"),
            Err(ValidationError::NotNumeric {
                field: "Age".to_string(),
            })
        );
    }

    #[test]
    fn rejects_age_too_large_for_u32() {
        // Passes the digit check, so this is the only test that reaches map_err
        assert_eq!(
            validate_age("99999999999"),
            Err(ValidationError::NotNumeric {
                field: "Age".to_string(),
            })
        );
    }

    // ---- validate_address ----

    #[test]
    fn accepts_a_normal_address() {
        assert_eq!(
            validate_address(" 123 Main St. "),
            Ok("123 Main St.".to_string())
        );
    }

    #[test]
    fn accepts_minimum_length_address() {
        let address = "A".repeat(MIN_ADDRESS_LENGTH);
        assert_eq!(validate_address(&address), Ok(address));
    }

    #[test]
    fn accepts_maximum_length_address() {
        let address = "A".repeat(MAX_ADDRESS_LENGTH);
        assert_eq!(validate_address(&address), Ok(address));
    }

    #[test]
    fn rejects_empty_address() {
        assert_eq!(
            validate_address("   "),
            Err(ValidationError::Empty {
                field: "Address".to_string(),
            })
        );
    }

    #[test]
    fn rejects_address_too_short() {
        let address = "A".repeat(MIN_ADDRESS_LENGTH - 1);
        assert_eq!(
            validate_address(&address),
            Err(ValidationError::TooShort {
                field: "Address".to_string(),
                min: MIN_ADDRESS_LENGTH,
            })
        );
    }

    #[test]
    fn rejects_address_too_long() {
        let address = "A".repeat(MAX_ADDRESS_LENGTH + 1);
        assert_eq!(
            validate_address(&address),
            Err(ValidationError::TooLong {
                field: "Address".to_string(),
                max: MAX_ADDRESS_LENGTH,
            })
        );
    }

    #[test]
    fn accepts_maximum_length_address_with_non_ascii() {
        // 500 characters but 1000 bytes: fails if the length check uses .len()
        let address = "é".repeat(MAX_ADDRESS_LENGTH);
        assert_eq!(validate_address(&address), Ok(address));
    }

    #[test]
    fn accepts_valid_voter() {
        assert_eq!(
            validate_voter(
                " Ziyad ",
                "Shaikh",
                "1234567890",
                "123 Main St",
                "000123456",
                "25"
            ),
            Ok(ValidVoter {
                first_name: "Ziyad".to_string(),
                last_name: "Shaikh".to_string(),
                phone_number: "1234567890".to_string(),
                address: "123 Main St".to_string(),
                unique_id: "000123456".to_string(),
                age: 25,
            })
        );
    }

    #[test]
    fn reports_the_first_invalid_field() {
        // Both the phone number and the age are invalid; the phone number is checked first
        assert_eq!(
            validate_voter("Ziyad", "Shaikh", "123", "123 Main St", "123456789", "17"),
            Err(ValidationError::WrongLength {
                field: "Phone number".to_string(),
                expected: PHONE_NUMBER_LENGTH,
            })
        );
    }
}
