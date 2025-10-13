use std::fmt::Display;

use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ObjectIdStatic(&'static str);

impl ObjectIdStatic {
    pub const DEFAULT_REF: Self = Self("");

    pub const fn new(value: &'static str) -> Self {
        Self(value)
    }

    pub fn validate(&self) -> Option<bool> {
        if self.0.len() == 0 {
            return None;
        }

        let mut first = ' ';
        let mut last = ' ';

        for (i, c) in self.0.chars().enumerate() {
            if !is_ok_char(c) {
                return Some(false);
            }

            if i == 0 {
                first = c;
            }

            last = c;
        }

        if first == ' ' {
            return Some(false);
        }

        if last == ' ' {
            return Some(false);
        }

        Some(true)
    }

    pub fn as_str(&self) -> &'static str {
        self.0
    }

    pub fn eq_with_opt(&self, other: Option<&ObjectIdStatic>) -> bool {
        match other {
            Some(other) => self.eq_with(other),
            None => false,
        }
    }

    pub fn eq_with(&self, other: &ObjectIdStatic) -> bool {
        self.0 == other.0
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl Default for ObjectIdStatic {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<TObjectIdValidator: IdExtension> Into<ObjectId<TObjectIdValidator>> for ObjectIdStatic {
    fn into(self) -> ObjectId<TObjectIdValidator> {
        ObjectId::new(self.0.to_string())
    }
}

fn is_ok_char(c: char) -> bool {
    if c.is_ascii_alphabetic() {
        return true;
    }

    if c.is_ascii_digit() {
        return true;
    }

    if c == ' ' || c == '-' {
        return true;
    }

    return c as u8 >= 32;
}

impl AsRef<str> for ObjectIdStatic {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl Display for ObjectIdStatic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Into<String> for ObjectIdStatic {
    fn into(self) -> String {
        self.0.to_string()
    }
}

impl Into<ObjectIdStatic> for &'static str {
    fn into(self) -> ObjectIdStatic {
        ObjectIdStatic::new(self)
    }
}
