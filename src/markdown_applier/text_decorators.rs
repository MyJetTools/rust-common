const MAX_SIZE: usize = 3;
pub struct TextDecoratorsDetector {
    value: Vec<u8>,
}

impl TextDecoratorsDetector {
    pub fn new() -> Self {
        Self {
            value: Vec::with_capacity(MAX_SIZE),
        }
    }

    pub fn process_and_get_value(&mut self, c: char) -> Option<TextDecorators> {
        if super::utils::is_decorator_symbol(c) {
            if self.value.len() == MAX_SIZE {
                self.value.remove(0);
            }
            self.value.push(c as u8);

            return None;
        }

        let result = TextDecorators::from(&self.value);
        self.value.clear();
        return result;
    }

    pub fn in_detection(&self) -> bool {
        self.value.len() > 0
    }
}

#[derive(Debug, Clone, Copy)]
pub struct TextDecorators {
    pub bold: bool,
    pub italic: bool,
    pub cross_through: bool,
}

impl TextDecorators {
    pub fn from(val: &[u8]) -> Option<Self> {
        if val.len() == 0 {
            return None;
        }

        if val == b"***" || val == b"___" {
            return TextDecorators {
                bold: true,
                cross_through: false,
                italic: true,
            }
            .into();
        }

        if val == b"**" || val == b"__" {
            return TextDecorators {
                bold: true,
                cross_through: false,
                italic: false,
            }
            .into();
        }

        if val == b"~~" {
            return TextDecorators {
                bold: false,
                cross_through: true,
                italic: false,
            }
            .into();
        }

        if val == b"*" || val == b"_" {
            return TextDecorators {
                bold: false,
                cross_through: false,
                italic: true,
            }
            .into();
        }

        None
    }
    pub fn render_tag(&self, open: bool, out: &mut String) {
        if open {
            if self.bold {
                super::html_renderer::render_tag("b", open.into(), out);
            }
            if self.italic {
                super::html_renderer::render_tag("em", open.into(), out);
            }
            if self.cross_through {
                super::html_renderer::render_tag("del", open.into(), out);
            }
        } else {
            if self.cross_through {
                super::html_renderer::render_tag("del", open.into(), out);
            }
            if self.italic {
                super::html_renderer::render_tag("em", open.into(), out);
            }
            if self.bold {
                super::html_renderer::render_tag("b", open.into(), out);
            }
        }
    }
}
