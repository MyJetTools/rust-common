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
        self.value.len() > 0
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

    fn reset(&mut self) {
        self.value.clear();
        self.last = ' ';
    }

    pub fn try_render_image(&mut self, out: &mut String) {
        if self.value.len() == 2 && self.value != "![" {
            if let Some(last) = self.value.chars().last() {
                if last == '\n' {
                    self.value.pop();
                }
            }

            out.push_str(self.value.as_str());
            self.reset();
        }

        if self.last != ')' {
            return;
        }

        if let Some(mut index_alt_start) = self.value.find('[') {
            index_alt_start += 1;
            if let Some(index_alt_end) = self.value.find(']') {
                if index_alt_start <= index_alt_end {
                    if let Some(mut index_url_start) = self.value.find('(') {
                        index_url_start += 1;
                        if let Some(index_url_end) = self.value.find(')') {
                            if index_alt_start <= index_url_end {
                                let src = &self.value[index_url_start..index_url_end];

                                let mut src = src.split(' ');

                                out.push_str("<img src=\"");
                                out.push_str(src.next().unwrap());
                                out.push('"');

                                if let Some(title) = src.next() {
                                    let mut title = title.to_string();

                                    while let Some(other_segment) = src.next() {
                                        title.push(' ');
                                        title.push_str(other_segment);
                                    }

                                    if title.starts_with('"') {
                                        out.push_str(" title=");
                                        out.push_str(title.as_str());
                                    } else {
                                        out.push_str(" title=\"");
                                        out.push_str(title.as_str());
                                        out.push('"');
                                    }
                                }

                                out.push_str(" alt=\"");
                                out.push_str(&self.value[index_alt_start..index_alt_end]);
                                out.push_str("\">");
                            }
                        }
                    }
                }
            }
        }
        self.reset();
    }
}
