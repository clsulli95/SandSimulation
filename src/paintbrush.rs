use crate::world::PixelType;
use crate::world::Point;
use crate::world::World;

pub fn paint_circle(world: &mut World, point: &Point, radius: f32, pixel_type: PixelType) {
    for i in 0..world.size {
        for j in 0..world.size {
            let x_f32 = point.x as f32;
            let y_f32 = point.y as f32;
            let i_f32 = i as f32;
            let j_f32 = j as f32;

            let distance = ((x_f32 - i_f32).powf(2.0) + (y_f32 - j_f32).powf(2.0)).sqrt();

            if distance < radius {
                world.set_pixel_type(Point::new(i, j), pixel_type).unwrap();
            }
        }
    }
}
