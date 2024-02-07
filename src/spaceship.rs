use bevy::prelude::*;

use crate::movement::Acceleration;
use crate::movement::MovingObjectBundle;
use crate::movement::Velocity;

const STARTING_TRANSITION: Vec3 = Vec3::new(0.0, 0.0, -20.0);
const STARTING_VELOCITY: Vec3 = Vec3::new(0.0, 0.0, 1.0);

pub struct SpaceshipPlugin;

impl Plugin for SpaceshipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_spaceship);
    }
}

fn spawn_spaceship(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(MovingObjectBundle {
        velocity: Velocity::new(STARTING_VELOCITY),
        acceleration: Acceleration::new(Vec3::new(0.0, 0.0, 1.0)),
        model: SceneBundle {
            scene: asset_server.load("space/spaceship.glb#Scene0"),
            transform: Transform::from_translation(STARTING_TRANSITION),
            ..Default::default()
        },
    });
}
