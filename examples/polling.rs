use puppetmaster::PollingInputHandler;

use ggez::{
    conf::WindowSetup,
    event::{self, EventHandler},
    graphics::{self, Color, DrawMode, DrawParam, FillOptions, Mesh},
    input::keyboard::{self, KeyCode},
    timer, Context, ContextBuilder, GameResult,
};

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
enum Control {
    Up,
    Down,
    Left,
    Right,
    Speed,
    Return,
}

struct MainState {
    x: f32,
    y: f32,
    controls: PollingInputHandler<KeyCode, Control>,
}

fn main() {
    let (ctx, event_loop) = ContextBuilder::new("polling_example", "gamma-delta")
        .window_setup(WindowSetup {
            title: String::from("PollingInputHandler Example"),
            ..Default::default()
        })
        .build()
        .unwrap();

    let controls = PollingInputHandler::new_with_controls(vec![
        (KeyCode::W, Control::Up),
        (KeyCode::A, Control::Left),
        (KeyCode::S, Control::Down),
        (KeyCode::D, Control::Right),
        (KeyCode::Return, Control::Speed),
        (KeyCode::Q, Control::Return),
    ]);
    let state = MainState {
        x: 0.0,
        y: 0.0,
        controls,
    };

    println!("WASD to move, Enter to move faster, Q to return to the center.");
    event::run(ctx, event_loop, state)
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.controls.update(keyboard::pressed_keys(ctx).clone());

        if self.controls.clicked(Control::Return) {
            self.x = 0.0;
            self.y = 0.0;
        } else {
            let speed = if self.controls.down(Control::Speed) {
                10.0
            } else {
                5.0
            };
            if self.controls.down(Control::Left) {
                self.x -= speed;
            }
            if self.controls.down(Control::Right) {
                self.x += speed;
            }
            if self.controls.down(Control::Up) {
                self.y -= speed;
            }
            if self.controls.down(Control::Down) {
                self.y += speed;
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, Color::BLACK);

        let size = 16.0;
        let (w, h) = graphics::size(ctx);

        // Things are drawn from their upper-left corner
        let circle = Mesh::new_circle(
            ctx,
            DrawMode::Fill(FillOptions::default()),
            [self.x - size + w / 2.0, self.y - size + h / 2.0],
            size,
            0.1,
            Color::RED,
        )?;
        graphics::draw(ctx, &circle, DrawParam::default())?;

        graphics::present(ctx)?;
        timer::yield_now();

        Ok(())
    }
}
