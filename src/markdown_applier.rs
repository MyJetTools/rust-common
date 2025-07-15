pub enum ReadMode {
    None,
    Br1,
    Bold,
    BoldEnd1,
}

impl Default for ReadMode {
    fn default() -> Self {
        Self::None
    }
}

/*
impl MarkDownToPush {
    pub fn as_str(&self) -> &'static str {
        match self {
            MarkDownToPush::BrOpen => "<b>",
            MarkDownToPush::BrClose => "</b>",
        }
    }
}
 */
pub fn apply_markdown(src: &str) -> String {
    let data = preapply_markdown(src);

    let mut result = String::with_capacity(data.len());

    let mut read_mode = ReadMode::None;

    for c in data.chars() {
        match read_mode {
            ReadMode::None => match c {
                '*' => {
                    read_mode = ReadMode::Br1;
                }
                _ => {
                    result.push(c);
                }
            },
            ReadMode::Br1 => match c {
                '*' => {
                    result.push_str("<b>");
                    read_mode = ReadMode::Bold;
                }
                _ => {
                    result.push('*');
                    result.push(c);
                    read_mode = ReadMode::None;
                }
            },

            ReadMode::Bold => match c {
                '*' => {
                    read_mode = ReadMode::BoldEnd1;
                }
                _ => {
                    result.push(c);
                }
            },
            ReadMode::BoldEnd1 => match c {
                '*' => {
                    result.push_str("</b>");
                    read_mode = ReadMode::None;
                }
                _ => {
                    result.push('*');
                    result.push(c);
                    read_mode = ReadMode::Bold;
                }
            },
        }
    }

    result
}

fn preapply_markdown(src: &str) -> String {
    let mut result = String::new();

    for line in src.split('\n') {
        if line.starts_with("# ") {
            result.push_str("<h1>");
            result.push_str(&line[2..]);
            result.push_str("</h1>");
            continue;
        }

        if line.starts_with("## ") {
            result.push_str("<h2>");
            result.push_str(&line[3..]);
            result.push_str("</h2>");
            continue;
        }

        if line.starts_with("### ") {
            result.push_str("<h3>");
            result.push_str(&line[4..]);
            result.push_str("</h3>");
            continue;
        }

        if line.starts_with("###$ ") {
            result.push_str("<h4>");
            result.push_str(&line[5..]);
            result.push_str("</h4>");
            continue;
        }

        result.push_str(line);
        result.push_str("<br/>");
    }

    result
}

#[cfg(test)]
mod tests {

    #[test]
    fn apply_markdown_text() {
        let text = "1. **Text Title** Other text **TextTitle2**";

        let result = super::apply_markdown(text);

        assert_eq!("1. <b>Text Title</b> Other text <b>TextTitle2</b>", result);
    }

    /*
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
     */
}
