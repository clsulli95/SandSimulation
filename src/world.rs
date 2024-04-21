use anyhow::{anyhow, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PixelType {
    Solid,
    Sand,
    Water,
    Air,
    Border,
    OutOfBounds,
}

#[derive(Debug, Clone, Copy)]
pub struct Pixel {
    pixel_type: PixelType,
}

impl Pixel {
    pub fn new(pixel_type: PixelType) -> Self {
        Self { pixel_type }
    }
}

impl Default for Pixel {
    fn default() -> Self {
        Self {
            pixel_type: PixelType::Solid,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Adjacency {
    AboveLeft,
    Above,
    AboveRight,
    Left,
    Right,
    BelowLeft,
    Below,
    BelowRight,
}

pub struct World {
    world: Vec<Vec<Pixel>>,
    size: usize,
}

impl World {
    pub fn new(size: usize) -> Self {
        Self {
            world: vec![vec![Pixel::default(); size]; size],
            size,
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, pixel_type: PixelType) -> Result<()> {
        let real_x = self.size - 1 - y;
        let real_y = x;
        self.world
            .get_mut(real_x)
            .and_then(|y_vec| y_vec.get_mut(real_y))
            .map(|pixel| pixel.pixel_type = pixel_type)
            .ok_or(anyhow!("Out of Bounds Access"))
        //self.world[real_x][real_y].pixel_type = pixel_type;
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> PixelType {
        let real_x = self.size - 1 - y;
        let real_y = x;
        self.world
            .get(real_x)
            .and_then(|y_vec| y_vec.get(real_y))
            .map_or(PixelType::OutOfBounds, |pixel| pixel.pixel_type)
    }

    pub fn get_pixel_adj(&self, x: usize, y: usize, adj: Adjacency) -> PixelType {
        let mut lookup_x = x;
        let mut lookup_y = y;
        match adj {
            Adjacency::AboveLeft => {
                lookup_x -= 1;
                lookup_y += 1;
            }
            Adjacency::Above => {
                lookup_y += 1;
            }
            Adjacency::AboveRight => {
                lookup_x += 1;
                lookup_y += 1;
            }
            Adjacency::Left => {
                lookup_x -= 1;
            }
            Adjacency::Right => {
                lookup_x += 1;
            }
            Adjacency::BelowLeft => {
                lookup_x -= 1;
                lookup_y -= 1;
            }
            Adjacency::Below => {
                lookup_y -= 1;
            }

            Adjacency::BelowRight => {
                lookup_x += 1;
                lookup_y -= 1;
            }
        }

        let real_x = self.size - 1 - lookup_y;
        let real_y = lookup_x;

        self.world
            .get(real_x)
            .and_then(|y_vec| y_vec.get(real_y))
            .map_or(PixelType::OutOfBounds, |pixel| pixel.pixel_type)
    }
}

impl std::fmt::Debug for World {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (xidx, x) in self.world.iter().enumerate() {
            for (yidx, y) in x.iter().enumerate() {
                match y.pixel_type {
                    PixelType::Solid => write!(f, "({},{} : S) ", xidx, yidx)?,
                    PixelType::Sand => write!(f, "({},{} : s) ", xidx, yidx)?,
                    PixelType::Water => write!(f, "({},{} : W) ", xidx, yidx)?,
                    PixelType::Air => write!(f, "({},{} : A) ", xidx, yidx)?,
                    PixelType::Border => write!(f, "({},{} : B) ", xidx, yidx)?,
                    PixelType::OutOfBounds => write!(f, "({},{} : O) ", xidx, yidx)?,
                }
            }

            writeln!(f)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_pixel_type() {
        let pixel = Pixel::new(PixelType::Solid);
        assert_eq!(PixelType::Solid, pixel.pixel_type);
        let pixel = Pixel::new(PixelType::Sand);
        assert_eq!(PixelType::Sand, pixel.pixel_type);
        let pixel = Pixel::new(PixelType::Water);
        assert_eq!(PixelType::Water, pixel.pixel_type);
        let pixel = Pixel::new(PixelType::Air);
        assert_eq!(PixelType::Air, pixel.pixel_type);
        let pixel = Pixel::new(PixelType::Border);
        assert_eq!(PixelType::Border, pixel.pixel_type);
        let pixel = Pixel::new(PixelType::OutOfBounds);
        assert_eq!(PixelType::OutOfBounds, pixel.pixel_type);
    }

    #[test]
    fn test_get_pixel() {
        let mut world = World::new(4);
        world.world[0][0].pixel_type = PixelType::Water;
        world.world[1][1].pixel_type = PixelType::Water;
        world.world[2][2].pixel_type = PixelType::Water;
        world.world[3][3].pixel_type = PixelType::Water;
        assert_eq!(PixelType::Water, world.get_pixel(0, 3));
        assert_eq!(PixelType::Water, world.get_pixel(1, 2));
        assert_eq!(PixelType::Water, world.get_pixel(2, 1));
        assert_eq!(PixelType::Water, world.get_pixel(3, 0));
    }

    #[test]
    fn test_set_pixel() {
        let mut world = World::new(4);
        world.set_pixel(0, 3, PixelType::Water).unwrap();
        world.set_pixel(1, 2, PixelType::Water).unwrap();
        world.set_pixel(2, 1, PixelType::Water).unwrap();
        world.set_pixel(3, 0, PixelType::Water).unwrap();
        assert_eq!(PixelType::Water, world.world[0][0].pixel_type);
        assert_eq!(PixelType::Water, world.world[1][1].pixel_type);
        assert_eq!(PixelType::Water, world.world[2][2].pixel_type);
        assert_eq!(PixelType::Water, world.world[3][3].pixel_type);
    }

    #[test]
    fn test_get_pixel_adj_above_left() {
        let mut world = World::new(4);
        world.set_pixel(1, 3, PixelType::Sand).unwrap();
        world.set_pixel(2, 2, PixelType::Water).unwrap();

        assert_eq!(
            PixelType::Sand,
            world.get_pixel_adj(2, 2, Adjacency::AboveLeft)
        );
    }

    #[test]
    fn test_get_pixel_adj_above() {
        let mut world = World::new(4);
        world.set_pixel(2, 3, PixelType::Sand).unwrap();
        world.set_pixel(2, 2, PixelType::Water).unwrap();

        assert_eq!(PixelType::Sand, world.get_pixel_adj(2, 2, Adjacency::Above));
    }

    #[test]
    fn test_get_pixel_adj_left() {
        let mut world = World::new(4);
        world.set_pixel(1, 2, PixelType::Sand).unwrap();
        world.set_pixel(2, 2, PixelType::Water).unwrap();

        assert_eq!(PixelType::Sand, world.get_pixel_adj(2, 2, Adjacency::Left));
    }

    #[test]
    fn test_get_pixel_adj_right() {
        let mut world = World::new(4);
        world.set_pixel(3, 2, PixelType::Sand).unwrap();
        world.set_pixel(2, 2, PixelType::Water).unwrap();

        assert_eq!(PixelType::Sand, world.get_pixel_adj(2, 2, Adjacency::Right));
    }

    #[test]
    fn test_get_pixel_adj_above_right() {
        let mut world = World::new(4);
        world.set_pixel(3, 3, PixelType::Sand).unwrap();
        world.set_pixel(2, 2, PixelType::Water).unwrap();

        assert_eq!(
            PixelType::Sand,
            world.get_pixel_adj(2, 2, Adjacency::AboveRight)
        );
    }

    #[test]
    fn test_get_pixel_adj_below_left() {
        let mut world = World::new(4);
        world.set_pixel(1, 1, PixelType::Sand).unwrap();
        world.set_pixel(2, 2, PixelType::Water).unwrap();

        assert_eq!(
            PixelType::Sand,
            world.get_pixel_adj(2, 2, Adjacency::BelowLeft)
        );
    }

    #[test]
    fn test_get_pixel_adj_below() {
        let mut world = World::new(4);
        world.set_pixel(2, 1, PixelType::Sand).unwrap();
        world.set_pixel(2, 2, PixelType::Water).unwrap();

        assert_eq!(PixelType::Sand, world.get_pixel_adj(2, 2, Adjacency::Below));
    }

    #[test]
    fn test_get_pixel_adj_below_right() {
        let mut world = World::new(4);
        world.set_pixel(3, 1, PixelType::Sand).unwrap();
        world.set_pixel(2, 2, PixelType::Water).unwrap();

        assert_eq!(
            PixelType::Sand,
            world.get_pixel_adj(2, 2, Adjacency::BelowRight)
        );
    }
}
