use ggez::conf::WindowMode;
use ggez::event;
use ggez::ContextBuilder;
use ggez::GameResult;

use turtle::App;

fn main() -> GameResult {
    let file = include_str!("instructions");
    let instructions =
        turtle::instructions::parse_file(file).expect("Failed to parse instructions");
    println!("{:#?}", instructions);

    let window_mode = WindowMode::default().dimensions(600.0, 400.0);

    // Create app context
    let (mut ctx, event_loop) = ContextBuilder::new("turtle", "darcy")
        .window_mode(window_mode)
        .build()?;

    // Change window properties
    ctx.gfx.set_window_title("turtle");

    // Create app state
    let app = App::new(&mut ctx, instructions);

    // Run game loop
    event::run(ctx, event_loop, app);
}

