use std::collections::HashSet;

use ggez;
use ggez::event;
use ggez::event::KeyCode;
use ggez::graphics;
use ggez::graphics::Color;
use ggez::nalgebra as na;

pub struct Tank {
    pub position: na::Point2<f32>,
    pub direction: na::Vector2<f32>,
    pub texture: Option<graphics::Image>,
}

impl event::EventHandler for Tank {
    fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        let keys = ggez::input::keyboard::pressed_keys(ctx);
        self.movement(keys);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        graphics::draw(
            ctx,
            self.texture.as_ref().unwrap(),
            (self.position, 0.0, Color::from_rgb(255, 255, 255)),
        )?;
        Ok(())
    }
}

impl Tank {
    pub fn movement(&mut self, keys: &HashSet<KeyCode>) {
        if keys.contains(&KeyCode::W) || keys.contains(&KeyCode::Up) {
            self.position = na::Point2::from([
                self.position.x + self.direction.x,
                self.position.y + self.direction.y,
            ]);
        }

        if keys.contains(&KeyCode::S) || keys.contains(&KeyCode::Down) {
            self.position = na::Point2::from([
                self.position.x - self.direction.x,
                self.position.y - self.direction.y,
            ]);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn initial_info() {
        let tank = tank();
        assert_eq!(tank.position, na::Point2::from([400., 300.]));
        assert_eq!(tank.direction, na::Vector2::from([-1., 0.]));
    }

    #[test]
    fn move_forward() {
        let mut tank = tank();
        let keys = vec![KeyCode::W].into_iter().collect();
        tank.movement(&keys);
        assert_eq!(tank.position, na::Point2::from([399., 300.]));
        let keys = vec![KeyCode::Up].into_iter().collect();
        tank.movement(&keys);
        assert_eq!(tank.position, na::Point2::from([398., 300.]));
    }

    #[test]
    fn move_backwards() {
        let mut tank = tank();
        let keys = vec![KeyCode::S].into_iter().collect();
        tank.movement(&keys);
        assert_eq!(tank.position, na::Point2::from([401., 300.]));
        let keys = vec![KeyCode::Down].into_iter().collect();
        tank.movement(&keys);
        assert_eq!(tank.position, na::Point2::from([402., 300.]));
    }

    fn tank() -> Tank {
        Tank {
            position: na::Point2::from([400., 300.]),
            direction: na::Vector2::from([-1., 0.]),
            texture: None,
        }
    }
}
