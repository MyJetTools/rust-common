use super::*;

pub fn apply_markdown(src: &str) -> String {
    let mut result = String::with_capacity(src.len());

    let mut header = None;

    let mut esc_mode = false;

    let mut text_decorators: Option<TextDecorators> = None;

    let mut img_detector = ImgDetector::new();

    let mut header_detector = HeaderDetector::new();

    let mut text_decorators_detector = TextDecoratorsDetector::new();

    let mut ul_detector = UlDetector::new();

    let mut charged_br = 0;

    for c in src.chars() {
        if esc_mode {
            result.push(c);
            esc_mode = false;
            continue;
        }

        let mut push_char = true;

        if img_detector.push_and_try_render(c, &mut result) {
            if c != '\n' {
                continue;
            }
        }

        if ul_detector.push_and_detect(c, &mut result) {
            charged_br = 0;
            if c != '\n' {
                continue;
            }
        }

        match header_detector.push_and_detect(c) {
            HeaderDetectionResult::None => {}
            HeaderDetectionResult::InDetection => {
                continue;
            }
            HeaderDetectionResult::HasResult(header_size) => {
                super::html_renderer::render_header(header_size, true, &mut result);
                charged_br = 0;
                header = Some(header_size);
                push_char = false;
            }
        }

        if c == '\n' {
            if let Some(header_size) = header.take() {
                super::html_renderer::render_header(header_size, false, &mut result);
                charged_br = 0;
                push_char = false;
            } else {
                if charged_br > 0 {
                    result.push_str("<br/>");
                }
                charged_br = 2;
            }
        }

        if let Some(new_text_decorators) = text_decorators_detector.process_and_get_value(c) {
            if let Some(text_decorators) = text_decorators.take() {
                text_decorators.render_tag(false, &mut result);
            } else {
                new_text_decorators.render_tag(true, &mut result);
                text_decorators = Some(new_text_decorators);
            }
        }
        if text_decorators_detector.in_detection() {
            push_char = false;
        }

        if push_char {
            if c != '\n' {
                if charged_br == 1 {
                    result.push_str("<br/>");
                }
                result.push(c);
            }
        }

        if charged_br > 0 {
            charged_br -= 1;
        }
    }

    if let Some(text_decorators) = text_decorators {
        text_decorators.render_tag(false, &mut result);
    }

    if let Some(header_size) = header {
        super::html_renderer::render_header(header_size, false, &mut result);
    }

    ul_detector.eof(&mut result);

    result
}

pub enum NotPrintedEnter {
    Discharged,
    Charged,
    ToPrint,
}

impl NotPrintedEnter {
    pub fn to_print(&self) -> bool {
        match self {
            Self::ToPrint => true,
            _ => false,
        }
    }

    pub fn mark_to_print(&mut self) {
        match self {
            NotPrintedEnter::Discharged => {}
            NotPrintedEnter::Charged => *self = Self::ToPrint,
            NotPrintedEnter::ToPrint => {}
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn apply_markdown_text() {
        let text = "1. **Text Title** Other text **TextTitle2**";

        let result = super::apply_markdown(text);

        assert_eq!("1. <b>Text Title</b> Other text <b>TextTitle2</b>", result);
    }

    #[test]
    fn apply_markdown_text_with_text_at_the_end() {
        let text = "1. **Text Title** Other text **TextTitle2** Other text2";

        let result = super::apply_markdown(text);

        assert_eq!(
            "1. <b>Text Title</b> Other text <b>TextTitle2</b> Other text2",
            result
        );
    }

    #[test]
    fn test_apply_markdown_no_special_symbols() {
        let text = "Hi, I’m Noor! Excited to help you.";
        let result = super::apply_markdown(text);

        assert_eq!(result, text)
    }

    #[test]
    fn test_header_detector() {
        let text =
            "Hi, I’m Noor!\n## Prime Location Positioned in Business Bay\nExcited to help you.";

        let result = super::apply_markdown(text);
        assert_eq!(
            result,
            "Hi, I’m Noor!<h2>Prime Location Positioned in Business Bay</h2>Excited to help you."
        )
    }

    #[test]
    fn test_img_with_title() {
        let text = "![A cat](https://example.com/cat.jpg \"Cat Image\")";

        let result = super::apply_markdown(text);
        assert_eq!(
            result,
            "<img src=\"https://example.com/cat.jpg\" title=\"Cat Image\" alt=\"A cat\">"
        )
    }

    #[test]
    fn test_img_with_no_title() {
        let text = "![A cat](https://example.com/cat.jpg)";

        let result = super::apply_markdown(text);
        assert_eq!(
            result,
            "<img src=\"https://example.com/cat.jpg\" alt=\"A cat\">"
        )
    }

    #[test]
    fn test_img_with_other_content() {
        let text = "Other content ![A cat](https://example.com/cat.jpg) Other content";

        let result = super::apply_markdown(text);
        assert_eq!(
            result,
            "Other content <img src=\"https://example.com/cat.jpg\" alt=\"A cat\"> Other content"
        )
    }

    #[test]
    fn test_ul() {
        let src = r#"## Key Features
- List1
- List2
- List3"#;

        let result = super::apply_markdown(src);

        assert_eq!(
            result,
            "<h2>Key Features</h2><ul><li>List1</li><li>List2</li><li>List3</li></ul>"
        )
    }

    #[test]
    fn test_ul_with_enter_at_the_end() {
        let src = r#"## Key Features
- List1
- **List2**
- List3
"#;

        let result = super::apply_markdown(src);

        assert_eq!(
            result,
            "<h2>Key Features</h2><ul><li>List1</li><li><b>List2</b></li><li>List3</li></ul>"
        )
    }

    #[test]
    fn test_some_example() {
        let text = r#"Comfort by Pagani.

![Da Vinci Tower](https://img.jpg)

## Prime Location
Positioned in Business Bay, you will have access to top-tier amenities and stunning views of downtown Dubai.

## Key Features
- **Size**: 1,605.65 sq. ft. with a spacious balcony of 420.65 sq. ft.
- **Amenities**: State-of-the-art gym, rooftop pool, sauna, steam rooms, 24/7 concierge services, and exclusive residential access.
- **Special Resident Offer**: 20% discount on the Insignia Royal Card

## Project Update
This off-plan project is scheduled for completion in 2024. Priced at AED 7,895,020 (approximately USD 2,131,655).

Would you like more information, or would you like to explore further options?
"#;

        let result = super::apply_markdown(text);

        println!("{}", result);
    }
}
