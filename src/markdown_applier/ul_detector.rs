pub struct UlDetector {
    enter: bool,
    dash: bool,
    ul_is_opened: bool,
}

impl UlDetector {
    pub fn new() -> Self {
        Self {
            enter: false,
            dash: false,
            ul_is_opened: false,
        }
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
                        out.push_str("</li>");
                        self.enter = true;
                        self.dash = false;
                    } else {
                        out.push(c);
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
}
