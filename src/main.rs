#![warn(clippy::all, clippy::pedantic)]
mod map;

mod prelude {
    pub use bracket_lib::prelude::*;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub use crate::map::*;
}

struct State {}

impl prelude::GameState for State {
    fn tick(&mut self, ctx: &mut prelude::BTerm) {
        ctx.cls();
        ctx.print(1, 1, "Hello!");
    }
}
fn main() -> prelude::BError {
    let context = prelude::BTermBuilder::simple80x50()
        .with_title("Rogue")
        .build()?;
    prelude::main_loop(context, State {})
}
