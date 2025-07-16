use crate::TextDecorators;

const MAX_CHARS: usize = 6;
pub struct LastChars {
    chars: Vec<u8>,
}

impl LastChars {
    pub fn new() -> Self {
        let mut result = Self {
            chars: Vec::with_capacity(MAX_CHARS),
        };

        result.chars.push(b'\n');

        result
    }

    pub fn push_char(&mut self, c: char) {
        if self.chars.len() == MAX_CHARS {
            self.chars.remove(0);
        }

        self.chars.push(c as u8);
    }

    pub fn are(&self, src: &str) -> bool {
        if self.chars.len() < src.len() {
            return false;
        }

        let start_element = self.chars.len() - src.len();

        let sub_chars = &self.chars[start_element..];

        sub_chars == src.as_bytes()
    }

    pub fn try_get_text_decorator(&self) -> Option<TextDecorators> {
        if self.chars.len() == 0 {
            return None;
        }

        if self.are("***") || self.are("___") {
            return TextDecorators {
                bold: true,
                cross_through: false,
                italic: true,
            }
            .into();
        }

        if self.are("**") || self.are("__") {
            return TextDecorators {
                bold: true,
                cross_through: false,
                italic: false,
            }
            .into();
        }
        if self.are("~~") {
            return TextDecorators {
                bold: false,
                cross_through: true,
                italic: false,
            }
            .into();
        }

        if self.are("*") || self.are("_") {
            return TextDecorators {
                bold: false,
                cross_through: false,
                italic: true,
            }
            .into();
        }

        None
    }
}
