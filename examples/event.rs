use puppetmaster::EventInputHandler;

use ggez::{
    conf::WindowSetup,
    event::{self, EventHandler},
    graphics::{self, Color, DrawMode, DrawParam, FillOptions, Mesh},
    input::keyboard::{KeyCode, KeyInput},
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
    controls: EventInputHandler<KeyCode, Control>,
}

fn main() {
    let (ctx, event_loop) = ContextBuilder::new("event_example", "gamma-delta")
        .window_setup(WindowSetup {
            title: String::from("EventInputHandler Example"),
            ..Default::default()
        })
        .build()
        .unwrap();

    let controls = EventInputHandler::new_with_controls(vec![
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
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.controls.update();

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
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);

        let size = 16.0;
        let (w, h) = ctx.gfx.drawable_size();

        // Things are drawn from their upper-left corner
        let circle = Mesh::new_circle(
            ctx,
            DrawMode::Fill(FillOptions::default()),
            [self.x - size + w / 2.0, self.y - size + h / 2.0],
            size,
            0.1,
            Color::RED,
        )?;
        canvas.draw(&circle, DrawParam::default());

        canvas.finish(ctx)?;
        timer::yield_now();
        Ok(())
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keyinput: KeyInput,
        _repeat: bool,
    ) -> GameResult {
        if let Some(kc) = keyinput.keycode {
            self.controls.on_input_down(kc);
        }
        Ok(())
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keyinput: KeyInput) -> GameResult {
        if let Some(kc) = keyinput.keycode {
            self.controls.on_input_up(kc);
        }
        Ok(())
    }
}
