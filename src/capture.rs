use std::collections::HashMap;

use image::ImageBuffer;
use screenshots::Screen;

use crate::threshold::Threshold;

pub(crate) struct Capture {
    location: (u16, u16),
    size: (u16, u16),
    threshold: Threshold,
}

#[derive(Default)]
pub(crate) struct CaptureBuilder {
    location: (u16, u16),
    size: (u16, u16),
    threshold: Threshold,
}

pub(crate) type Rgb = (u8, u8, u8);
pub(crate) type Screenshot = HashMap<usize, (u8, u8, u8)>;

impl Capture {
    pub(crate) fn builder() -> CaptureBuilder {
        CaptureBuilder::default()
    }

    pub(crate) fn screenshot(
        &self,
        predicate: Option<fn(rgb: &Rgb, threshold: &Threshold) -> bool>,
    ) -> Screenshot {
        let screens = Screen::all().unwrap();
        let mut target_pixels = Screenshot::new();

        let (x, y) = self.location;
        let (width, height) = self.size;

        for screen in screens.iter() {
            let image = screen
                .capture_area(x as i32, y as i32, width as u32, height as u32)
                .unwrap();
            let buffer = image.buffer();
            let image = image::load_from_memory(&buffer).unwrap().to_rgb8();

            for (index, pix) in image.pixels().enumerate() {
                let rgb = (pix.0[0], pix.0[1], pix.0[2]);
                let predicate = predicate.unwrap_or(|_, _| true);

                if predicate(&rgb, &self.threshold) {
                    target_pixels.insert(index, rgb);
                }
            }
        }

        target_pixels
    }

    pub(crate) fn on_change(
        &self,
        screenshot: &Screenshot,
        mut callback: impl FnMut(&Rgb, &ImageBuffer<image::Rgb<u8>, Vec<u8>>, &Threshold),
    ) {
        let screens = Screen::all().unwrap();

        let (x, y) = self.location;
        let (width, height) = self.size;

        loop {
            for screen in screens.iter() {
                let image = screen
                    .capture_area(x as i32, y as i32, width as u32, height as u32)
                    .unwrap();

                let buffer = image.buffer();
                let image = image::load_from_memory(&buffer).unwrap().to_rgb8();

                for (index, (r, g, b)) in screenshot.iter() {
                    let bar = image.pixels().collect::<Vec<_>>();

                    if let Some(baz) = bar.get(*index) {
                        let new_rgb = (baz.0[0], baz.0[1], baz.0[2]);
                        let (nr, ng, nb) = new_rgb;

                        if nr != *r || ng != *g || nb != *b {
                            callback(&new_rgb, &image, &self.threshold);
                        }
                    }
                }
            }
        }
    }

    pub(crate) fn location(&self) -> (u16, u16) {
        self.location
    }

    pub(crate) fn location_mut(&mut self) -> &mut (u16, u16) {
        &mut self.location
    }

    pub(crate) fn size(&self) -> (u16, u16) {
        self.size
    }

    pub(crate) fn size_mut(&mut self) -> &mut (u16, u16) {
        &mut self.size
    }

    pub(crate) fn threshold(&self) -> &Threshold {
        &self.threshold
    }
}

impl CaptureBuilder {
    pub(crate) fn build(self) -> Capture {
        Capture {
            location: self.location,
            size: self.size,
            threshold: self.threshold,
        }
    }

    pub(crate) fn location(mut self, location: (u16, u16)) -> Self {
        self.location = location;
        self
    }

    pub(crate) fn size(mut self, size: (u16, u16)) -> Self {
        self.size = size;
        self
    }

    pub(crate) fn threshold(mut self, threshold: Threshold) -> Self {
        self.threshold = threshold;
        self
    }
}
