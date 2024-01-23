extern crate speedy2d;

use speedy2d::Graphics2D;
use speedy2d::Window;
use speedy2d::window::{KeyScancode, MouseButton, VirtualKeyCode, WindowHelper};
use speedy2d::window::WindowHandler;

use std::ops;
use std::panic::resume_unwind;
use std::time::Instant;
use speedy2d::color::Color;
use speedy2d::dimen::Vec2;

#[derive(Copy, Clone)]
struct c32{
    a: f64,
    b: f64,
}

impl c32{
    fn len(&self) -> f64{
        f64::sqrt(self.a * self.a + self.b * self.b)
    }
}

impl ops::Mul for c32{
    type Output = c32;
    fn mul(self, rhs: Self) -> Self::Output {
        c32 {
            a: self.a * rhs.a + -1.0 * self.b * rhs.b,
            b: self.a * rhs.b + self.b * rhs.a,
        }
    }
}

impl ops::Add for c32{
    type Output = c32;
    fn add(self, rhs: Self) -> Self::Output {
        c32{
            a: self.a + rhs.a,
            b: self.b + rhs.b,
        }
    }
}

fn main()
{
    let window = Window::new_centered("Mandelbread", (600, 600)).unwrap();

    window.run_loop(MyWindowHandler {
        scale: 500.0,
        drawed: false,
        centre: Vec2::ZERO,
        mouse_pos: Vec2::ZERO,
    })
}

struct MyWindowHandler
{
    centre: Vec2,
    scale: f64,
    drawed: bool,
    mouse_pos: Vec2,
}

impl WindowHandler for MyWindowHandler
{
    fn on_mouse_move(&mut self, helper: &mut WindowHelper<()>, position: Vec2) {
        self.mouse_pos = (position - Vec2::new(300.0, 300.0)) / self.scale as f32;
    }
    fn on_mouse_button_down(&mut self, helper: &mut WindowHelper<()>, button: MouseButton) {
        println!("{:?}", self.mouse_pos);
        self.centre += self.mouse_pos;
        self.drawed = false;
        helper.request_redraw();
    }
    fn on_key_down(&mut self, helper: &mut WindowHelper<()>, virtual_key_code: Option<VirtualKeyCode>, scancode: KeyScancode) {
        if let Some(key_code) = virtual_key_code{
            println!("key");
            if key_code == VirtualKeyCode::Equals{
                print!("fff");
                self.scale *= 2.0;
            }
            else if key_code == VirtualKeyCode::Minus {
                self.scale *= 0.5;
            }
            else{
                return;
            }
            helper.request_redraw();
        }
    }
    fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D)
    {
        if self.drawed{
            return
        }
        self.drawed = true;
        graphics.clear_screen(Color::BLACK);
        println!("drawing");
        for x in 0..600{
            for y in 0..600{
                let c = c32 { a: (x as f64 - 300.0) / self.scale + self.centre.x as f64,
                              b: (y as f64 - 300.0) / self.scale + self.centre.y as f64};
                let mut z = c32 { a: 0.0, b: 0.0};
                for n in 0..600{
                    z = (z * z) + c;
                    if (z.len() > 2.0) {
                        graphics.draw_line(Vec2::new(x as f32 / 1.0, y as f32 / 1.0), Vec2::new(x as f32 / 1.0 + 1.0, y as f32 / 1.0),
                                           1.0, Color::from_gray(n as f32 / 64.0));
                        break;
                    }
                    else{
                            //graphics.draw_line(Vec2::new(x as f32, y as f32), Vec2::new(x as f32 + 1.0, y as f32),
                            //         1.0, Color::from_gray((z.len() / 2.0) as f32));
                    }
                }
            }
        }
        println!("Success!");
        helper.request_redraw();
    }
}
