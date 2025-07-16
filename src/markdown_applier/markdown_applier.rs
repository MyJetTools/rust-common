use super::*;

pub fn apply_markdown(src: &str) -> String {
    let mut result = String::with_capacity(src.len());

    let mut header = None;

    let mut esc_mode = false;

    let mut last_chars = LastChars::new();

    let mut text_decorators: Option<TextDecorators> = None;

    let mut img_detector = ImgDetector::new();

    for c in src.chars() {
        if esc_mode {
            result.push(c);
            esc_mode = false;
            continue;
        }

        let mut push_it = true;

        img_detector.push(c);

        if img_detector.is_image_in_process() {
            push_it = false;
        }

        img_detector.try_render_image(&mut result);

        match c {
            ' ' => {
                if last_chars.are("\n#") {
                    result.pop();
                    super::html_renderer::render_tag("h1", Some(true), &mut result);
                    header = Some(1);
                    push_it = false;
                }

                if last_chars.are("\n##") {
                    result.pop();
                    result.pop();
                    super::html_renderer::render_tag("h2", Some(true), &mut result);
                    header = Some(2);
                    push_it = false;
                }
                if last_chars.are("\n###") {
                    result.pop();
                    result.pop();
                    result.pop();
                    super::html_renderer::render_tag("h3", Some(true), &mut result);
                    header = Some(3);
                    push_it = false;
                }
                if last_chars.are("\n####") {
                    result.pop();
                    result.pop();
                    result.pop();
                    result.pop();
                    super::html_renderer::render_tag("h4", Some(true), &mut result);
                    header = Some(4);
                    push_it = false;
                }
            }
            '\n' => {
                if let Some(header_size) = header.take() {
                    super::html_renderer::render_header(header_size, false, &mut result);
                } else {
                    result.push_str("<br/>");
                }
                push_it = false;
            }
            '\\' => {
                push_it = false;
                esc_mode = true
            }
            _ => {}
        }

        if !super::utils::is_decorator_symbol(c) {
            if let Some(new_text_decorators) = last_chars.try_get_text_decorator() {
                if let Some(text_decorators) = text_decorators.take() {
                    text_decorators.render_tag(false, &mut result);
                } else {
                    new_text_decorators.render_tag(true, &mut result);
                    text_decorators = Some(new_text_decorators);
                }
            }
        } else {
            push_it = false;
        }

        last_chars.push_char(c);

        if push_it {
            result.push(c);
        }
    }

    if let Some(text_decorators) = text_decorators {
        text_decorators.render_tag(false, &mut result);
    }

    if let Some(header_size) = header {
        super::html_renderer::render_header(header_size, false, &mut result);
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
            "Hi, I’m Noor!<br/><h2>Prime Location Positioned in Business Bay</h2>Excited to help you."
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
}
