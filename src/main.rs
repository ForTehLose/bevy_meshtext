//! A simple 3D scene with light shining over a cube sitting on a plane.

use std::string;

use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::diagnostic::LogDiagnosticsPlugin;
use bevy::pbr::NotShadowCaster;
use bevy::pbr::NotShadowReceiver;
use bevy::prelude::*;
use bevy::render::mesh::Indices;

use bevy::render::render_resource::PrimitiveTopology;
use meshtext::QualitySettings;
use meshtext::{IndexedMeshText, MeshGenerator, TextSection};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(LogDiagnosticsPlugin::default())
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_systems(Startup, setup)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(5.0).into()),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });

    // cube
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(-0.5, 0.5, 0.0),
        ..default()
    });

    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 1.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
    let text: String = "Test Text".to_owned();
    //text?
    let mesh_handle = meshes.add(create_mesh(&text));

    commands.spawn((
        PbrBundle {
            mesh: mesh_handle,
            material: materials.add(StandardMaterial {
                base_color: Color::BLUE,
                unlit: true,
                ..Default::default()
            }),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        },
        NotShadowCaster,
        NotShadowReceiver,
    ));
}


fn create_mesh(text: &String) -> Mesh {
    let mut cube_mesh = Mesh::new(PrimitiveTopology::TriangleList);

    let mesh_data = get_text_vertices(&text);
    let chunks = mesh_data
        .vertices
        .chunks(3)
        .map(|c| <[_; 3]>::try_from(c).unwrap())
        .collect::<Vec<_>>();

    #[rustfmt::skip]
    cube_mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        chunks,
    );

    #[rustfmt::skip]
    cube_mesh.set_indices(Some(Indices::U32(mesh_data.indices)));
    cube_mesh
}

fn get_text_vertices(text: &String) -> IndexedMeshText {
    let font_data = include_bytes!("assets/FiraSans-Bold.ttf");
    let mut generator = MeshGenerator::new_without_cache(font_data, QualitySettings::default());

    let thing = generator
        .generate_section(&text, true, None)
        .expect("Failed to generate glyph.");
    return thing;
}
