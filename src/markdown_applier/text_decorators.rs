#[derive(Debug, Clone, Copy)]
pub struct TextDecorators {
    pub bold: bool,
    pub italic: bool,
    pub cross_through: bool,
}

impl TextDecorators {
    pub fn render_tag(&self, open: bool, out: &mut String) {
        if open {
            if self.bold {
                super::html_renderer::render_tag("b", open.into(), out);
            }
            if self.italic {
                super::html_renderer::render_tag("em", open.into(), out);
            }
            if self.cross_through {
                super::html_renderer::render_tag("del", open.into(), out);
            }
        } else {
            if self.cross_through {
                super::html_renderer::render_tag("del", open.into(), out);
            }
            if self.italic {
                super::html_renderer::render_tag("em", open.into(), out);
            }
            if self.bold {
                super::html_renderer::render_tag("b", open.into(), out);
            }
        }
    }
}
