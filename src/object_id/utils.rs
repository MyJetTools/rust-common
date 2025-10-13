use crate::validators::ValueValidationResult;

pub fn validate_generic_object_id(src: &str) -> Result<(), ValueValidationResult> {
    if src.len() == 0 {
        return Err(ValueValidationResult::Empty);
    }

    for c in src.chars() {
        if !is_ok_char(c) {
            return Err(ValueValidationResult::IllegalChars);
        }
    }

    Ok(())
}

pub fn is_ok_char(c: char) -> bool {
    if c.is_ascii_alphabetic() {
        return true;
    }

    if c.is_ascii_digit() {
        return true;
    }

    if c == '-' || c == '_' {
        return true;
    }

    return false;
}

pub fn generic_id_would_be_ok(value: &str) -> bool {
    for c in value.chars() {
        if !is_ok_char(c) {
            return false;
        }
    }

    true
}
