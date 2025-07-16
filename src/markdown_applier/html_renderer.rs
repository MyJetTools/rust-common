pub fn render_header(value: u8, open: bool, out: &mut String) {
    match value {
        1 => {
            super::html_renderer::render_tag("h1", open.into(), out);
        }
        2 => {
            super::html_renderer::render_tag("h2", open.into(), out);
        }
        3 => {
            super::html_renderer::render_tag("h3", open.into(), out);
        }
        4 => {
            super::html_renderer::render_tag("h4", open.into(), out);
        }
        _ => {}
    }
}

pub fn render_tag(name: &str, open: Option<bool>, out: &mut String) {
    out.push('<');
    match open {
        Some(true) => {
            out.push_str(name);
        }
        Some(false) => {
            out.push('/');
            out.push_str(name);
        }
        None => {
            out.push_str(name);
            out.push('/');
        }
    }

    out.push('>');
}
