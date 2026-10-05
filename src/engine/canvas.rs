// Used to store information about the pixels
use crate::engine::geometry::rectangle;

#[inline]
fn flat_index(x: u32, y: u32, w: u32) -> usize {
    (y * w + x) as usize
}

#[inline]
fn from_index(i: u32, w: u32) -> (u32, u32) {
    (i % w, i / w)
}

#[derive(Clone)]
pub struct Canvas {
    width: u32,
    height: u32,
    pixels: Vec<u32>,
}

impl Default for Canvas {
    // Default canvas implementation with some arbitrary values used
    fn default() -> Self {
        Self::new(16, 16, 0xFF000000)
    }
}

// Boiler plate stuff
impl Canvas {
    pub fn new(width: u32, height : u32, color: u32) -> Self {
        Self {
            width,
            height,
            pixels: vec![color; (width * height) as usize],
        }
    }

    pub fn get_pixels(&self) -> &Vec<u32> {
        &self.pixels
    }

    pub fn into_pixels(self) -> Vec<u32> {
        self.pixels
    }

    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }
}

// Canvas methods
impl Canvas {
    // Draw a source canvas into self at a position offset
    pub fn blit(
        &mut self,
        src: &Canvas,
        position: (u32, u32),
    ) {
        for y in 0..src.height {
            for x in 0..src.width {
                let src_index: usize = flat_index(x, y, src.get_width());
                let dst_y = position.1 + y;
                let dst_x = position.0 + x;
                let dst_index: usize = flat_index(dst_x, dst_y, self.get_width());

                // Check for bounds
                if dst_y >= self.height || dst_x >= self.width {
                    continue;
                }

                self.pixels[dst_index] = src.get_pixels()[src_index];
            }
        }
    }

    // Draw a single point to the canvas
    pub fn draw_point(
        &mut self,
        point: (i32, i32),
        color: u32,
    ) {
        println!("Draw point: {} : {}", point.0, point.1);
        let index = flat_index(point.0 as u32, point.1 as u32, self.width);
        self.pixels[index] = color;
    }

    // Fill entire canvas with a color
    pub fn fill(
        &mut self,
        color: u32,
    ) {
        self.pixels = vec![color; (self.width * self.height) as usize];
    }

    // Fill a rectangle portion of the canvas
    pub fn fill_rect(
        &mut self,
        rect: &rectangle::Rectangle,
        color: u32,
    ) {
        for y in 0..rect.height as i32 {
            for x in 0..rect.width as i32 {
                let dx = x + rect.get_x();
                let dy = y + rect.get_y();

                // Check for bounds
                if dx < 0 || dy < 0 ||
                    dy >= self.height as i32 || dx >= self.width as i32 {
                    continue;
                }

                let src_index: usize = flat_index(dx as u32, dy as u32, self.width);


                self.pixels[src_index] = color;
            }
        }
    }

    // Scale image by factor
    pub fn scale_by(
        &mut self,
        factor: f32,
    ) {
        // Check for irrelevant scaling
        if factor == 1.0 {
            return;
        }

        let width: u32 = (self.width as f32 * factor) as u32;
        let height: u32 = (self.width as f32 * factor) as u32;
        let mut pixels: Vec<u32> = vec![0; (width * height) as usize];

        for i in 0..(width * height) {
            let (new_x, new_y) = from_index(i, width);
            let (ratio_x, ratio_y): (f64, f64) = (
                new_x as f64 / width as f64,
                new_y as f64 / height as f64
                );

            let (old_x, old_y): (u32, u32) = (
                ((ratio_x * self.width as f64).floor() as u32).min(self.width - 1),
                ((ratio_y * self.height as f64).floor() as u32).min(self.height - 1),
                );

            let old_index = flat_index(old_x, old_y, self.width);

            pixels[i as usize] = self.pixels[old_index];
        }

        self.width = width;
        self.height = height;
        self.pixels = pixels;
    }

    // Scale image to new size
    pub fn scale_to(
        &mut self,
        width: u32,
        height: u32,
    ) {
        // Check for irrelevant scaling
        if width == self.width && height == self.height {
            return;
        }

        let mut pixels: Vec<u32> = vec![0; (width * height) as usize];

        for i in 0..(width * height) {
            let (new_x, new_y) = from_index(i, width);
            let (ratio_x, ratio_y): (f64, f64) = (
                new_x as f64 / width as f64,
                new_y as f64 / height as f64
                );

            let (old_x, old_y): (u32, u32) = (
                ((ratio_x * self.width as f64).floor() as u32).min(self.width - 1),
                ((ratio_y * self.height as f64).floor() as u32).min(self.height - 1),
                );

            let old_index = flat_index(old_x, old_y, self.width);

            pixels[i as usize] = self.pixels[old_index];
        }

        self.width = width;
        self.height = height;
        self.pixels = pixels;
    }

    // Returns a rectangular zone from the canvas
    pub fn get_zone(
        &self,
        area: &rectangle::Rectangle,
    ) -> Self {
        // Check if the zone is the entire area

        println!("get_zone area dim: {}, {}, {}, {}", area.get_x(), area.get_y(), area.width, area.height);

        if area.get_x() == 0 && area.get_y() == 0 &&
            area.width >= self.width && area.height >= self.height {
            println!("return a clone");
            return self.clone();
        }

        let mut pixels: Vec<u32> = vec![0xFF000000; (area.width * area.height) as usize];

        for y in 0..area.height as i32 {
            for x in 0..area.width as i32 {
                let dy = y + area.get_y();
                let dx = x + area.get_x();

                // Check if out of bounds
                if dx < 0 || dy < 0 ||
                    dx >= self.width as i32 || dy >= self.height as i32 {
                    continue;
                }

                let pixels_index = flat_index(x as u32, y as u32, area.width);
                let canvas_index = flat_index(dx as u32, dy as u32, self.width);

                pixels[pixels_index] = self.pixels[canvas_index];
            }
        }

        Self {
            width: area.width,
            height: area.height,
            pixels,
        }
    }
}

// Specific methods
impl Canvas {
    // Add a new color to the canvas every 'steps' pixels
    pub fn dither(
        &mut self,
        color: u32,
        steps: u32,
    ) {
        for (current_steps, pixel) in self.pixels.iter_mut().enumerate() {
            if !current_steps.is_multiple_of(steps as usize) {
                continue;
            }

            *pixel = color;
        }
    }

    // Adds pixels to the surface to create a checkerboard pattern
    pub fn checkerboard(
        &mut self,
        color: u32,
    ) {
        if self.width % 2 == 1 {
            self.dither(color, 2);
            return;
        }
        let mut place: bool = false;
        for (current_steps, pixel) in self.pixels.iter_mut().enumerate() {
            if !current_steps.is_multiple_of(self.width as usize) {
                place = !place;
            }

            if !place {
                continue;
            }

            *pixel = color;
        }
    }
}
