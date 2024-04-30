use crate::world::PixelType;
use crate::world::Point;
use crate::world::World;
use rand::Rng;

pub fn paint_circle(
    world: &mut World,
    point: &Point,
    radius: f32,
    pixel_type: PixelType,
    draw_percentage: f32,
) {
    for i in 0..world.size {
        for j in 0..world.size {
            let x_f32 = point.x as f32;
            let y_f32 = point.y as f32;
            let i_f32 = i as f32;
            let j_f32 = j as f32;
            let threshold = draw_percentage / 100.0;

            let distance = ((x_f32 - i_f32).powf(2.0) + (y_f32 - j_f32).powf(2.0)).sqrt();

            if distance < radius && is_over_draw_threshold(threshold) {
                world.set_pixel_type(Point::new(i, j), pixel_type).unwrap();
            }
        }
    }
}

fn is_over_draw_threshold(draw_percentage: f32) -> bool {
    let threshold = 1.0 - draw_percentage;
    let mut rng = rand::thread_rng();
    let random_val: f32 = rng.gen();
    random_val > threshold
}
