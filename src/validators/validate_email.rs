pub fn validate_email(email: &str) -> bool {
    if email.len() == 0 {
        return false;
    }

    let bytes = email.as_bytes();

    if bytes[0] == b'@' {
        return false;
    }

    if bytes[0] == b'.' {
        return false;
    }

    let mut at_amount = 0;

    let mut last = 0;
    for i in 0..bytes.len() {
        last = bytes[i];

        if last == b'@' {
            at_amount += 1;

            if at_amount > 1 {
                return false;
            }
            continue;
        }

        if last == b'.' {
            continue;
        }

        if last >= b'a' && last <= b'z' {
            continue;
        }

        if last >= b'A' && last <= b'Z' {
            continue;
        }

        if last >= b'0' && last <= b'9' {
            continue;
        }

        if last == b'-' || last == b'_' {
            continue;
        }

        return false;
    }

    if at_amount == 0 || last == b'.' || last == b'@' {
        return false;
    }

    true
}

#[cfg(test)]
mod test {

    #[test]
    fn test_email() {
        assert_eq!(true, super::validate_email("test@test.com"));
        assert_eq!(false, super::validate_email("test@@test.com"));

        assert_eq!(false, super::validate_email("@test.com"));

        assert_eq!(false, super::validate_email("aaaa@"));

        assert_eq!(true, super::validate_email("df.aSaaa@vvv.f23.df"));
    }
}
