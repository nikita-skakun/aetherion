use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

#[derive(Component)]
pub struct Size(pub Vec3);

#[derive(Component)]
pub struct ModuleTag;

#[derive(Component)]
pub struct ModuleEngineTag {
    pub thrust: f32,
}

pub fn spawn_base_module(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    size: Vec3,
    position: Vec3,
    rotation: Quat,
    color: Color,
    velocity: Velocity,
    is_headless: bool,
) -> Entity {
    if is_headless {
        commands
            .spawn((
                ModuleTag,
                Size(size),
                SpatialBundle {
                    transform: Transform::from_translation(position)
                        * Transform::from_rotation(rotation),
                    ..Default::default()
                },
                Collider::cuboid(size.x / 2.0, size.y / 2.0, size.z / 2.0),
                RigidBody::Dynamic,
                velocity,
            ))
            .id()
    } else {
        commands
            .spawn((
                ModuleTag,
                Size(size),
                PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Box {
                        min_x: -size.x / 2.0,
                        min_y: -size.y / 2.0,
                        min_z: -size.z / 2.0,
                        max_x: size.x / 2.0,
                        max_y: size.y / 2.0,
                        max_z: size.z / 2.0,
                    })),
                    material: materials.add(color.into()),
                    transform: Transform::from_translation(position),
                    ..Default::default()
                },
                Collider::cuboid(size.x / 2.0, size.y / 2.0, size.z / 2.0),
                RigidBody::Dynamic,
                velocity,
            ))
            .id()
    }
}

pub fn engine_system(
    mut query: Query<(&mut Velocity, &mut ModuleEngineTag, &mut Transform)>,
    time: Res<Time>,
) {
    for (mut velocity, engine, transform) in query.iter_mut() {
        velocity.linvel += transform.rotation * Vec3::Y * engine.thrust * time.delta_seconds();
    }
}
