mod instructions;
mod keyboard;
mod machine;
mod screen;

use crate::keyboard::Keyboard;
use crate::machine::Machine;
use crate::screen::Screen;
use piston_window::types::Color;
use piston_window::*;
use std::fs::read;
use std::time::{Duration, Instant};

const BACK_COLOR: [f32; 4] = [0.2, 0.2, 0.2, 1.0];
const RATIO: f64 = 20.0;
const EMULATOR_RATE: u64 = 1851; //540 Hz

fn main() {
    let (width, height) = (64, 32);

    let mut window: PistonWindow = WindowSettings::new(
        "CMSC388Z Snake Game",
        [
            ((width as f64) * RATIO) as u32,
            ((height as f64) * RATIO) as u32,
        ],
    )
    .exit_on_esc(true)
    .build()
    .unwrap();

    let f = read("chipquarium.ch8").expect("file not found");

    let mut screen = Screen::new();
    let mut keyboard = Keyboard::new();
    let mut machine = Machine::new();

    machine.load(&f);

    let mut last_tick = Instant::now();

    while let Some(event) = window.next() {
        if last_tick.elapsed() >= Duration::from_nanos(EMULATOR_RATE) {
            machine.step(&keyboard, &mut screen);
            last_tick = Instant::now();
        }

        if let Some(Button::Keyboard(key)) = event.press_args() {
            match key {
                Key::Up => {
                    keyboard.press(1);
                    keyboard.press(2);
                },
                Key::Down => {
                    keyboard.press(4);
                    keyboard.press(8);
                },
                Key::W => keyboard.press(11),
                Key::A => keyboard.press(12),
                Key::S => keyboard.press(13),
                Key::D => keyboard.press(14),
                Key::Space => keyboard.press(15),
                _ => {}
            }
        }

        if let Some(Button::Keyboard(key)) = event.release_args() {
            match key {
                Key::Up => {
                    keyboard.release(1);
                    keyboard.release(2);
                },
                Key::Down => {
                    keyboard.release(4);
                    keyboard.release(8);
                },
                Key::W => keyboard.release(11),
                Key::A => keyboard.release(12),
                Key::S => keyboard.release(13),
                Key::D => keyboard.release(14),
                Key::Space => keyboard.release(15),
                _ => {}
            }
        }

        window.draw_2d(&event, |c, g, _| {
            clear(BACK_COLOR, g);
            for i in 0..32 {
                for j in 0..64 {
                    match screen.get(j, i) {
                        x if x > 0 => {
                            let x = x as f32 / 100.0;
                            let clr: Color = [x, x, x, 1.0];
                            draw_block(clr, j as i32, i as i32, &c, g);
                        }
                        _ => {}
                    }
                }
            }
        });
    }
}

pub fn draw_block(color: Color, x: i32, y: i32, con: &Context, g: &mut G2d) {
    let gui_x = (x as f64) * RATIO;
    let gui_y = (y as f64) * RATIO;

    rectangle(color, [gui_x, gui_y, RATIO, RATIO], con.transform, g);
}
