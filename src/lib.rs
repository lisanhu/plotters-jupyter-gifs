#![allow(dead_code)]
use plotters::prelude::*;
use plotters::style::text_anchor::{HPos, Pos, VPos};

// rgba color
#[derive(Copy, Clone)]
pub struct Color(pub u8, pub u8, pub u8, pub u8);

#[allow(dead_code)]
pub const RED: Color = Color(255, 0, 0, 255);
#[allow(dead_code)]
pub const GREEN: Color = Color(0, 255, 0, 255);
#[allow(dead_code)]
pub const BLUE: Color = Color(0, 0, 255, 255);
#[allow(dead_code)]
pub const CYAN: Color = Color(0, 255, 255, 255);
// const BLACK: Color = Color(0, 0, 0, 255);

pub fn to_plotters_rgba(color: Color) -> plotters::style::RGBAColor {
    plotters::style::RGBAColor(color.0, color.1, color.2, color.3 as f64 / 255f64)
}

pub struct Cell {
    pub lt: (i32, i32),                     // left pos
    pub rb: (i32, i32),                     // top pos
    pub border_color: Color,                // rgba
    pub background_color: Color,            // rgba
    pub text: Option<String>,               // cell text
    pub font: Option<(String, i32, Color)>, // font family and font size
}

impl Cell {
    #[allow(dead_code)]
    pub fn set_text(&mut self, text: &str) {
        self.text = Some(text.to_owned());
    }

    #[allow(dead_code)]
    pub fn draw(&self, draw: &DrawingArea<BitMapBackend, plotters::coord::Shift>) {
        draw.draw(&Rectangle::new(
            [self.lt, self.rb],
            ShapeStyle {
                color: to_plotters_rgba(self.background_color),
                filled: true,
                stroke_width: 1,
            },
        ))
        .unwrap();
        draw.draw(&Rectangle::new(
            [self.lt, self.rb],
            ShapeStyle {
                color: to_plotters_rgba(self.border_color),
                filled: false,
                stroke_width: 1,
            },
        ))
        .unwrap();

        let width = self.rb.0 - self.lt.0;
        let height = self.rb.1 - self.lt.1;
        if let Some(text) = &self.text {
            let ft = self.font.as_ref().unwrap();
            let ff = &ft.0 as &str;
            let fs = ft.1;
            let fc = to_plotters_rgba(ft.2);
            let font = TextStyle::from((ff, fs).into_font())
                .color(&fc)
                .pos(Pos::new(HPos::Center, VPos::Center));
            draw.draw_text(
                text,
                &font,
                (self.lt.0 + (width / 2), self.lt.1 + (height / 2)),
            )
            .unwrap();
        }
    }
}

use plotters::coord::Shift;

/**
 * (path to gif, style text)
 */
pub struct GIFWrapper(pub String, pub String);

impl GIFWrapper {
    pub fn evcxr_display(&self) {
        println!("{:?}", self);
    }

    pub fn style<S: Into<String>>(mut self, style: S) -> Self {
        self.1 = style.into();
        self
    }
}

impl std::fmt::Debug for GIFWrapper {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        use std::time::{SystemTime, UNIX_EPOCH};
        let start = SystemTime::now();
        let time_since_epoch = start.duration_since(UNIX_EPOCH).expect("We are back in time!").as_millis();
        
        let path = &self.0;
        write!(
            formatter,
            "EVCXR_BEGIN_CONTENT text/html\n<div><img style=\"{}\" src=\"{}?{}\"/></div>\nEVCXR_END_CONTENT",
            self.1, path, time_since_epoch
        )
    }
}

pub fn evcxr_gif_figure<
    Draw: FnOnce(DrawingArea<BitMapBackend, Shift>) -> Result<(), Box<dyn std::error::Error>>,
>(
    size: (u32, u32),
    path: &str,
    frame_delay: u32,
    draw: Draw,
) -> GIFWrapper {
    let root = BitMapBackend::gif(path, size, frame_delay);
    let root = root.unwrap().into_drawing_area();
    draw(root).expect("Drawing failure");
    GIFWrapper(path.to_owned(), "".to_owned())
}
