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
}

pub fn validate_name(name: &str, field: &str) -> Result<String, ValidationError> {
    let trimmed = name.trim();

    if trimmed.is_empty() {
        return Err(ValidationError::Empty {
            field: field.to_string(),
        });
    }

    let length = trimmed.chars().count();

    if length < MIN_NAME_LENGTH {
        return Err(ValidationError::TooShort {
            field: field.to_string(),
            min: MIN_NAME_LENGTH,
        });
    }

    if length > MAX_NAME_LENGTH {
        return Err(ValidationError::TooLong {
            field: field.to_string(),
            max: MAX_NAME_LENGTH,
        });
    }

    if !trimmed.chars().all(|c| c.is_alphabetic() || c == ' ') {
        return Err(ValidationError::NotAlphabetic {
            field: field.to_string(),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_a_normal_name() {
        assert_eq!(
            validate_name("  Ziyad  ", "First name"),
            Ok("Ziyad".to_string())
        );
    }

    #[test]
    fn rejects_digits_in_a_name() {
        assert!(validate_name("Ziyad99", "First name").is_err());
    }

    #[test]
    fn phone_must_be_ten_digits() {
        assert!(validate_phone_number("1234567890").is_ok());
        assert!(validate_phone_number("12345").is_err());
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
    fn accepts_accented_names() {
        assert_eq!(validate_name("José", "First name"), Ok("José".to_string()));
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
    fn rejects_non_numeric_unique_id() {
        assert_eq!(
            validate_unique_id("abcdefghi"),
            Err(ValidationError::NotNumeric {
                field: "Unique ID".to_string(),
            })
        );
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
    fn rejects_wrong_length_unique_id() {
        assert_eq!(
            validate_unique_id("12345678901"),
            Err(ValidationError::WrongLength {
                field: "Unique ID".to_string(),
                expected: UNIQUE_ID_LENGTH,
            })
        );
    }
}
