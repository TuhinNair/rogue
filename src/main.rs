#![warn(clippy::all, clippy::pedantic)]

use prelude::Map;
mod map;

mod prelude {
    pub use bracket_lib::prelude::*;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub use crate::map::*;
}

struct State {
    map: Map,
}

impl State {
    fn new() -> Self {
        Self { map: Map::new() }
    }
}
impl prelude::GameState for State {
    fn tick(&mut self, ctx: &mut prelude::BTerm) {
        ctx.cls();
        self.map.render(ctx);
    }
}
fn main() -> prelude::BError {
    let context = prelude::BTermBuilder::simple80x50()
        .with_title("Rogue")
        .with_fps_cap(30.0)
        .build()?;
    prelude::main_loop(context, State::new())
}
