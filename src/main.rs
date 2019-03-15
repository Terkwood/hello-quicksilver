// Draw a clock with changed draw and update rates and disabled vsync
extern crate quicksilver;

use stdweb::web::Date;

use quicksilver::{
    geom::{Circle, Line, Vector},
    graphics::{Background::Col, Color},
    lifecycle::{run, Settings, State, Window},
    Result,
};

const RESOLUTION: (u32, u32) = (480, 480);
const MIDPOINT: (u32, u32) = (RESOLUTION.0 / 2, RESOLUTION.1 / 2);

struct Clock {
    hours: f64,
    minutes: f64,
    seconds: f64,
}

impl State for Clock {
    fn new() -> Result<Clock> {
        let now = Date::new();
        let min = now.get_minutes() as f64;
        let sec = now.get_seconds() as f64;
        Ok(Clock {
            hours: now.get_hours() as f64 + 1.0 / 60.0 * min + 1.0 / 60.0 / 60.0 * sec,
            minutes: min + 1.0 / 60.0 * sec,
            seconds: sec,
        })
    }

    fn update(&mut self, _window: &mut Window) -> Result<()> {
        let now = Clock::new()?;
        self.hours = now.hours;
        self.minutes = now.minutes;
        self.seconds = now.seconds;
        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        // clear everything
        window.clear(Color::BLACK)?;

        // draw the frame
        window.draw(&Circle::new(MIDPOINT, 203), Col(Color::WHITE));
        window.draw(&Circle::new(MIDPOINT, 200), Col(Color::BLACK));

        // draw the hour markers
        for i in 1..=12 {
            let angle = 360. * ((i as f64 + 9.) * 2. / 24.);
            let pos = Vector::from_angle(angle as f32) * 200. + Vector::new(MIDPOINT.0, MIDPOINT.1);
            let line = Line::new(MIDPOINT, pos).with_thickness(5);
            window.draw(&line, Col(Color::WHITE));
        }

        window.draw(&Circle::new(MIDPOINT, 180), Col(Color::BLACK));

        let hour_angle = 360. * ((self.hours as f64 + 9.) * 2. / 24.);
        let minute_angle = 360. * ((self.minutes as f64 + 45.) / 60.);
        let second_angle = 360. * ((self.seconds as f64 + 45.) / 60.);

        let hour_pos =
            Vector::from_angle(hour_angle as f32) * 150. + Vector::new(MIDPOINT.0, MIDPOINT.1);
        let min_pos =
            Vector::from_angle(minute_angle as f32) * 180. + Vector::new(MIDPOINT.0, MIDPOINT.1);
        let second_pos =
            Vector::from_angle(second_angle as f32) * 180. + Vector::new(MIDPOINT.0, MIDPOINT.1);

        let hour = Line::new(MIDPOINT, hour_pos).with_thickness(10);
        let minute = Line::new(MIDPOINT, min_pos).with_thickness(5);
        let second = Line::new(MIDPOINT, second_pos).with_thickness(3);

        window.draw(&hour, Col(Color::WHITE));
        window.draw(&minute, Col(Color::CYAN));
        window.draw(&second, Col(Color::PURPLE));

        Ok(())
    }
}

fn main() {
    run::<Clock>(
        "Clock",
        Vector::new(RESOLUTION.0, RESOLUTION.1),
        Settings {
            draw_rate: 1000. / 10., // 10 FPS are enough
            update_rate: 1000.,     // every second to make it appear like a clock
            vsync: false,           // don't use VSync, we're limiting to 10 FPS on our own
            ..Settings::default()
        },
    );
}
