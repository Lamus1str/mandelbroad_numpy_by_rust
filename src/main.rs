extern crate speedy2d;

use speedy2d::Graphics2D;
use speedy2d::Window;
use speedy2d::window::WindowHelper;
use speedy2d::window::WindowHandler;

use std::ops;
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
        drawed: false
    })
}

struct MyWindowHandler
{
    drawed: bool,
}

impl WindowHandler for MyWindowHandler
{
    fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D)
    {
        if self.drawed{
            return
        }
        self.drawed = true;
        for x in 0..1200{
            for y in 0..1200{
                let c = c32 { a: (x as f64 - 600.0) * 0.003, b: (y as f64 - 600.0) * 0.003 };
                let mut z = c32 { a: 0.0, b: 0.0};
                for n in 0..200{
                    z = (z * z) + c;
                    if (z.len() > 2.0) {
                        graphics.draw_line(Vec2::new(x as f32 / 2.0, y as f32 / 2.0), Vec2::new(x as f32 / 2.0 + 1.0, y as f32 / 2.0),
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
        helper.request_redraw();
    }
}
