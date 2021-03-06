use std::path;

use ggez;
use ggez::conf::WindowSetup;
use ggez::event;

mod objects;
mod state;

use state::{window_state_mode, MainState};

pub fn main() -> ggez::GameResult {
    let resources_dir = path::PathBuf::from("./resources");
    let cb = ggez::ContextBuilder::new("tank_survive", "naomijub")
        .add_resource_path(resources_dir)
        .window_setup(WindowSetup {
            title: "Tank Survive Ground".to_owned(),
            samples: ggez::conf::NumSamples::Zero,
            vsync: true,
            icon: String::new(),
            srgb: true,
        })
        .window_mode(window_state_mode());

    let (ctx, event_loop) = &mut cb.build()?;
    let mut state = MainState::new(ctx, state::Player::P1)?;
    event::run(ctx, event_loop, &mut state)
}
