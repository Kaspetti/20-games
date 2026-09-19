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
    mut next_state: ResMut<NextState<GameState>>,
) {
    let (ball, mut ball_transform, mut ball_movement) = ball_q.into_inner();

    for (col_transform, col, entity) in collider_q {
        let distance = ball_transform.translation - col_transform.translation;
        let clamped_distance = distance.clamp(-col.aabb, col.aabb);
        let closest_point = col_transform.translation + clamped_distance;

        let collision = (ball_transform.translation - closest_point).length() <= ball.radius;

        if collision {
            match col.collider_type {
                ColliderType::Bar => {
                    let offset = ball_transform.translation.x - col_transform.translation.x;
                    let normalized_offset = offset / (col.aabb.x);
                    let bounce_angle = (45.0 * normalized_offset) * (PI / 180.0);

                    // Slide ball out of the bar
                    ball_transform.translation.y =
                        col_transform.translation.y + (col.aabb.y + ball.radius + 0.1);

                    ball_movement.direction =
                        Vec3::new(-ops::sin(-bounce_angle), ops::cos(-bounce_angle), 0.0);
                }

                ColliderType::Wall => {
                    let sign = ball_transform.translation.x.signum();

                    ball_transform.translation.x =
                        col_transform.translation.x - (col.aabb.x + ball.radius + 0.1) * sign;

                    ball_movement.direction.x *= -1.0;
                }

                ColliderType::Ceiling => {
                    ball_transform.translation.y =
                        col_transform.translation.y - (col.aabb.y + ball.radius + 0.1);

                    ball_movement.direction.y *= -1.0;
                }

                ColliderType::Brick => {
                    let tolerance = 0.00001;
                    let mut t_high =
                        (col.aabb.x.powi(2) + col.aabb.y.powi(2)).sqrt() + 2.0 * ball.radius;

                    let mut t_low = 0.0;
                    while t_high - t_low > tolerance {
                        let t_mid = (t_low + t_high) / 2.0;
                        let clamped_distance = ((ball_transform.translation
                            - ball_movement.direction * t_mid)
                            - col_transform.translation)
                            .clamp(-col.aabb, col.aabb);

                        if clamped_distance.length() >= ball.radius {
                            t_high = t_mid;
                        } else {
                            t_low = t_mid;
                        }
                    }

                    ball_transform.translation -= ball_movement.direction * t_high;
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
