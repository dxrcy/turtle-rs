pub mod instructions;

use std::time::{Duration, Instant};

use ggez::{
    event::EventHandler,
    graphics::{self, DrawMode},
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
    pen_down: bool,

    instructions: Vec<Instruction>,
    drawn_lines: Vec<[Point2<f32>; 2]>,

    current_instruction: usize,
    is_first_run: bool,
    time_last_step: Instant,
}

impl App {
    pub fn new(ctx: &mut Context, instructions: Vec<Instruction>) -> Self {
        let (width, height) = ctx.gfx.size();
        let app = Self {
            position: Point2 {
                x: width / 2.0,
                y: height / 2.0,
            },
            direction: Direction::North,
            pen_down: false,

            instructions,
            drawn_lines: vec![],

            current_instruction: 0,
            is_first_run: true,
            time_last_step: Instant::now(),
        };
        app
    }

    fn reset(&mut self) {
        self.current_instruction = 0;
    }

    fn step_instruction(&mut self, ctx: &mut Context) {
        let Some(instr) = self.instructions.get(self.current_instruction) else {
            self.is_first_run = false;
            return;
        };
        self.current_instruction += 1;

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

                let previous_position = self.position;
                self.position = Point2 { x, y };

                if self.pen_down {
                    self.drawn_lines.push([previous_position, self.position]);
                }
            }

            Instruction::Face(dir) => {
                self.direction = *dir;
            }

            Instruction::Move(amount) => {
                let (x, y) = match self.direction {
                    Direction::North => (0.0, -*amount),
                    Direction::South => (0.0, *amount),
                    Direction::West => (-*amount, 0.0),
                    Direction::East => (*amount, 0.0),
                    Direction::NorthWest => (-*amount, -*amount),
                    Direction::SouthWest => (-*amount, *amount),
                    Direction::NorthEast => (*amount, -*amount),
                    Direction::SouthEast => (*amount, *amount),
                };

                let previous_position = self.position;
                self.position.x += x;
                self.position.y += y;

                if self.pen_down {
                    self.drawn_lines.push([previous_position, self.position]);
                }
            }

            Instruction::Pen(state) => {
                self.pen_down = *state;
            }

            Instruction::Clear => {
                self.drawn_lines.clear();
            }
        }
    }
}

impl EventHandler for App {
    fn update(&mut self, ctx: &mut Context) -> Result<(), ggez::GameError> {
        let step_timeout = Duration::from_millis(50);

        if self.is_first_run {
            let now = Instant::now();
            if now > self.time_last_step + step_timeout {
                self.step_instruction(ctx);
                self.time_last_step = now;
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> Result<(), ggez::GameError> {
        let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::BLACK);

        for points in &self.drawn_lines {
            let line = graphics::Mesh::new_line(ctx, points, 2.0, color!(WHITE))?;
            canvas.draw(&line, graphics::DrawParam::default());
        }

        let circle = graphics::Mesh::new_circle(
            ctx,
            DrawMode::fill(),
            self.position,
            3.0,
            0.1,
            color!(YELLOW),
        )?;
        canvas.draw(&circle, graphics::DrawParam::default());

        let text_lines = [
            self.current_instruction.to_string(),
            match self.instructions.get(self.current_instruction) {
                Some(instr) => format!("{:?}", instr),
                None => "[none]".to_string(),
            },
            format!("{}, {}", self.position.x, self.position.y),
            format!("{:?}", self.direction),
        ];

        let font_size = 15.0;
        for (i, line) in text_lines.into_iter().enumerate() {
            let mut text = graphics::Text::new(line);
            text.set_scale(font_size);
            let param = graphics::DrawParam::default()
                .dest(Point2 {
                    x: 5.0,
                    y: 5.0 + i as f32 * font_size,
                })
                .color(color!(YELLOW));
            canvas.draw(&text, param);
        }

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
            Some(KeyCode::R) => self.reset(),
            Some(KeyCode::Return) => {
                self.reset();
                self.is_first_run = true;
                self.time_last_step = Instant::now();
            }
            _ => (),
        }
        Ok(())
    }
}
