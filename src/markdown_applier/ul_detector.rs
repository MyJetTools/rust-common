pub struct UlDetector {
    enter: bool,
    dash: bool,
    ul_is_opened: bool,
    content: String,
}

impl UlDetector {
    pub fn new() -> Self {
        Self {
            enter: false,
            dash: false,
            ul_is_opened: false,
            content: String::new(),
        }
    }

    fn flush_content(&mut self, out: &mut String) {
        let to_push = super::apply_markdown(self.content.trim());
        out.push_str(&to_push);
    }

    pub fn push_and_detect(&mut self, c: char, out: &mut String) -> bool {
        match self.enter {
            false => {
                if c == '\n' {
                    self.enter = true;
                    self.dash = false;
                }
            }

            true => {
                if self.dash {
                    if c == '\n' {
                        if self.content.len() > 0 {
                            self.flush_content(out);
                            self.content.clear();
                        }

                        out.push_str("</li>");
                        self.enter = true;
                        self.dash = false;
                    } else {
                        self.content.push(c);
                    }
                    return true;
                } else {
                    if c == '-' {
                        self.dash = true;

                        if !self.ul_is_opened {
                            self.ul_is_opened = true;
                            out.push_str("<ul>");
                        }

                        out.push_str("<li>");
                        return true;
                    }
                    self.enter = false;
                    self.dash = false;
                    if self.ul_is_opened {
                        out.push_str("</ul>");
                        self.ul_is_opened = false;
                    }
                }
            }
        }

        false
    }

    pub fn eof(mut self, out: &mut String) {
        if self.dash {
            if self.content.len() > 0 {
                self.flush_content(out);
            }

            out.push_str("</li>");
        }

        if self.ul_is_opened {
            out.push_str("</ul>");
        }
    }
}
