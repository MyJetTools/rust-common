use std::{
    fmt::{Debug, Display},
    marker::PhantomData,
};

use crate::validators::*;

use rust_extensions::AsStr;
use serde::*;

pub trait IdExtension {
    fn validate_value(src: &str) -> Result<(), ValueValidationResult>;
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[serde(transparent)]
pub struct ObjectId<TObjectIdValidator: IdExtension> {
    value: String,
    #[serde(skip)]
    phantom: PhantomData<TObjectIdValidator>,
}

impl<TObjectIdValidator: IdExtension> ObjectId<TObjectIdValidator> {
    pub const DEFAULT_REF: Self = Self {
        value: String::new(),
        phantom: PhantomData,
    };

    pub fn new(value: String) -> Self {
        Self {
            value,
            phantom: Default::default(),
        }
    }

    pub fn validate(&self) -> Result<(), ValueValidationResult> {
        TObjectIdValidator::validate_value(&self.value)
    }

    pub fn fix_me(&mut self) {
        if super::utils::generic_id_would_be_ok(&self.value) {
            return;
        }

        let mut result = String::new();

        for c in self.value.chars() {
            if super::utils::is_ok_char(c) {
                result.push(c);
            }
        }

        self.value = result
    }

    pub fn from_str(value: &str) -> Self {
        Self::new(value.to_string())
    }

    pub fn as_str(&self) -> &str {
        self.value.as_str()
    }

    pub fn eq_with_opt(&self, other: Option<&ObjectId<TObjectIdValidator>>) -> bool {
        match other {
            Some(other) => self.eq_with(other),
            None => false,
        }
    }

    pub fn eq_with(&self, other: &ObjectId<TObjectIdValidator>) -> bool {
        self.value == other.value
    }

    pub fn is_empty(&self) -> bool {
        self.value.is_empty()
    }

    pub fn len(&self) -> usize {
        self.value.len()
    }
}

impl<TObjectIdValidator: IdExtension> ValueValidator for ObjectId<TObjectIdValidator> {
    fn validate_value(&self) -> Result<(), ValueValidationResult> {
        self.validate()
    }
}

impl<TObjectIdValidator: IdExtension> AsStr for ObjectId<TObjectIdValidator> {
    fn as_str(&self) -> &str {
        self.value.as_str()
    }
}

impl<TObjectIdValidator: IdExtension> Default for ObjectId<TObjectIdValidator> {
    fn default() -> Self {
        Self::new(Default::default())
    }
}

impl<TObjectIdValidator: IdExtension> AsRef<str> for ObjectId<TObjectIdValidator> {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl<TObjectIdValidator: IdExtension> Display for ObjectId<TObjectIdValidator> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl<TObjectIdValidator: IdExtension> Debug for ObjectId<TObjectIdValidator> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl<TObjectIdValidator: IdExtension> Into<String> for ObjectId<TObjectIdValidator> {
    fn into(self) -> String {
        self.value
    }
}

impl<TObjectIdValidator: IdExtension> Into<ObjectId<TObjectIdValidator>> for &'_ str {
    fn into(self) -> ObjectId<TObjectIdValidator> {
        ObjectId::new(self.to_string())
    }
}

impl<TObjectIdValidator: IdExtension> Into<ObjectId<TObjectIdValidator>> for &'_ String {
    fn into(self) -> ObjectId<TObjectIdValidator> {
        ObjectId::new(self.to_string())
    }
}

impl<TObjectIdValidator: IdExtension> From<String> for ObjectId<TObjectIdValidator> {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl<TObjectIdValidator: IdExtension> IdExtension for ObjectId<TObjectIdValidator> {
    fn validate_value(src: &str) -> Result<(), ValueValidationResult> {
        TObjectIdValidator::validate_value(src)
    }
}

#[cfg(test)]
mod test {
    use serde::*;

    use crate::validators::ValueValidationResult;

    use super::IdExtension;

    use super::ObjectId;

    pub struct TestObjectIdValidator;

    impl IdExtension for TestObjectIdValidator {
        fn validate_value(src: &str) -> Result<(), ValueValidationResult> {
            super::super::utils::validate_generic_object_id(src)
        }
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Object {
        pub id: ObjectId<TestObjectIdValidator>,
        pub name: String,
    }

    #[test]
    fn test() {
        let object = Object {
            id: "id".into(),
            name: "name".into(),
        };

        let result = serde_json::to_string(&object).unwrap();

        println!("{}", result);
    }
}
