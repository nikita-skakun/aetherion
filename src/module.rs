use bevy::prelude::*;

#[derive(Component)]
pub struct Size(pub Vec3);

#[derive(Component)]
pub struct ModuleTag;

pub fn spawn_module(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    size: Vec3,
    position: Vec3,
    color: Color,
    is_headless: bool,
) {
    if is_headless {
        commands.spawn((
            ModuleTag,
            Size(size),
            SpatialBundle {
                transform: Transform::from_translation(position),
                ..Default::default()
            },
        ));
    } else {
        commands.spawn((
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
        ));
    }
}
