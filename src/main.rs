use libm::{cos, sin};
use vecmath::{vec3_add, vec3_normalized, vec3_scale, Vector3};

enum Turn {
    Left,
    Right,
}

const TURN_SPEED_RPS: f64 = 1.0;
const VELOCITY_RPS: f64 = 0.1;

fn velocity(position: &Vector3<f64>, direction: &Vector3<f64>) -> Vector3<f64> {
    vecmath::vec3_cross(*position, *direction)
}

fn update(position: &mut Vector3<f64>, direction: &mut Vector3<f64>, turn: Option<Turn>, dt: f64) {
    // turning before moving, increases responsiveness
    let current_velocity = velocity(position, direction);

    *direction = match turn {
        None => *direction,
        Some(Turn::Left) => {
            let new_direction = vec3_scale(*direction, cos(dt * TURN_SPEED_RPS));
            let new_velocity = vec3_scale(current_velocity, sin(dt * TURN_SPEED_RPS));
            vec3_normalized(vec3_add(new_direction, new_velocity))
        }
        Some(Turn::Right) => {
            let new_direction = vec3_scale(*direction, cos(dt * TURN_SPEED_RPS));
            let new_velocity = vec3_scale(current_velocity, -sin(dt * TURN_SPEED_RPS));
            vec3_normalized(vec3_add(new_direction, new_velocity))
        }
    };

    let new_velocity = velocity(position, direction);

    *position = vec3_normalized(vec3_add(
        *position,
        vec3_scale(new_velocity, dt * VELOCITY_RPS),
    ));
}

fn main() {
    let position = &mut [1.0, 0.0, 0.0];
    // normal vector of the plane containing the great circle
    let direction = &mut [0.0, 1.0, 0.0];
    for _ in 1..600 {
        update(position, direction, None, 1.0 / 60.0);
        println!("{}, {}, {}", position[0], position[1], position[2]);
    }
    for _ in 1..600 {
        update(position, direction, Some(Turn::Right), 1.0 / 60.0);
        println!("{}, {}, {}", position[0], position[1], position[2]);
    }
}