use crate::world::{Adjacency, PixelType, Point, World};
use anyhow::Result;

pub fn update_world(world: &mut World) -> Result<()> {
    for x in 0..world.size {
        for y in 0..world.size {
            match world.get_pixel_type(Point::new(x, y)) {
                PixelType::Solid => handle_solid(world, Point::new(x, y))?,
                PixelType::Sand => handle_sand(world, Point::new(x, y))?,
                PixelType::Water => handle_water(world, Point::new(x, y))?,
                PixelType::Air => handle_air(world, Point::new(x, y))?,
                PixelType::OutOfBounds => handle_out_of_bounds(world, Point::new(x, y))?,
            }
        }
    }

    Ok(())
}

fn handle_solid(_world: &mut World, _point: Point) -> Result<()> {
    Ok(())
}

fn handle_sand(world: &mut World, point: Point) -> Result<()> {
    // Rule #1 - If nothing below, move down
    if world.get_pixel_type_adj(point, Adjacency::Below) == PixelType::Air {
        world.swap_pixel_type(&point, Adjacency::Below)?;
    // Rule #2 - If solid below, attempt left
    } else if world.get_pixel_type_adj(point, Adjacency::BelowLeft) == PixelType::Air {
        world.swap_pixel_type(&point, Adjacency::BelowLeft)?;
    // Rule #3- Attempt right
    } else if world.get_pixel_type_adj(point, Adjacency::BelowRight) == PixelType::Air {
        world.swap_pixel_type(&point, Adjacency::BelowRight)?;
    }

    Ok(())
}

fn handle_water(world: &mut World, point: Point) -> Result<()> {
    // Rule #1 - If nothing below, move down
    if world.get_pixel_type_adj(point, Adjacency::Below) == PixelType::Air {
        world.swap_pixel_type(&point, Adjacency::Below)?;
    // Rule #2 - If solid below, attempt left
    } else if world.get_pixel_type_adj(point, Adjacency::BelowLeft) == PixelType::Air {
        world.swap_pixel_type(&point, Adjacency::BelowLeft)?;
    // Rule #3- Attempt right
    } else if world.get_pixel_type_adj(point, Adjacency::BelowRight) == PixelType::Air {
        world.swap_pixel_type(&point, Adjacency::BelowRight)?;
    } else if world.get_pixel_type_adj(point, Adjacency::Left) == PixelType::Air {
        world.swap_pixel_type(&point, Adjacency::Left)?;
    } else if world.get_pixel_type_adj(point, Adjacency::Right) == PixelType::Air {
        world.swap_pixel_type(&point, Adjacency::Right)?;
    }

    Ok(())
}

fn handle_air(_world: &mut World, _point: Point) -> Result<()> {
    Ok(())
}

fn handle_out_of_bounds(_world: &mut World, _point: Point) -> Result<()> {
    Ok(())
}
