
use std::collections::HashMap;
use std::path::PathBuf;

use fontdue::{Font,FontSettings};
use fontdue::layout::{Layout as TextLayout, LayoutSettings as TextLayoutSettings,TextStyle};

use crate::{Renderer,RendererBase,State,Node,Mesh,View,Content};

macro_rules! add_font {
    ($fonts:expr, $font_names:expr, $path:literal) => {
        {
            let path_buf = PathBuf::from($path);
            let name = path_buf.file_stem().and_then(|x| x.to_str()).expect("Failed to load font");
            let font = fontdue::Font::from_bytes(&include_bytes!($path)[..], FontSettings::default())
                .expect("Failed to load font");
            let i = $fonts.len();
            $fonts.push(font);
            $font_names.insert(name.to_owned(), i);
        }
    }
}

pub struct GuiRenderer {
    pub fonts: Vec<fontdue::Font>,
    pub font_names: HashMap<String, usize>,
    pub text_layout: TextLayout,
    // pub glyph_mesh: MeshId,
}

impl GuiRenderer {
    pub fn create(base: &RendererBase) -> Self {
        let mut fonts = Vec::new();
        let mut font_names = HashMap::default();

        add_font!(fonts, font_names, "../font/Inter-Regular.otf");
        add_font!(fonts, font_names, "../font/Inter-Bold.otf");
        add_font!(fonts, font_names, "../font/Inter-Italic.otf");
        add_font!(fonts, font_names, "../font/Inter-BoldItalic.otf");

        let text_layout = TextLayout::new(fontdue::layout::CoordinateSystem::PositiveYUp);

        // let glyph_mesh = base.create_mesh_with_data();

        Self {
            fonts,
            font_names,
            text_layout,
            // glyph_mesh,
        }
    }
}

impl Renderer {
    pub fn render_gui(
        &mut self,
        frame: &wgpu::SwapChainFrame,
        encoder: &mut wgpu::CommandEncoder,
        state: &State,
    ) {
        let view = View::new(state);

        let mut current_box = (
            0.0f32,
            0.0f32,
            self.swap_chain_descriptor.width as f32,
            self.swap_chain_descriptor.height as f32
        );

        let mut node_queue = vec![ &view.root ];

        loop {
            let node = match node_queue.pop() {
                Some(x) => x,
                None => break,
            };

            match &node.content {
                Content::Text(text) => {
                    self.gui_renderer.text_layout.reset(&TextLayoutSettings {
                        x: 0.0,
                        y: 0.0,
                        max_height: None,
                        max_width: None,
                        horizontal_align: fontdue::layout::HorizontalAlign::Left,
                        vertical_align: fontdue::layout::VerticalAlign::Top,
                        wrap_style: fontdue::layout::WrapStyle::Word,
                        wrap_hard_breaks: true,
                    });

                    // TODO: Compute glyphs in multiple stages for large amounts of text?

                    self.gui_renderer.text_layout.append(
                        &self.gui_renderer.fonts,
                        &TextStyle::new(text.as_str(), 45.0, 0)
                    );

                    println!("{:?}", self.gui_renderer.text_layout.glyphs());
                },
                _ => {

                },
            }
        }
    }
}