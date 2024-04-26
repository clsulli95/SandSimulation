use anyhow::{anyhow, bail, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CoordinateFrame {
    Raw,
    Translated,
}

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PixelType {
    Solid,
    Sand,
    Water,
    Air,
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
            pixel_type: PixelType::Air,
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
    pub size: usize,
}

impl World {
    pub fn new(size: usize) -> Self {
        Self {
            world: vec![vec![Pixel::default(); size]; size],
            size,
        }
    }

    pub fn swap_pixel_type(&mut self, source_point: &Point, adj: Adjacency) -> Result<()> {
        let target_point = self.determine_adj_coords(*source_point, adj);
        let source_type = self.get_pixel_type(*source_point);
        let target_type = self.get_pixel_type(target_point);

        if source_type == PixelType::OutOfBounds || target_type == PixelType::OutOfBounds {
            bail!("Out of bounds access when trying to swap")
        }

        self.set_pixel_type(*source_point, target_type)?;
        self.set_pixel_type(target_point, source_type)?;

        Ok(())
    }

    pub fn set_pixel_type(&mut self, point: Point, pixel_type: PixelType) -> Result<()> {
        let translated_point = self.translate_to_bottom_left_coords(point);

        self.world
            .get_mut(translated_point.x)
            .and_then(|y_vec| y_vec.get_mut(translated_point.y))
            .map(|pixel| pixel.pixel_type = pixel_type)
            .ok_or(anyhow!("Out of Bounds Access"))
    }

    pub fn set_pixel_type_adj(
        &mut self,
        point: Point,
        adj: Adjacency,
        pixel_type: PixelType,
    ) -> Result<()> {
        let adj_point = self.determine_adj_coords(point, adj);
        let translated_point = self.translate_to_bottom_left_coords(adj_point);

        self.world
            .get_mut(translated_point.x)
            .and_then(|y_vec| y_vec.get_mut(translated_point.y))
            .map(|pixel| pixel.pixel_type = pixel_type)
            .ok_or(anyhow!("Out of Bounds Access"))
    }

    pub fn get_pixel_type(&self, point: Point) -> PixelType {
        let translated_point = self.translate_to_bottom_left_coords(point);

        self.world
            .get(translated_point.x)
            .and_then(|y_vec| y_vec.get(translated_point.y))
            .map_or(PixelType::OutOfBounds, |pixel| pixel.pixel_type)
    }

    pub fn get_pixel_type_adj(&self, point: Point, adj: Adjacency) -> PixelType {
        let adj_point = self.determine_adj_coords(point, adj);
        let translated_point = self.translate_to_bottom_left_coords(adj_point);

        self.world
            .get(translated_point.x)
            .and_then(|y_vec| y_vec.get(translated_point.y))
            .map_or(PixelType::OutOfBounds, |pixel| pixel.pixel_type)
    }

    fn determine_adj_coords(&self, point: Point, adj: Adjacency) -> Point {
        let mut lookup_x = point.x;
        let mut lookup_y = point.y;

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

        Point::new(lookup_x, lookup_y)
    }

    fn translate_to_bottom_left_coords(&self, point: Point) -> Point {
        Point::new(self.size - 1 - point.y, point.x)
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
        assert_eq!(PixelType::Water, world.get_pixel_type(Point::new(0, 3)));
        assert_eq!(PixelType::Water, world.get_pixel_type(Point::new(1, 2)));
        assert_eq!(PixelType::Water, world.get_pixel_type(Point::new(2, 1)));
        assert_eq!(PixelType::Water, world.get_pixel_type(Point::new(3, 0)));
    }

    #[test]
    fn test_set_pixel() {
        let mut world = World::new(4);
        world
            .set_pixel_type(Point::new(0, 3), PixelType::Water)
            .unwrap();
        world
            .set_pixel_type(Point::new(1, 2), PixelType::Water)
            .unwrap();
        world
            .set_pixel_type(Point::new(2, 1), PixelType::Water)
            .unwrap();
        world
            .set_pixel_type(Point::new(3, 0), PixelType::Water)
            .unwrap();
        assert_eq!(PixelType::Water, world.world[0][0].pixel_type);
        assert_eq!(PixelType::Water, world.world[1][1].pixel_type);
        assert_eq!(PixelType::Water, world.world[2][2].pixel_type);
        assert_eq!(PixelType::Water, world.world[3][3].pixel_type);
    }

    #[test]
    fn test_get_pixel_adj_above_left() {
        let mut world = World::new(4);
        world
            .set_pixel_type(Point::new(1, 3), PixelType::Sand)
            .unwrap();
        world
            .set_pixel_type(Point::new(2, 2), PixelType::Water)
            .unwrap();

        assert_eq!(
            PixelType::Sand,
            world.get_pixel_type_adj(Point::new(2, 2), Adjacency::AboveLeft)
        );
    }

    #[test]
    fn test_get_pixel_adj_above() {
        let mut world = World::new(4);
        world
            .set_pixel_type(Point::new(2, 3), PixelType::Sand)
            .unwrap();
        world
            .set_pixel_type(Point::new(2, 2), PixelType::Water)
            .unwrap();

        assert_eq!(
            PixelType::Sand,
            world.get_pixel_type_adj(Point::new(2, 2), Adjacency::Above)
        );
    }

    #[test]
    fn test_get_pixel_adj_left() {
        let mut world = World::new(4);
        world
            .set_pixel_type(Point::new(1, 2), PixelType::Sand)
            .unwrap();
        world
            .set_pixel_type(Point::new(2, 2), PixelType::Water)
            .unwrap();

        assert_eq!(
            PixelType::Sand,
            world.get_pixel_type_adj(Point::new(2, 2), Adjacency::Left)
        );
    }

    #[test]
    fn test_get_pixel_adj_right() {
        let mut world = World::new(4);
        world
            .set_pixel_type(Point::new(3, 2), PixelType::Sand)
            .unwrap();
        world
            .set_pixel_type(Point::new(2, 2), PixelType::Water)
            .unwrap();

        assert_eq!(
            PixelType::Sand,
            world.get_pixel_type_adj(Point::new(2, 2), Adjacency::Right)
        );
    }

    #[test]
    fn test_get_pixel_adj_above_right() {
        let mut world = World::new(4);
        world
            .set_pixel_type(Point::new(3, 3), PixelType::Sand)
            .unwrap();
        world
            .set_pixel_type(Point::new(2, 2), PixelType::Water)
            .unwrap();

        assert_eq!(
            PixelType::Sand,
            world.get_pixel_type_adj(Point::new(2, 2), Adjacency::AboveRight)
        );
    }

    #[test]
    fn test_get_pixel_type_adj_below_left() {
        let mut world = World::new(4);
        world
            .set_pixel_type(Point::new(1, 1), PixelType::Sand)
            .unwrap();
        world
            .set_pixel_type(Point::new(2, 2), PixelType::Water)
            .unwrap();

        assert_eq!(
            PixelType::Sand,
            world.get_pixel_type_adj(Point::new(2, 2), Adjacency::BelowLeft)
        );
    }

    #[test]
    fn test_get_pixel_type_adj_below() {
        let mut world = World::new(4);
        world
            .set_pixel_type(Point::new(2, 1), PixelType::Sand)
            .unwrap();
        world
            .set_pixel_type(Point::new(2, 2), PixelType::Water)
            .unwrap();

        assert_eq!(
            PixelType::Sand,
            world.get_pixel_type_adj(Point::new(2, 2), Adjacency::Below)
        );
    }

    #[test]
    fn test_get_pixel_type_adj_below_right() {
        let mut world = World::new(4);
        world
            .set_pixel_type(Point::new(3, 1), PixelType::Sand)
            .unwrap();
        world
            .set_pixel_type(Point::new(2, 2), PixelType::Water)
            .unwrap();

        assert_eq!(
            PixelType::Sand,
            world.get_pixel_type_adj(Point::new(2, 2), Adjacency::BelowRight)
        );
    }

    #[test]
    fn test_set_pixel_type_adj_below_right() {
        let mut world = World::new(4);
        world
            .set_pixel_type_adj(Point::new(2, 2), Adjacency::BelowRight, PixelType::Sand)
            .unwrap();
        world
            .set_pixel_type(Point::new(2, 2), PixelType::Water)
            .unwrap();

        assert_eq!(
            PixelType::Sand,
            world.get_pixel_type_adj(Point::new(2, 2), Adjacency::BelowRight)
        );
    }

    #[test]
    fn test_swap_pixel_type() {
        let mut world = World::new(4);
        world
            .set_pixel_type(Point::new(2, 2), PixelType::Sand)
            .unwrap();
        world
            .set_pixel_type(Point::new(2, 1), PixelType::Water)
            .unwrap();
        world
            .swap_pixel_type(&Point::new(2, 2), Adjacency::Below)
            .unwrap();

        assert_eq!(PixelType::Water, world.get_pixel_type(Point::new(2, 2)));
        assert_eq!(PixelType::Sand, world.get_pixel_type(Point::new(2, 1)));
    }
}
