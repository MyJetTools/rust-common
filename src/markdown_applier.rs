use std::str::Chars;

enum MarkDownToPush {
    BrOpen,
    BrClose,
}

impl MarkDownToPush {
    pub fn as_str(&self) -> &'static str {
        match self {
            MarkDownToPush::BrOpen => "<b>",
            MarkDownToPush::BrClose => "</b>",
        }
    }
}

pub struct MarkdownApplier<'s> {
    text: &'s str,
    chars: Chars<'s>,
    pos: usize,
    from_pos: usize,
    bold_open: bool,
    prev_char: Option<char>,
    push_markdown_to_push: Option<MarkDownToPush>,
}

impl<'s> MarkdownApplier<'s> {
    pub fn new(text: &'s str) -> Self {
        Self {
            text,
            chars: text.chars(),
            pos: 0,
            from_pos: 0,
            bold_open: true,
            prev_char: None,
            push_markdown_to_push: None,
        }
    }

    pub fn get_next(&mut self) -> Option<&'s str> {
        if let Some(markdown_applier) = self.push_markdown_to_push.take() {
            return Some(markdown_applier.as_str());
        }

        let mut pos = self.pos;

        if pos >= self.text.len() {
            return None;
        }
        loop {
            let next_char = self.chars.next();

            if next_char.is_none() {
                let result = &self.text[self.from_pos..pos];

                println!("Pos: {}. Chunk: {}", pos, result);
                self.pos = pos + 1;
                return Some(result);
            }

            let next_char = next_char.unwrap();

            if next_char == '*' {
                if let Some(prev_char) = self.prev_char {
                    if prev_char == '*' {
                        if self.bold_open {
                            self.push_markdown_to_push = Some(MarkDownToPush::BrOpen);
                        } else {
                            self.push_markdown_to_push = Some(MarkDownToPush::BrClose);
                        }

                        self.bold_open = !self.bold_open;

                        let result = &self.text[self.from_pos..pos - 1];

                        self.pos = pos + 1;
                        self.from_pos = pos + 1;

                        return Some(result);
                    }
                }
            }

            self.prev_char = Some(next_char);

            pos += next_char.len_utf8();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::markdown_applier::MarkdownApplier;

    #[test]
    fn apply_markdown_text() {
        let text = "1. **Text Title** Other text **TextTitle2**";

        let mut markdown_applier = MarkdownApplier::new(text);

        let item = markdown_applier.get_next().unwrap();
        assert_eq!("1. ", item);

        let item = markdown_applier.get_next().unwrap();
        assert_eq!("<b>", item);

        let item = markdown_applier.get_next().unwrap();
        assert_eq!("Text Title", item);

        let item = markdown_applier.get_next().unwrap();
        assert_eq!("</b>", item);

        let item = markdown_applier.get_next().unwrap();
        assert_eq!(" Other text ", item);

        let item = markdown_applier.get_next().unwrap();
        assert_eq!("<b>", item);

        let item = markdown_applier.get_next().unwrap();
        assert_eq!("TextTitle2", item);

        let item = markdown_applier.get_next().unwrap();
        assert_eq!("</b>", item);

        assert!(markdown_applier.get_next().is_none())
    }

    #[test]
    fn apply_markdown_text_with_text_at_the_end() {
        let text = "1. **Text Title** Other text **TextTitle2** Other text2";

        let mut markdown_applier = MarkdownApplier::new(text);

        let item = markdown_applier.get_next().unwrap();
        assert_eq!("1. ", item);

        let item = markdown_applier.get_next().unwrap();
        assert_eq!("<b>", item);

        let item = markdown_applier.get_next().unwrap();
        assert_eq!("Text Title", item);

        let item = markdown_applier.get_next().unwrap();
        assert_eq!("</b>", item);

        let item = markdown_applier.get_next().unwrap();
        assert_eq!(" Other text ", item);

        let item = markdown_applier.get_next().unwrap();
        assert_eq!("<b>", item);

        let item = markdown_applier.get_next().unwrap();
        assert_eq!("TextTitle2", item);

        let item = markdown_applier.get_next().unwrap();
        assert_eq!("</b>", item);

        let item = markdown_applier.get_next().unwrap();
        assert_eq!(" Other text2", item);

        assert!(markdown_applier.get_next().is_none())
    }

    #[test]
    fn test_apply_markdown_no_special_symbols() {
        let text = "Hi, I’m Noor! Excited to help you.";

        let mut markdown_applier = MarkdownApplier::new(text);

        let item = markdown_applier.get_next().unwrap();
        assert_eq!(text, item);

        let item = markdown_applier.get_next();

        assert!(item.is_none());

        //assert!(item.is_none());
    }
}
