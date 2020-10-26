use crate::{
    utils::font::{
        fontdue::layout::{GlyphPosition, GlyphRasterConfig, Layout, LayoutSettings, TextStyle},
        Font,
    },
    widget::Widget,
    Event, Handler,
};
use cgmath::Vector2;
use cybuf::{Buffer, Drawable};
use std::marker::PhantomData;
use std::sync::mpsc::Sender;

pub struct TextBuilder {
    position: Vector2<isize>,
    font: Font,
    text: Option<String>,
    layout_settings: LayoutSettings,
}

impl TextBuilder {
    pub fn new(position: Vector2<isize>, font: Font) -> Self {
        Self {
            text: None,
            position,
            font,
            layout_settings: Default::default(),
        }
    }

    pub fn with_layout_settings(mut self, layout_settings: LayoutSettings) -> Self {
        self.layout_settings = layout_settings;
        self
    }

    pub fn with_text(mut self, text: String) -> Self {
        self.text = Some(text);
        self
    }

    pub fn build<D, T>(self) -> Text<D, T>
    where
        D: Drawable,
        T: Clone,
    {
        Text {
            text: self.text.clone().unwrap_or("".into()),
            buffer: Buffer::new((1, 1).into(), self.font.bg.clone()),
            position: self.position.clone(),
            font: self.font,
            layout: Layout::new(),
            settings: self.layout_settings.clone(),
            _ph_drawable: Default::default(),
            _ph_event: Default::default(),
        }
    }
}

pub struct Text<D, T>
where
    D: Drawable,
    T: Clone,
{
    text: String,
    buffer: Buffer,
    position: Vector2<isize>,
    font: Font,
    layout: Layout,
    settings: LayoutSettings,
    _ph_drawable: PhantomData<D>,
    _ph_event: PhantomData<T>,
}

impl<D, T> Text<D, T>
where
    D: Drawable,
    T: Clone,
{
    pub fn update_text(&mut self, text: String) {
        self.text = text;
    }

    pub fn render(&mut self) {
        // The vector where the glyphs positional information will be written to. This vec is cleared before it's written to.
        let mut output = Vec::new();

        // The list of fonts that will be used during layout.
        let fonts = &[self.font.font.as_ref()];
        // The text that will be laid out, its size, and the index of the font in the font list to use for that section of text.
        let styles = &[&TextStyle::new(self.text.as_str(), self.font.size, 0)];
        // Calculate the layout.
        self.layout
            .layout_horizontal(fonts, styles, &self.settings, &mut output);
        let last = output.last().unwrap_or(&GlyphPosition {
            key: GlyphRasterConfig {
                c: 'a',
                px: 0.0,
                font_index: 0,
            },
            x: 0.0,
            y: 0.0,
            width: 0,
            height: 0,
        });
        let size = (
            last.x as isize + last.width as isize,
            output.iter().map(|o| o.y.abs() as isize).max().unwrap_or(0),
        );
        println!("{:?}", size);

        let mut buffer = Buffer::new(size.into(), self.font.bg);

        for o in output {
            let (metrics, letter) = self.font.font.rasterize(o.key.c, self.font.size);
            for x in 0..metrics.width {
                for (y_buf, y) in (0..metrics.height).rev().enumerate() {
                    if letter[y_buf * metrics.width + x] != 0 {
                        buffer.put_pixel(
                            Vector2 {
                                x: (x as f32 + metrics.xmin as f32 + o.x) as isize,
                                y: ((100.0 - y as f32) + (-metrics.ymin as f32)) as isize,
                            },
                            self.font.fg,
                        )
                    }
                }
            }
        }
        self.buffer = buffer;
    }
}

impl<D, T> Widget<D, T> for Text<D, T>
where
    D: Drawable,
    T: Clone,
{
    fn draw(&self, buffer: &mut D) {
        buffer.put_buffer(self.position, &self.buffer);
    }
}

impl<D, T: Clone> Handler<T> for Text<D, T>
where
    D: Drawable,
    T: Clone,
{
    fn attach(&mut self, _: Sender<Event<T>>) {}

    fn handle_event(&mut self, _: Event<T>) {}
}
