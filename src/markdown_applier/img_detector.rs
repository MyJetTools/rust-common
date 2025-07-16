pub struct ImgDetector {
    value: String,
}

impl ImgDetector {
    pub fn new() -> Self {
        Self {
            value: String::new(),
        }
    }

    pub fn push_and_try_render(&mut self, c: char, out: &mut String) -> bool {
        if self.value.len() == 0 {
            if c == '!' {
                self.value.push(c);
            }
            return false;
        }

        if self.value.len() == 1 {
            if c == '[' {
                self.value.push(c);
                out.pop();
                return true;
            } else {
                self.value.clear();
                return false;
            }
        }
        self.value.push(c);

        if c == ')' {
            self.try_render_image(out);
            self.value.clear();
        }

        return true;
    }

    fn try_render_image(&mut self, out: &mut String) {
        if let Some(mut index_alt_start) = self.value.find('[') {
            index_alt_start += 1;
            if let Some(index_alt_end) = self.value.find(']') {
                if index_alt_start <= index_alt_end {
                    if let Some(mut index_url_start) = self.value.find('(') {
                        index_url_start += 1;
                        if let Some(index_url_end) = self.value.find(')') {
                            if index_alt_start <= index_url_end {
                                let src = &self.value[index_url_start..index_url_end];

                                let space_index = src.find(' ');

                                let (left, right) = if let Some(space_index) = space_index {
                                    (&src[..space_index], Some(&src[space_index + 1..]))
                                } else {
                                    (src, None)
                                };

                                out.push_str("<img src=\"");
                                out.push_str(left);
                                out.push('"');

                                if let Some(title) = right {
                                    if title.starts_with('"') {
                                        out.push_str(" title=");
                                        out.push_str(title);
                                    } else {
                                        out.push_str(" title=\"");
                                        out.push_str(title);
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
    }
}
