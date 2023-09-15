//! A simple 3D scene with light shining over a cube sitting on a plane.

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
        .add_systems(Update, sping_wheels)
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
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    //text?
    let mesh_handle = meshes.add(create_mesh());

    commands.spawn((
        PbrBundle {
            mesh: mesh_handle,
            material: materials.add(StandardMaterial {
                base_color: Color::BLUE,
                emissive: Color::BLUE,
                unlit: false,
                ..Default::default()
            }),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        },
        NotShadowCaster,
        NotShadowReceiver,
    ));
}

fn sping_wheels() {

    create_mesh();
}

fn create_mesh() -> Mesh {
    let mut cube_mesh = Mesh::new(PrimitiveTopology::TriangleList);

    let mesh_data = get_text_vertices();
    let chunks = mesh_data
        .vertices
        .chunks(3)
        .map(|c| <[_; 3]>::try_from(c).unwrap())
        .collect::<Vec<_>>();

    #[rustfmt::skip]
    cube_mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        // Each array is an [x, y, z] coordinate in local space.
        // Meshes always rotate around their local [0, 0, 0] when a rotation is applied to their Transform.
        // By centering our mesh around the origin, rotating the mesh preserves its center of mass.
        chunks,
    );

    // Create the triangles out of the 24 vertices we created.
    // To construct a square, we need 2 triangles, therefore 12 triangles in total.
    // To construct a triangle, we need the indices of its 3 defined vertices, adding them one
    // by one, in a counter-clockwise order (relative to the position of the viewer, the order
    // should appear counter-clockwise from the front of the triangle, in this case from outside the cube).
    // Read more about how to correctly build a mesh manually in the Bevy documentation of a Mesh,
    // further examples and the implementation of the built-in shapes.
    #[rustfmt::skip]
    cube_mesh.set_indices(Some(Indices::U32(mesh_data.indices)));

    cube_mesh
}

fn get_text_vertices() -> IndexedMeshText {
    let font_data = include_bytes!("assets/FiraSans-Bold.ttf");
    let mut generator = MeshGenerator::new_without_cache(font_data, QualitySettings::default());

    let thing = generator
        .generate_section("Test Text", true, None)
        .expect("Failed to generate glyph.");

    generator.clear_cache();

    return thing;
}
