use ggez;
use ggez::event;
use ggez::graphics;
use ggez::graphics::Color;
use ggez::nalgebra as na;

pub struct Tank {
    pub position: na::Point2<f32>,
    pub texture: graphics::Image,
}

impl event::EventHandler for Tank {
    fn update(&mut self, _ctx: &mut ggez::Context) -> ggez::GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        graphics::draw(
            ctx,
            &self.texture,
            (self.position, 0.0, Color::from_rgb(255, 255, 255)),
        )?;
        Ok(())
    }
}