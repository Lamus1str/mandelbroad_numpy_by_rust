extern crate speedy2d;

use speedy2d::Graphics2D;
use speedy2d::Window;
use speedy2d::window::{KeyScancode, MouseButton, VirtualKeyCode, WindowHelper};
use speedy2d::window::WindowHandler;

use std::ops;
use std::panic::resume_unwind;
use std::time::Instant;
use speedy2d::color::Color;
use speedy2d::dimen::{Vec2, Vector2};

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

fn get_color(n: usize, offset: f32) -> Color{
    let mut new_n = n as f32;
    Color::from_rgb(
        (-(new_n * 0.02).cos() * 0.5 + 0.5) * 0.6,
        (-(new_n * 0.1).cos() * 0.5 + 0.5),
        (-(new_n * 0.03).cos() * 0.5 + 0.5) * 0.8
    )
    //Color::from_rgb((new_n) * 0.008,
    //                (new_n) * 0.032,
    //                (new_n) * 0.032 - 0.64)
}

const WIN_SIZE: (u32, u32) = (640, 640);

fn main()
{
    let window = Window::new_centered("Mandelbrot's set", WIN_SIZE).unwrap();

    window.run_loop(MyWindowHandler {
        scale: 400.0,
        drawed: false,
        centre: Vector2::ZERO,
        mouse_pos: Vector2::ZERO,
        degree: 32,
        mul_or_add: true,
    })
}

struct MyWindowHandler
{
    centre: Vector2<f64>,
    scale: f64,
    drawed: bool,
    mouse_pos: Vector2<f64>,
    degree: usize,
    mul_or_add: bool,
}

impl WindowHandler for MyWindowHandler
{
    fn on_mouse_move(&mut self, helper: &mut WindowHelper<()>, position: Vec2) {
        let pos_f64 = Vector2::<f64>::new(position.x as f64, position.y as f64);
        self.mouse_pos = (pos_f64 -
            Vector2::<f64>::new(WIN_SIZE.0 as f64 / 2.0, WIN_SIZE.1 as f64 / 2.0))
            / self.scale as f64;
    }
    fn on_mouse_button_down(&mut self, helper: &mut WindowHelper<()>, button: MouseButton) {
        //println!("{:?}", self.mouse_pos);
        self.centre += self.mouse_pos;
        helper.request_redraw();
    }
    fn on_key_down(&mut self, helper: &mut WindowHelper<()>, virtual_key_code: Option<VirtualKeyCode>, scancode: KeyScancode) {
        let mut redraw = false;
        if let Some(key_code) = virtual_key_code{
            if key_code == VirtualKeyCode::A{
                if self.mul_or_add{
                    self.degree *= 2;
                    redraw = true;
                }
                else{
                    self.degree += 1;
                    redraw = true;
                }
            }
            else if key_code == VirtualKeyCode::S{
                if self.mul_or_add{
                    self.degree /= 2;
                    redraw = true;
                }
                else{
                    self.degree -= 1;
                    redraw = true;
                }
            }
            else if key_code == VirtualKeyCode::Equals{
                println!("scale+");
                self.scale *= 2.0;
                redraw = true;
            }
            else if key_code == VirtualKeyCode::Minus {
                self.scale *= 0.5;
                println!("scale-");
                redraw = true;
            }
            else if key_code == VirtualKeyCode::LControl {
                self.mul_or_add = !self.mul_or_add;
                redraw = true;
            }
            else{
                return;
            }
            if redraw{
                helper.request_redraw();
            }
        }
    }
    fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D)
    {
        self.drawed = true;
        graphics.clear_screen(Color::BLACK);
        println!("drawing");
        for x in 0..WIN_SIZE.0{
            for y in 0..WIN_SIZE.1{
                let c = c32 {
                    a: (x as f64 - WIN_SIZE.0 as f64 / 2.0) / self.scale + self.centre.x,
                    b: (y as f64 - WIN_SIZE.1 as f64 / 2.0) / self.scale + self.centre.y
                };
                let mut z = c32 { a: 0.0, b: 0.0};
                for n in 0..self.degree{
                    z = (z * z) + c;
                    if (z.len() > 2.0) {
                        //let color = Color::from_rgb(0.0, n as f32 / self.degree as f32 * 1.6 + 0.05, 0.0);
                        graphics.draw_line(
                            Vec2::new(x as f32 / 1.0, y as f32 / 1.0),
                            Vec2::new(x as f32 / 1.0 + 1.0, y as f32 / 1.0),
                            1.0, get_color(n, 1.0 / self.scale as f32));
                        break;
                    }
                    else{
                            //graphics.draw_line(Vec2::new(x as f32, y as f32), Vec2::new(x as f32 + 1.0, y as f32),
                            //         1.0, Color::from_gray((z.len() / 2.0) as f32));
                    }
                }
            }
        }
        println!(
            "Success! Zoom {}(1/pixel size); position: ({}, {})",
            self.scale, self.centre.x, self.centre.y
        );
        //helper.request_redraw();
    }
}
