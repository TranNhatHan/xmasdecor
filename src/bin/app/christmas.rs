use embassy_time::{Duration, Timer};
use rgb::RGB8;
use smart_leds_trait::SmartLedsWrite;

use crate::config::{
    BRIGHTNESS,
    FRAME_INTERVAL_MS,
    HEIGHT,
    NUM_LEDS,
    NUM_SNOWFLAKES,
    WIDTH,
};

const BLACK: RGB8 = RGB8::new(0, 0, 0);

pub struct Christmas {
    leds: [RGB8; NUM_LEDS],
    snow_columns: [usize; NUM_SNOWFLAKES],
    random_state: u32,
    frame: u32,
}

impl Christmas {

    fn new() -> Self {
        let mut christmas = Self {
            leds: [BLACK; NUM_LEDS],
            snow_columns: [0; NUM_SNOWFLAKES],
            random_state: 42,
            frame: 0,
        };
        for i in 0..NUM_SNOWFLAKES {
            christmas.snow_columns[i] = christmas.random(0, WIDTH as u32) as usize;
        }
        christmas
    }

    fn random(&mut self, min: u32, max: u32) -> u32 {
        if min >= max {
            return min;
        }

        self.random_state ^= self.random_state << 13;
        self.random_state ^= self.random_state >> 17;
        self.random_state ^= self.random_state << 5;

        min + self.random_state % (max - min)
    }

    fn xy(x: usize, y: usize) -> Option<usize> {

        if x >= WIDTH || y >= HEIGHT {
            return None;
        }

        let index = if y % 2 == 0 {
            y * WIDTH + x
        } else {
            y * WIDTH + (WIDTH - 1 - x)
        };

        Some(index)
    }

    fn set_pixel( &mut self, x: usize, y: usize, colour: RGB8) {
        if let Some(index) = Self::xy(x, y) {
            self.leds[index] = colour;
        }
    }

    fn clear(&mut self) {
      self.leds.fill(BLACK);
    }

    fn draw_tree(&mut self) {
        let green = RGB8::new(0, 180, 25);
        let branches: [(usize, usize); 11] = [
            (6, 7), (5, 8), (5, 8), 
            (4, 9), (4, 9), (3, 10), (3, 10), 
            (2, 11), (2, 11), (1, 12), (1, 12),
        ];
        for (row, &(left, right)) in branches.iter().enumerate() {
            let y = row + 3;
            for x in left..=right {
                self.set_pixel(x, y, green);
            }
        }

        let brown = RGB8::new(120, 55, 15);
        for y in 14..=16 {
            self.set_pixel(6, y, brown);
            self.set_pixel(7, y, brown);
        }
    }

    fn draw_star(&mut self) {
        let phase = self.frame as f32 * 0.24;
        let brightness = (150.0 + 105.0 * (0.5 + 0.5 * libm::sinf(phase))) as u8;

        let yellow = RGB8::new(brightness, (brightness as u16 * 65 / 100) as u8, 0);

        let positions = [
            (6, 0), (7, 0),
            (6, 2), (7, 2),
            (5, 1), (6, 1),
            (7, 1), (8, 1),
        ];

        for (x, y) in positions {
            self.set_pixel(x, y, yellow);
        }
    }

    fn draw_ornaments(&mut self) {

        let ornaments = [
            (6, 4), (4, 13), (5, 6), (7, 7), (4, 8), (9, 8), (6, 9), 
            (8, 10), (3, 10), (10, 11), (5, 11), (7, 12), (9, 13), (2, 12)
        ];

        let colours = [
            RGB8::new(255, 0, 0),
            RGB8::new(0, 80, 255),
            RGB8::new(255, 180, 0),
            RGB8::new(255, 0, 180),
            RGB8::new(255, 255, 255),
        ];

        for (x, y) in ornaments {
            if self.random(0, 5) == 0 {
                continue;
            }
            let colour_index = self.random(0, colours.len() as u32) as usize;
            let colour = colours[colour_index];
            self.set_pixel(x, y, colour);
        }
    }

    fn draw_snow(&mut self) {
        let snow = RGB8::new(200, 200, 200);
        for i in 0..NUM_SNOWFLAKES {
            let x = self.snow_columns[i];
            let speed = 2 + (i % 3);
            let y = ((self.frame / speed as u32) % HEIGHT as u32) as usize;
            self.set_pixel(x, y, snow);
        }
    }


    fn draw_presents(&mut self) {
        let red = RGB8::new(255, 0, 0);
        let blue = RGB8::new(0, 40, 255);
        let gold = RGB8::new(255, 180, 0);

        for y in 15..=17 {
            for x in 0..=3 {
                self.set_pixel(x, y, red);
            }
            self.set_pixel(1, y, gold);
        }
        for x in 0..=3 {
            self.set_pixel(x, 16, gold);
        }

        for y in 15..=17 {
            for x in 10..=13 {
                self.set_pixel(x, y, blue);
            }
            self.set_pixel(12, y, gold);
        }
        for x in 10..=13 {
            self.set_pixel(x, 16, gold);
        }
    }

    fn draw_snow_floor(&mut self) {
        let white = RGB8::new(200, 200, 200);
        for x in 4..WIDTH - 4 {
            self.set_pixel(x, 17, white);
        }
    }

    fn render(&mut self) {
        self.clear();
        self.draw_snow();
        self.draw_tree();
        self.draw_star();
        self.draw_ornaments();
        self.draw_presents();
        self.draw_snow_floor();
        self.frame = self.frame.wrapping_add(1);
    }
}

fn scale_brightness(value: u8) -> u8 {
    (value as u16 * BRIGHTNESS as u16 / 255) as u8
}

pub async fn run(mut led: impl SmartLedsWrite<Color = RGB8>) -> ! {
    let mut christmas = Christmas::new();

    loop {
        christmas.render();

        let frame = christmas.leds.iter().copied().map(
            |pixel| {
                RGB8::new(
                    scale_brightness(pixel.r),
                    scale_brightness(pixel.g),
                    scale_brightness(pixel.b),
                )
            }
        );

        if led.write(frame).is_err() {
            // Handle LED transmission failure.
        }

        Timer::after(Duration::from_millis(FRAME_INTERVAL_MS)).await;
    }
}
