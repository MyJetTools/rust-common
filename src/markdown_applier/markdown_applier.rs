use super::*;

#[derive(Debug, Clone, Copy)]
enum LineMode {
    None,
    Ul,
}

impl LineMode {
    pub fn is_none(&self) -> bool {
        match self {
            Self::None => true,
            _ => false,
        }
    }
}

pub fn apply_markdown(src: &str) -> String {
    let mut line_mode = LineMode::None;
    let mut result = String::with_capacity(src.len());

    for line in src.lines() {
        if line.starts_with("# ") {
            result.push_str("<h1>");
            apply_markdown_to_line(&line[2..], &mut result);
            result.push_str("</h1>");
            continue;
        }

        if line.starts_with("## ") {
            result.push_str("<h2>");
            apply_markdown_to_line(&line[3..], &mut result);
            result.push_str("</h2>");
            continue;
        }

        if line.starts_with("### ") {
            result.push_str("<h3>");
            apply_markdown_to_line(&line[4..], &mut result);
            result.push_str("</h3>");
            continue;
        }

        if line.starts_with("#### ") {
            result.push_str("<h4>");
            apply_markdown_to_line(&line[5..], &mut result);
            result.push_str("</h4>");
            continue;
        }

        if line.starts_with("- ") {
            match line_mode {
                LineMode::None => {
                    result.push_str("<ul>");
                    line_mode = LineMode::Ul;
                }
                LineMode::Ul => {}
            }

            result.push_str("<li>");
            apply_markdown_to_line(&line[2..], &mut result);
            result.push_str("</li>");
            continue;
        }

        match line_mode {
            LineMode::None => {}
            LineMode::Ul => {
                result.push_str("</ul>");
            }
        }

        result.push_str("<p>");
        apply_markdown_to_line(line, &mut result);
        result.push_str("</p>");
    }

    match line_mode {
        LineMode::None => {}
        LineMode::Ul => {
            result.push_str("</ul>");
        }
    }

    result
}

fn apply_markdown_to_line(src: &str, out: &mut String) {
    let mut esc_mode = false;

    let mut text_decorators: Option<TextDecorators> = None;

    let mut img_detector = ImgDetector::new();

    let mut text_decorators_detector = TextDecoratorsDetector::new();

    let mut charged_br = 0;

    for c in src.chars() {
        if esc_mode {
            out.push(c);
            esc_mode = false;
            continue;
        }

        let mut push_char = true;

        if img_detector.push_and_try_render(c, out) {
            if c != '\n' {
                continue;
            }
        }

        if let Some(new_text_decorators) = text_decorators_detector.process_and_get_value(c) {
            if let Some(text_decorators) = text_decorators.take() {
                text_decorators.render_tag(false, out);
            } else {
                new_text_decorators.render_tag(true, out);
                text_decorators = Some(new_text_decorators);
            }
        }
        if text_decorators_detector.in_detection() {
            push_char = false;
        }

        if push_char {
            if c != '\n' {
                if charged_br == 1 {
                    out.push_str("<br/>");
                }
                out.push(c);
            }
        }

        if charged_br > 0 {
            charged_br -= 1;
        }
    }

    if let Some(text_decorators) = text_decorators {
        text_decorators.render_tag(false, out);
    }
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

        assert_eq!(
            "<p>1. <b>Text Title</b> Other text <b>TextTitle2</b></p>",
            result
        );
    }

    #[test]
    fn apply_markdown_text_with_text_at_the_end() {
        let text = "1. **Text Title** Other text **TextTitle2** Other text2";

        let result = super::apply_markdown(text);

        assert_eq!(
            "<p>1. <b>Text Title</b> Other text <b>TextTitle2</b> Other text2</p>",
            result
        );
    }

    #[test]
    fn test_apply_markdown_no_special_symbols() {
        let text = "Hi, I’m Noor! Excited to help you.";
        let result = super::apply_markdown(text);

        assert_eq!("<p>Hi, I’m Noor! Excited to help you.</p>", result)
    }

    #[test]
    fn test_header_detector() {
        let text =
            "Hi, I’m Noor!\n## Prime Location Positioned in Business Bay\nExcited to help you.";

        let result = super::apply_markdown(text);
        assert_eq!(
            result,
            "<p>Hi, I’m Noor!</p><h2>Prime Location Positioned in Business Bay</h2><p>Excited to help you.</p>"
        )
    }

    #[test]
    fn test_img_with_title() {
        let text = "![A cat](https://example.com/cat.jpg \"Cat Image\")";

        let result = super::apply_markdown(text);
        assert_eq!(
            result,
            "<p><img src=\"https://example.com/cat.jpg\" title=\"Cat Image\" alt=\"A cat\"></p>"
        )
    }

    #[test]
    fn test_img_with_no_title() {
        let text = "![A cat](https://example.com/cat.jpg)";

        let result = super::apply_markdown(text);
        assert_eq!(
            result,
            "<p><img src=\"https://example.com/cat.jpg\" alt=\"A cat\"></p>"
        )
    }

    #[test]
    fn test_img_with_other_content() {
        let text = "Other content ![A cat](https://example.com/cat.jpg) Other content";

        let result = super::apply_markdown(text);
        assert_eq!(
            result,
            "<p>Other content <img src=\"https://example.com/cat.jpg\" alt=\"A cat\"> Other content</p>"
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

    #[test]
    fn test_example_from_real_life() {
        let src = r#"I'm here to offer you a personalized experience with our luxurious properties. Could you please share your preferred location or budget? This will help me tailor the best options for you."#;

        let result = super::apply_markdown(src);

        assert_eq!(format!("<p>{}</p>", src), result);
    }

    #[test]
    fn test_example_from_real_life_2() {
        let src = "Bt\n\n- **Bold**: Just text\n- **2-Bold**: 2-text\n- **3-Bold**: 3-text\n- **4-Bold**: 4-text\n\nAfter text";

        let result = super::apply_markdown(src);

        println!("{}", result);
    }
}
