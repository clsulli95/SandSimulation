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
    main_camera_mut().zoom = 1000.0; //state.world.size as f32;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PixelType {
    Solid,
    Sand,
    Water,
    Air,
    Border,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CoordSource {
    Screen,
    Code,
}

#[derive(Debug, Clone, Copy)]
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

struct World {
    pub world: Vec<Vec<Pixel>>,
    pub size: usize,
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

        Ok(Self { world, size })
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> Pixel {
        self.world[self.size - 1 - x][y]
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, t: PixelType, c: CoordSource) {
        match c {
            CoordSource::Screen => self.world[x][y].p_type = t,
            CoordSource::Code => self.world[self.size - 1 - x][y].p_type = t,
        }
    }

    pub fn get_pixel_type_below(&self, pixel: &Pixel) -> PixelType {
        //        self.get_pixel(pixel.pos.x, pixel.pos.y - 1);

        //let x = pixel.pos.y + 1 >= self.size
        PixelType::Solid
    }
}

impl std::fmt::Debug for World {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for x in 0..self.size {
            for y in 0..self.size {
                match self.get_pixel(x, y).p_type {
                    PixelType::Solid => write!(f, "S ")?,
                    PixelType::Sand => write!(f, "s ")?,
                    PixelType::Water => write!(f, "W ")?,
                    PixelType::Air => write!(f, "A ")?,
                    PixelType::Border => write!(f, "B ")?,
                }
            }

            writeln!(f)?;
        }

        Ok(())
    }
}

impl Render for World {
    fn render(&self) {
        for row in self.world.iter() {
            for pixel in row.iter() {
                let color = match pixel.p_type {
                    PixelType::Solid => RED,
                    PixelType::Sand => BEIGE,
                    PixelType::Water => BLUE,
                    PixelType::Air => TRANSPARENT,
                    PixelType::Border => WHITE,
                };

                draw_rect(pixel.pos, vec2(1.0, 1.0), color, 0);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_pixel() {
        let world = crate::World::new(50).unwrap();
        assert_eq!(
            PixelType::Solid,
            world.get_pixel_type_below(&world.get_pixel(0, 0))
        );
    }
}
