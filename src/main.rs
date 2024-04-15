use anyhow::{Context, Result};
use comfy::*;
use num_traits::ToPrimitive;

simple_game!("My Game Demo", GameState, config, setup, update);

fn config(config: GameConfig) -> GameConfig {
    GameConfig {
        vsync_enabled: false,
        target_framerate: 60,
        ..config
    }
}

fn setup(_state: &mut GameState, _c: &mut EngineContext) {
    main_camera_mut().zoom = 250.0;
    main_camera_mut().center = vec2(0.0, 0.0);
    let dim = main_camera().world_viewport();
    let half_x = dim.x / 2.0;
    let half_y = dim.y / 2.0;
    let center_x = main_camera().world_viewport().x - half_x;
    let center_y = main_camera().world_viewport().y - half_y;
    main_camera_mut().center = vec2(center_x, center_y);
    println!("{}, {}", half_x, half_y);
    println!("{}", main_camera_mut().world_viewport());

    game_config_mut().dev.show_fps = true;
}

fn update(state: &mut GameState, _c: &mut EngineContext) {
    let delta = delta();
    draw_light(Light::default());

    if is_mouse_button_down(MouseButton::Left) || is_mouse_button_pressed(MouseButton::Left) {
        let mouse_pos = mouse_world();
        let pixel_pos = vec2(mouse_pos.x.ceil(), mouse_pos.y.ceil());
        state.world.world[pixel_pos.x as usize][pixel_pos.y as usize].p_type = PixelType::Sand;
    }

    for row in state.world.world.iter_mut() {
        for pixel in row.iter_mut() {
            if pixel.p_type == PixelType::Sand {
                pixel._vel = vec2(0.0, -1.0);
                pixel.pos.x = pixel.pos.x + pixel._vel.x;
                pixel.pos.y = pixel.pos.y + pixel._vel.y;
            }
        }
    }

    for row in state.world.world.iter_mut() {
        for pixel in row.iter_mut() {
            let color = match pixel.p_type {
                PixelType::Solid => RED,
                PixelType::Sand => BEIGE,
                PixelType::Water => BLUE,
                PixelType::Air => TRANSPARENT,
            };

            draw_rect(pixel.pos, vec2(1.0, 1.0), color, 0);
        }
    }
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

#[derive(Debug, PartialEq, Eq)]
enum PixelType {
    Solid,
    Sand,
    Water,
    Air,
}

#[derive(Debug)]
struct Pixel {
    pub pos: Vec2,
    pub p_type: PixelType,
    pub _vel: Vec2,
}

impl Pixel {
    pub fn new(pos: Vec2, _vel: Vec2) -> Self {
        Self {
            pos,
            p_type: PixelType::Air,
            _vel,
        }
    }
}

#[derive(Debug)]
struct World {
    pub world: Vec<Vec<Pixel>>,
}

impl World {
    pub fn new(size: usize) -> Result<Self> {
        let mut world: Vec<Vec<Pixel>> = Vec::with_capacity(size);

        for row in 0..size {
            let mut row_vec = Vec::with_capacity(size);
            for col in 0..size {
                let row_f32 = row.to_f32().context("usize to f32 conversion failed")?;
                let col_f32 = col.to_f32().context("usize to f32 conversion failed")?;
                row_vec.push(Pixel::new(vec2(row_f32, col_f32), Vec2::ZERO));
            }
            world.push(row_vec);
        }

        Ok(Self { world })
    }
}
