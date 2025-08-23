use bevy::prelude::*;
use bevy::{app::{App, Startup}, asset::Assets, color::Color, core_pipeline::core_2d::Camera2d, ecs::{component::Component, system::{Commands, Query, ResMut}}, math::{primitives::Circle, Vec2, Vec3}, render::mesh::{Mesh, Mesh2d}, sprite::{ColorMaterial, MeshMaterial2d}, transform::components::Transform, DefaultPlugins};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.0, 0.0, 0.0))) // background color
        .add_systems(Startup, spawn_particles)
        .insert_resource(Time::<Fixed>::from_hz(60.0)) 
        .add_systems(FixedUpdate, update_particles)
        .run()
        ;
}

struct Entity(u64);

#[derive(Component, Clone, Copy)]
struct Particle{

}


fn spawn_particles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
){
    commands.spawn(Camera2d);


    let particle_mesh = meshes.add(Circle::default());
    let particle_material = materials.add(Color::srgb(255.0, 0.0, 0.0));

    let radius = 75.0;
    let cnt  = 50;
    let particles = (0..50).map(move |i|{
        let angle = i as f32 / cnt as f32 * 360.0;
        let (y, x) = angle.sin_cos();
        let x = x*radius;
        let y = y*radius;
        (
            Particle{
            },
            Mesh2d(particle_mesh.clone()),
            MeshMaterial2d(particle_material.clone()),
            Transform::from_translation(Vec3::new(x, y, 1.0)).with_scale(Vec2::splat(5.0).extend(1.))
        )
    });

    commands.spawn_batch(particles);
}

fn update_particles(mut particles: Query<(&mut Particle, &mut Transform)>){
    for (mut p, mut transform) in &mut particles{
        transform.translation += Vec3::new(0.1, 0.0, 0.0);
    }
}