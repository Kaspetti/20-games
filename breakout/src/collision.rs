use std::f32::consts::PI;

use bevy::prelude::*;

use crate::{ball::Ball, movement::Movement, schedule::GameSet, state::GameState};

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, collision_system.in_set(GameSet::Collision));
    }
}

pub enum ColliderType {
    Bar,
    Wall,
    Ceiling,
    Brick,
}

#[derive(Component)]
pub struct Collider {
    pub collider_type: ColliderType,
    pub aabb: Vec3,
}

fn collision_system(
    ball_q: Single<(&Ball, &mut Transform, &mut Movement)>,
    collider_q: Query<(&Transform, &Collider, Entity), Without<Ball>>,
    mut commands: Commands,
) {
    let (ball, mut ball_transform, mut ball_movement) = ball_q.into_inner();

    for (col_transform, col, entity) in collider_q {
        let closest_point = closest_point_on_box(
            ball_transform.translation,
            col_transform.translation,
            col.aabb,
        );

        let collision = (ball_transform.translation - closest_point).length() <= ball.radius;

        if collision {
            // Slide out of the collider
            let tolerance = 0.001;
            let mut t_high = (col.aabb.x.powi(2) + col.aabb.y.powi(2)).sqrt() + 2.0 * ball.radius;

            let mut t_low = 0.0;
            while t_high - t_low > tolerance {
                let t_mid = (t_low + t_high) / 2.0;

                let new_ball_position =
                    ball_transform.translation - ball_movement.direction * t_mid;
                let closest_point =
                    closest_point_on_box(new_ball_position, col_transform.translation, col.aabb);
                let distance = (new_ball_position - closest_point).length();

                if distance > ball.radius {
                    t_high = t_mid;
                } else {
                    t_low = t_mid;
                }
            }

            ball_transform.translation -= ball_movement.direction * t_high;

            match col.collider_type {
                ColliderType::Bar => {
                    let offset = ball_transform.translation.x - col_transform.translation.x;
                    let normalized_offset = offset / (col.aabb.x);
                    let bounce_angle = (45.0 * normalized_offset) * (PI / 180.0);

                    ball_movement.direction =
                        Vec3::new(-ops::sin(-bounce_angle), ops::cos(-bounce_angle), 0.0);
                }

                ColliderType::Wall => {
                    ball_movement.direction.x *= -1.0;
                }

                ColliderType::Ceiling => {
                    ball_movement.direction.y *= -1.0;
                }

                ColliderType::Brick => {
                    let closest_point = closest_point_on_box(
                        ball_transform.translation,
                        col_transform.translation,
                        col.aabb,
                    );

                    let collision_direction = ball_transform.translation - closest_point;
                    if collision_direction.x.abs() > collision_direction.y.abs() {
                        ball_movement.direction.x *= -1.0;
                    } else {
                        ball_movement.direction.y *= -1.0;
                    }

                    commands.entity(entity).despawn();
                }
            }

            ball_movement.direction = ball_movement.direction.normalize_or_zero();

            // Allow ball to only collide with one object per frame
            break;
        }
    }
}

fn closest_point_on_box(point: Vec3, box_center: Vec3, aabb: Vec3) -> Vec3 {
    box_center + (point - box_center).clamp(-aabb, aabb)
}
