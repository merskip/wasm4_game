use core::f32::consts::{FRAC_PI_2, PI};
use libm::{ceilf, fabsf, floorf, sqrtf, tanf};
use crate::ray_casting::Direction::{Horizontal, Vertical};
use crate::wasm4::geometry::Point;
use crate::world_map::WorldMap;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Direction {
    Horizontal,
    Vertical
}

pub fn ray_cast(origin: &Point<f32>, world_map: &WorldMap, angle: f32) -> (f32, Direction) {
    // Get both the closest horizontal and vertical wall
    // intersections for this angle.
    let horizontal_distance = horizontal_intersection(origin, world_map, angle);
    let vertical_distance = vertical_intersection(origin, world_map, angle);

    return if horizontal_distance < vertical_distance {
        (horizontal_distance, Horizontal)
    } else {
        (vertical_distance, Vertical)
    }
}

/// Returns the nearest wall the ray intersects with on a horizontal grid line.
fn horizontal_intersection(origin: &Point<f32>, world_map: &WorldMap, angle: f32) -> f32 {
    // This tells you if the angle is "facing up"
    // regardless of how big the angle is.
    let up = fabsf(floorf(angle / PI) % 2.0) != 0.0;

    // first_y and first_x are the first grid intersections
    // that the ray intersects with.
    let first_y = if up {
        ceilf(origin.y) - origin.y
    } else {
        floorf(origin.y) - origin.y
    };
    let first_x = -first_y / tanf(angle);

    // dy and dx are the "ray extension" values mentioned earlier.
    let dy = if up { 1.0 } else { -1.0 };
    let dx = -dy / tanf(angle);

    // next_x and next_y are mutable values which will keep track
    // of how far away the ray is from the player.
    let mut next_x = first_x;
    let mut next_y = first_y;

    // This is the loop where the ray is extended until it hits
    // the wall. It's not an infinite loop as implied in the
    // explanation, instead it only goes from 0 to 256.
    //
    // This was chosen because if something goes wrong and the
    // ray never hits a wall (which should never happen) the
    // loop will eventually break and the game will keep on running.
    for _ in 0..256 {
        // current_x and current_y are where the ray is currently
        // on the map, while next_x and next_y are relative
        // coordinates, current_x and current_y are absolute
        // points.
        let current_x = next_x + origin.x;
        let current_y = if up {
            next_y + origin.y
        } else {
            next_y + origin.y - 1.0
        };

        // Tell the loop to quit if we've just hit a wall.
        if world_map.is_wall(current_x, current_y) {
            break;
        }

        // if we didn't hit a wall on this extension add
        // dx and dy to our current position and keep going.
        next_x += dx;
        next_y += dy;
    }

    // return the distance from next_x and next_y to the origin
    distance(next_x, next_y)
}

/// Returns the nearest wall the ray intersects with on a vertical grid line.
fn vertical_intersection(origin: &Point<f32>, world_map: &WorldMap, angle: f32) -> f32 {
    // This tells you if the angle is "facing up"
    // regardless of how big the angle is.
    let right = fabsf(floorf((angle - FRAC_PI_2) / PI) % 2.0) != 0.0;

    // first_y and first_x are the first grid intersections
    // that the ray intersects with.
    let first_x = if right {
        ceilf(origin.x) - origin.x
    } else {
        floorf(origin.x) - origin.x
    };
    let first_y = -tanf(angle) * first_x;

    // dy and dx are the "ray extension" values mentioned earlier.
    let dx = if right { 1.0 } else { -1.0 };
    let dy = dx * -tanf(angle);

    // next_x and next_y are mutable values which will keep track
    // of how far away the ray is from the player.
    let mut next_x = first_x;
    let mut next_y = first_y;

    // This is the loop where the ray is extended until it hits
    // the wall. It's not an infinite loop as implied in the
    // explanation, instead it only goes from 0 to 256.
    //
    // This was chosen because if something goes wrong and the
    // ray never hits a wall (which should never happen) the
    // loop will eventually quit and the game will keep on running.
    for _ in 0..256 {
        // current_x and current_y are where the ray is currently
        // on the map, while next_x and next_y are relative
        // coordinates, current_x and current_y are absolute
        // points.
        let current_x = if right {
            next_x + origin.x
        } else {
            next_x + origin.x - 1.0
        };
        let current_y = next_y + origin.y;

        // Tell the loop to quit if we've just hit a wall.
        if world_map.is_wall(current_x, current_y) {
            break;
        }

        // if we didn't hit a wall on this extension add
        // dx and dy to our current position and keep going.
        next_x += dx;
        next_y += dy;
    }

    // return the distance from next_x and next_y to the player.
    distance(next_x, next_y)
}

fn distance(a: f32, b: f32) -> f32 {
    sqrtf((a * a) + (b * b))
}