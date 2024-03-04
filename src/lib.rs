pub mod instructions;

use ggez::{
    event::EventHandler,
    graphics::{self, DrawMode, DrawParam},
    input::keyboard::KeyCode,
    mint::Point2,
    Context,
};
use instructions::{Coord, Direction, Instruction};

/// Returns `ggez::graphics::Color` value, as const
macro_rules! color {
    ($name:ident $(,)?) => {
        ::ggez::graphics::Color::$name
    };
    ($hex:literal $(,)?) => {
        color!(($hex >> 16) & 0xFF, ($hex >> 8) & 0xFF, $hex & 0xFF,)
    };
    ($r:expr, $g:expr, $b:expr $(,)?) => {
        ::ggez::graphics::Color::new(
            $r as u8 as f32 / 255.0,
            $g as u8 as f32 / 255.0,
            $b as u8 as f32 / 255.0,
            255.0,
        )
    };
    ($r:expr, $g:expr, $b:expr, $a:expr $(,)?) => {
        ::ggez::graphics::Color::new(
            $r as u8 as f32 / 255.0,
            $g as u8 as f32 / 255.0,
            $b as u8 as f32 / 255.0,
            $a as u8 as f32 / 255.0,
        )
    };
}

pub struct App {
    position: Point2<f32>,
    direction: Direction,
    pen_state: bool,

    instructions: Vec<Instruction>,
    current_instruction: usize,
}

impl App {
    pub fn new(ctx: &mut Context, instructions: Vec<Instruction>) -> Self {
        let mut app = Self {
            position: Point2 { x: 0.0, y: 0.0 },
            direction: Direction::Up,
            pen_state: false,

            instructions,
            current_instruction: 0,
        };
        app.step_instruction(ctx);

        app
    }

    fn step_instruction(&mut self, ctx: &mut Context) {
        let Some(instr) = self.instructions.get(self.current_instruction) else {
            return;
        };
        self.current_instruction += 1;
        if self.current_instruction >= self.instructions.len() {
            self.current_instruction = 0;
        }

        println!("{:?}", instr);
        let (width, height) = ctx.gfx.size();

        match instr {
            Instruction::Goto(x, y) => {
                let x = match x {
                    Coord::Absolute(value) => *value,
                    Coord::Center => width / 2.0,
                };
                let y = match y {
                    Coord::Absolute(value) => *value,
                    Coord::Center => height / 2.0,
                };
                self.position = Point2 { x, y };
            }

            Instruction::Face(dir) => {
                self.direction = *dir;
            }

            Instruction::Move(amount) => {
                let (x, y) = match self.direction {
                    Direction::Left => (-*amount, 0.0),
                    Direction::Right => (*amount, 0.0),
                    Direction::Up => (0.0, -*amount),
                    Direction::Down => (0.0, *amount),
                };
                self.position.x += x;
                self.position.y += y;
            }

            Instruction::Pen(state) => {
                self.pen_state = *state;
            }
        }
    }
}

impl EventHandler for App {
    fn update(&mut self, _ctx: &mut Context) -> Result<(), ggez::GameError> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> Result<(), ggez::GameError> {
        let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::BLACK);

        let circle = graphics::Mesh::new_circle(
            ctx,
            DrawMode::fill(),
            self.position,
            8.0,
            0.1,
            color!(YELLOW),
        )?;
        canvas.draw(&circle, graphics::DrawParam::default());

        let mut text = graphics::Text::new(format!("{}", self.current_instruction,));
        text.set_scale(15.0);
        let param = DrawParam::default()
            .dest(Point2 { x: 10.0, y: 10.0 })
            .color(color!(YELLOW));
        canvas.draw(&text, param);
        let mut text = graphics::Text::new(format!("{:?}", self.direction));
        text.set_scale(15.0);
        let param = DrawParam::default()
            .dest(Point2 { x: 10.0, y: 30.0 })
            .color(color!(YELLOW));
        canvas.draw(&text, param);

        canvas.finish(ctx)
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        input: ggez::input::keyboard::KeyInput,
        _repeated: bool,
    ) -> Result<(), ggez::GameError> {
        match input.keycode {
            Some(KeyCode::Space) => self.step_instruction(ctx),
            _ => (),
        }
        Ok(())
    }
}
