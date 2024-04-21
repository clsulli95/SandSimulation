pub mod world;

use crate::world::{PixelType, World};
use anyhow::{Context, Result};
use comfy::*;
use num_traits::ToPrimitive;
use std::io::{stdout, Write};

simple_game!("My Game Demo", GameState, config, setup, update);

fn config(config: GameConfig) -> GameConfig {
    GameConfig {
        vsync_enabled: false,
        target_framerate: 60,
        ..config
    }
}

fn setup(state: &mut GameState, _c: &mut EngineContext) {
    main_camera_mut().zoom = 250.0; //state.world.size as f32;
    main_camera_mut().center = vec2(0.0, 0.0);
    let dim = main_camera().world_viewport();
    let half_x = dim.x / 2.0;
    let half_y = dim.y / 2.0;
    let center_x = main_camera().world_viewport().x - half_x;
    let center_y = main_camera().world_viewport().y - half_y;
    main_camera_mut().center = vec2(center_x, center_y);
    game_config_mut().dev.show_fps = true;
}

fn update(state: &mut GameState, _c: &mut EngineContext) {
    let delta = delta();
    draw_light(Light::default());

    if is_mouse_button_down(MouseButton::Left) || is_mouse_button_pressed(MouseButton::Left) {
        let mouse_pos = mouse_world() + 0.5;
        let pixel_pos = vec2(mouse_pos.x.floor(), mouse_pos.y.floor());
        println!("{pixel_pos:?}");
        state.world.set_pixel(
            pixel_pos.x as usize,
            pixel_pos.y as usize,
            PixelType::Sand,
            CoordSource::Screen,
        );
    }

    if is_mouse_button_pressed(MouseButton::Right) {
        println!("{:?}", state.world);
    }

    state.world.render();
}

struct GameState {
    pub world: World,
}

impl GameState {
    pub fn new(_c: &EngineState) -> Self {
        Self {
            world: World::new(250).unwrap(),
        }
    }
}

trait Render {
    fn render(&self);
}

//#[derive(Debug, Clone, Copy, PartialEq, Eq)]
//enum PixelDirection {
//    Down,
//    Up,
//    Left,
//    Right,
//}
//
//#[derive(Debug, Clone, Copy, PartialEq, Eq)]
//enum CoordSource {
//    Screen,
//    Code,
//}

impl Render for World {
    fn render(&self) {
        for x in 0..self.size {
            for y in 0..self.size {
                let pixel_type = self.get_pixel(x, y);

                let color = match pixel_type {
                    PixelType::Solid => RED,
                    PixelType::Sand => BEIGE,
                    PixelType::Water => BLUE,
                    PixelType::Air => TRANSPARENT,
                    PixelType::Border => WHITE,
                    PixelType::OutOfBounds => BLUE,
                };

                draw_rect(pixel.pos, vec2(1.0, 1.0), color, 0);
            }
        }

            }
        }
    }
}

//#[cfg(test)]
//mod tests {
//    use super::*;
//
//    #[test]
//    fn test_get_pixel_down() {
//        let mut world = crate::World::new(50).unwrap();
//
//        world.set_pixel(0, 0, PixelType::Sand, CoordSource::Screen);
//        world.set_pixel(0, 1, PixelType::Sand, CoordSource::Screen);
//        println!("{world:?}");
//
//        assert_eq!(
//            PixelType::Sand,
//            world.get_pixel_type(0, 0, &PixelDirection::Down)
//        );
//    }

//#[test]
//fn test_get_pixel_right() {
//    let mut world = crate::World::new(50).unwrap();

//    world.set_pixel(0, 0, PixelType::Sand, CoordSource::Code);
//    world.set_pixel(0, 1, PixelType::Sand, CoordSource::Code);

//    assert_eq!(
//        PixelType::Sand,
//        world.get_pixel_type(world.get_pixel(0, 0).unwrap(), &PixelDirection::Right)
//    );
//}
//}
