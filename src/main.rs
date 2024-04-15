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
    println!("{}", main_camera_mut().world_viewport());
    game_config_mut().dev.show_fps = true;
}

fn update(state: &mut GameState, _c: &mut EngineContext) {
    draw_light(Light::default());

    if is_mouse_button_pressed(MouseButton::Left) {
        println!("{}", mouse_world());
    }

    for row in state.world.world.iter_mut() {
        for pixel in row.iter_mut() {
            draw_rect(pixel.pos, vec2(1.0, 1.0), RED, 0);
        }
    }
}

struct GameState {
    pub world: World,
}

impl GameState {
    pub fn new(_c: &EngineState) -> Self {
        Self {
            world: World::new(1).unwrap(),
        }
    }
}

#[derive(Debug)]
enum PixelType {
    Solid,
    Sand,
    Water,
    Air,
}

#[derive(Debug)]
struct Pixel {
    pub pos: Vec2,
    pub _p_type: PixelType,
    pub _vel: Vec2,
}

impl Pixel {
    pub fn new(pos: Vec2, _vel: Vec2) -> Self {
        Self {
            pos,
            _p_type: PixelType::Air,
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
