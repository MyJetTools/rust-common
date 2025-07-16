pub struct ImgDetector {
    value: String,
    last: char,
}

impl ImgDetector {
    pub fn new() -> Self {
        Self {
            value: String::new(),
            last: ' ',
        }
    }

    pub fn is_image_in_process(&self) -> bool {
        self.value.len() > 9
    }

    pub fn push(&mut self, c: char) {
        if self.value.len() == 0 {
            if c != '!' {
                return;
            }
        }

        self.value.push(c);
        self.last = c;
    }

    pub fn try_render_image(&mut self, out: &mut String) {
        if self.last != ')' {
            return;
        }

        if let Some(index_alt_start) = self.value.find('[') {
            if let Some(index_alt_end) = self.value.find(']') {
                if index_alt_start <= index_alt_end {
                    if let Some(index_url_start) = self.value.find('(') {
                        if let Some(index_url_end) = self.value.find(')') {
                            if index_alt_start <= index_url_end {
                                out.push_str("<img src=\"");
                                out.push_str(&self.value[index_url_start..index_url_end]);
                                out.push_str("\" alt=\"");
                                out.push_str(&self.value[index_alt_start..index_alt_end]);
                                out.push_str("\">");
                            }
                        }
                    }
                }
            }
        }

        self.value.clear();
        self.last = ' ';
    }
}
