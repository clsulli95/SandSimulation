pub mod paintbrush;
pub mod physics;
pub mod world;

use crate::world::{PixelType, Point, World};
use comfy::*;
use num_traits::ToPrimitive;

const SCALE_FACTOR: f32 = 1.0;
const WORLD_OFFSET: f32 = SCALE_FACTOR / 2.0;

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
    if is_mouse_button_down(MouseButton::Left) || is_mouse_button_pressed(MouseButton::Left) {
        let screen_pos = mouse_world();
        let point = Point::new(screen_pos.x as usize, screen_pos.y as usize);
        paintbrush::paint_circle(&mut state.world, &point, 1.0, PixelType::Solid);
    }

    if is_mouse_button_down(MouseButton::Right) || is_mouse_button_pressed(MouseButton::Right) {
        let screen_pos = mouse_world();
        let point = Point::new(screen_pos.x as usize, screen_pos.y as usize);
        paintbrush::paint_circle(&mut state.world, &point, 3.0, PixelType::Sand);
    }

    if is_key_down(KeyCode::W) || is_key_pressed(KeyCode::W) {
        let screen_pos = mouse_world();
        let point = Point::new(screen_pos.x as usize, screen_pos.y as usize);
        paintbrush::paint_circle(&mut state.world, &point, 3.0, PixelType::Water);
    }

    physics::update_world(&mut state.world).unwrap();

    state.world.render();
}

struct GameState {
    pub world: World,
}

impl GameState {
    pub fn new(_c: &EngineState) -> Self {
        Self {
            world: World::new(250),
        }
    }
}

trait Render {
    fn render(&self);
}

impl Render for World {
    fn render(&self) {
        for x in 0..self.size {
            for y in 0..self.size {
                let pt = self.get_pixel_type(Point::new(x, y));

                let color = match pt {
                    PixelType::Solid => WHITE,
                    PixelType::Sand => YELLOW,
                    PixelType::Water => BLUE,
                    PixelType::Air => TRANSPARENT,
                    PixelType::OutOfBounds => RED,
                };

                draw_rect(
                    vec2(
                        x.to_f32().unwrap() + WORLD_OFFSET,
                        y.to_f32().unwrap() + WORLD_OFFSET,
                    ),
                    vec2(SCALE_FACTOR, SCALE_FACTOR),
                    color,
                    0,
                );
            }
        }
    }
}
