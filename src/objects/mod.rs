use std::collections::HashSet;

use ggez;
use ggez::event;
use ggez::event::KeyCode;
use ggez::graphics::Rect;
use ggez::graphics::{self, screen_coordinates};
use ggez::input::mouse;
use ggez::nalgebra as na;
use ggez::timer::delta;

use crate::state::Player;

pub mod bullet;

pub struct Tank {
    pub position: na::Point2<f32>,
    pub tank_direction: na::Vector2<f32>,
    pub tank_rotation: f32,
    pub texture: Option<graphics::Image>,
    pub turret_texture: Option<graphics::Image>,
    pub turret_direction: na::Vector2<f32>,
    pub turret_rotation: f32,
    pub turret_width: f32,
    pub player: Player,
}

impl event::EventHandler for Tank {
    fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        let keys = ggez::input::keyboard::pressed_keys(ctx);
        let delta = delta(ctx).as_secs_f32();
        let screen_coord = screen_coordinates(ctx);
        let mouse_position = mouse::position(ctx);
        self.movement(keys, screen_coord);
        self.rotation(keys, delta);
        self.update_turret_direction(mouse_position.into());
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        let base_param = graphics::DrawParam::new()
            .dest(self.position)
            .offset(na::Point2::from([0.5, 0.5]))
            .rotation(self.tank_rotation);
        let turret_param = graphics::DrawParam::new()
            .dest(self.position)
            .offset(na::Point2::from([0.5, 0.5]))
            .src(self.get_player_turret())
            .rotation(self.turret_rotation);

        graphics::draw(ctx, self.texture.as_ref().unwrap(), base_param)?;
        graphics::draw(ctx, self.turret_texture.as_ref().unwrap(), turret_param)?;
        Ok(())
    }
}

impl Tank {
    pub fn movement(&mut self, keys: &HashSet<KeyCode>, screen_coord: Rect) {
        if keys.contains(&KeyCode::W) || keys.contains(&KeyCode::Up) {
            self.update_position(1., screen_coord);
        }

        if keys.contains(&KeyCode::S) || keys.contains(&KeyCode::Down) {
            self.update_position(-1., screen_coord);
        }
    }

    pub fn rotation(&mut self, keys: &HashSet<KeyCode>, delta: f32) {
        if keys.contains(&KeyCode::D) || keys.contains(&KeyCode::Right) {
            self.tank_rotation += delta;
            self.update_direction();
        }

        if keys.contains(&KeyCode::A) || keys.contains(&KeyCode::Left) {
            self.tank_rotation -= delta;
            self.update_direction();
        }
    }

    pub fn update_turret_direction(&mut self, mouse_position: na::Point2<f32>) {
        let mouse = na::Vector2::from([mouse_position.x, mouse_position.y]);
        let origin = na::Vector2::from([self.position.x, self.position.y]);
        let direction: na::Vector2<f32> = (mouse - origin).into();
        let angle = (mouse.y - origin.y).atan2(origin.x - mouse.x);

        self.turret_direction = direction.normalize();
        self.turret_rotation = -angle;
    }

    fn update_direction(&mut self) {
        let (sin, cos) = self.tank_rotation.sin_cos();
        self.tank_direction = na::Vector2::from([-cos, -sin]);
    }

    fn update_position(&mut self, direction: f32, screen_coord: Rect) {
        let new_position = na::Point2::from([
            self.position.x + (direction * self.tank_direction.x),
            self.position.y + (direction * self.tank_direction.y),
        ]);

        if screen_coord.contains(new_position) {
            self.position = new_position;
        }
    }

    pub(crate) fn get_player_turret(&self) -> Rect {
        let draw_height = 0.5;
        let draw_width = 0.5;

        match self.player {
            Player::P1 => Rect {
                x: 0.,
                y: 0.,
                h: draw_height,
                w: draw_width,
            },
            Player::P2 => Rect {
                x: draw_width,
                y: 0.,
                h: draw_height,
                w: draw_width,
            },
            Player::P3 => Rect {
                x: 0.,
                y: draw_height,
                h: draw_height,
                w: draw_width,
            },
            Player::P4 => Rect {
                x: draw_width,
                y: draw_height,
                h: draw_height,
                w: draw_width,
            },
        }
    }

    pub fn get_turret_end(&self) -> (f32, f32) {
        let origin = na::Point2::from(self.position);
        let length = self.turret_width;
        let (sin, cos) = self.turret_rotation.sin_cos();
        let (j, i) = (length * sin, length * cos);

        (origin.x - i, origin.y - j)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use float_eq::float_eq;

    #[test]
    fn initial_info() {
        let tank = tank(Player::P1);
        assert_eq!(tank.position, na::Point2::from([400., 300.]));
        assert_eq!(tank.tank_direction, na::Vector2::from([-1., 0.]));
    }

    #[test]
    fn move_forward() {
        let screen_coord = screen_coord();
        let mut tank = tank(Player::P1);
        let keys = vec![KeyCode::W].into_iter().collect();
        tank.movement(&keys, screen_coord);
        assert_eq!(tank.position, na::Point2::from([399., 300.]));
        let keys = vec![KeyCode::Up].into_iter().collect();
        tank.movement(&keys, screen_coord);
        assert_eq!(tank.position, na::Point2::from([398., 300.]));
    }

    #[test]
    fn move_backwards() {
        let screen_coord = screen_coord();
        let mut tank = tank(Player::P1);
        let keys = vec![KeyCode::S].into_iter().collect();
        tank.movement(&keys, screen_coord);
        assert_eq!(tank.position, na::Point2::from([401., 300.]));
        let keys = vec![KeyCode::Down].into_iter().collect();
        tank.movement(&keys, screen_coord);
        assert_eq!(tank.position, na::Point2::from([402., 300.]));
    }

    #[test]
    fn turn_left() {
        let screen_coord = screen_coord();
        let mut tank = tank(Player::P1);
        let keys = vec![KeyCode::A].into_iter().collect();
        tank.rotation(&keys, 0.3);
        let keys = vec![KeyCode::W].into_iter().collect();
        tank.movement(&keys, screen_coord);
        assert_eq!(tank.position, na::Point2::from([399.04468, 300.29553]));

        let keys = vec![KeyCode::Left].into_iter().collect();
        tank.rotation(&keys, 0.5);
        let keys = vec![KeyCode::W].into_iter().collect();
        tank.movement(&keys, screen_coord);
        assert_eq!(tank.position, na::Point2::from([398.34796, 301.01288]));
    }

    #[test]
    fn turn_right() {
        let screen_coord = screen_coord();
        let mut tank = tank(Player::P1);
        let keys = vec![KeyCode::D].into_iter().collect();
        tank.rotation(&keys, 0.3);
        let keys = vec![KeyCode::S].into_iter().collect();
        tank.movement(&keys, screen_coord);
        assert_eq!(tank.position, na::Point2::from([400.95532, 300.29553]));

        let keys = vec![KeyCode::Right].into_iter().collect();
        tank.rotation(&keys, 0.5);
        let keys = vec![KeyCode::S].into_iter().collect();
        tank.movement(&keys, screen_coord);
        assert_eq!(tank.position, na::Point2::from([401.65204, 301.01288]));
    }

    #[test]
    fn cant_move_backwards_outside_screen() {
        let screen_coord = Rect {
            x: 0.,
            y: 0.,
            w: 400.5,
            h: 300.5,
        };
        let mut tank = tank(Player::P1);
        let keys = vec![KeyCode::S].into_iter().collect();
        tank.movement(&keys, screen_coord);
        assert_eq!(tank.position, na::Point2::from([400., 300.]));
    }

    #[test]
    fn turret_rotation_direction() {
        let mut tank = tank(Player::P1);
        tank.update_turret_direction(na::Point2::from([500., 300.]));
        assert!(float_eq!(tank.turret_rotation, -3.1415925, abs <= 0.000_1));
        assert_eq!(tank.turret_direction, na::Vector2::from([1., 0.]));

        tank.update_turret_direction(na::Point2::from([100., 300.]));
        assert_eq!(tank.turret_rotation, -0.);
        assert_eq!(tank.turret_direction, na::Vector2::from([-1., 0.]));

        tank.update_turret_direction(na::Point2::from([400., 400.]));
        assert_eq!(tank.turret_rotation, -1.5707964);
        assert_eq!(tank.turret_direction, na::Vector2::from([0., 1.]));

        tank.update_turret_direction(na::Point2::from([400., 200.]));
        assert_eq!(tank.turret_rotation, 1.5707964);
        assert_eq!(tank.turret_direction, na::Vector2::from([0., -1.]));

        tank.update_turret_direction(na::Point2::from([540., 410.]));
        assert_eq!(tank.turret_rotation, -2.4756234);
        assert_eq!(
            tank.turret_direction,
            na::Vector2::from([0.78631836, 0.6178216])
        );
    }

    #[test]
    fn turret_drawing_reagion() {
        let tank1 = tank(Player::P1);
        assert_eq!(
            tank1.get_player_turret(),
            Rect {
                x: 0.,
                y: 0.,
                h: 0.5,
                w: 0.5,
            }
        );

        let tank2 = tank(Player::P2);
        assert_eq!(
            tank2.get_player_turret(),
            Rect {
                x: 0.5,
                y: 0.,
                h: 0.5,
                w: 0.5,
            }
        );

        let tank3 = tank(Player::P3);
        assert_eq!(
            tank3.get_player_turret(),
            Rect {
                x: 0.,
                y: 0.5,
                h: 0.5,
                w: 0.5,
            }
        );

        let tank4 = tank(Player::P4);
        assert_eq!(
            tank4.get_player_turret(),
            Rect {
                x: 0.5,
                y: 0.5,
                h: 0.5,
                w: 0.5,
            }
        );
    }

    #[test]
    fn turrets_end() {
        let tank = tank(Player::P1);
        let point = tank.get_turret_end();

        assert_eq!(point, (395., 300.))
    }

    fn tank(player: Player) -> Tank {
        Tank {
            position: na::Point2::from([400., 300.]),
            tank_direction: na::Vector2::from([-1., 0.]),
            tank_rotation: 0.,
            texture: None,
            turret_texture: None,
            turret_direction: na::Vector2::from([-1., 0.]),
            turret_rotation: 0.,
            turret_width: 5.,
            player,
        }
    }

    fn screen_coord() -> Rect {
        Rect {
            x: 0.,
            y: 0.,
            w: 1200.,
            h: 900.,
        }
    }
}
