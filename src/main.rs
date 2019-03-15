// Draw a clock with changed draw and update rates and disabled vsync
extern crate quicksilver;

use stdweb::web::Date;

use quicksilver::{
    geom::{Circle, Line, Vector},
    graphics::{Background::Col, Color},
    lifecycle::{run, Settings, State, Window},
    Result,
};

struct Clock {
    hours: u32,
    minutes: u32,
    seconds: u32,
}

impl State for Clock {
    fn new() -> Result<Clock> {
        let now = Date::new();
        Ok(Clock {
            hours: now.get_hours() as u32,
            minutes: now.get_minutes() as u32,
            seconds: now.get_seconds() as u32,
        })
    }

    fn update(&mut self, _window: &mut Window) -> Result<()> {
        let now = Date::new();
        self.hours = now.get_hours() as u32;
        self.minutes = now.get_minutes() as u32;
        self.seconds = now.get_seconds() as u32;
        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        // clear everything
        window.clear(Color::BLACK)?;

        // draw the frame
        window.draw(&Circle::new((400, 300), 203), Col(Color::WHITE));
        window.draw(&Circle::new((400, 300), 200), Col(Color::BLACK));

        // draw the hour markers
        for i in 1..=12 {
            let angle = 360. * ((i as f64 + 9.) * 2. / 24.);
            let pos = Vector::from_angle(angle as f32) * 200. + Vector::new(400, 300);
            let line = Line::new((400, 300), pos).with_thickness(5);
            window.draw(&line, Col(Color::WHITE));
        }

        window.draw(&Circle::new((400, 300), 180), Col(Color::BLACK));

        let hour_angle = 360. * ((self.hours as f64 + 9.) * 2. / 24.);
        let minute_angle = 360. * ((self.minutes as f64 + 45.) / 60.);
        let second_angle = 360. * ((self.seconds as f64 + 45.) / 60.);

        let hour_pos = Vector::from_angle(hour_angle as f32) * 150. + Vector::new(400, 300);
        let min_pos = Vector::from_angle(minute_angle as f32) * 180. + Vector::new(400, 300);
        let second_pos = Vector::from_angle(second_angle as f32) * 180. + Vector::new(400, 300);

        let hour = Line::new((400, 300), hour_pos).with_thickness(10);
        let minute = Line::new((400, 300), min_pos).with_thickness(5);
        let second = Line::new((400, 300), second_pos).with_thickness(3);

        window.draw(&hour, Col(Color::WHITE));
        window.draw(&minute, Col(Color::CYAN));
        window.draw(&second, Col(Color::PURPLE));

        Ok(())
    }
}

fn main() {
    run::<Clock>(
        "Clock",
        Vector::new(800, 600),
        Settings {
            draw_rate: 1000. / 10., // 10 FPS are enough
            update_rate: 1000.,     // every second to make it appear like a clock
            vsync: false,           // don't use VSync, we're limiting to 10 FPS on our own
            ..Settings::default()
        },
    );
}
