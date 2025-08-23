use bevy::prelude::*;
use bevy::{app::{App, Startup}, asset::Assets, color::Color, core_pipeline::core_2d::Camera2d, ecs::{component::Component, system::{Commands, Query, ResMut}}, math::{primitives::Circle, Vec2, Vec3}, render::mesh::{Mesh, Mesh2d}, sprite::{ColorMaterial, MeshMaterial2d}, transform::components::Transform, DefaultPlugins};

const TICK_RATE: f32 = 60.0;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.0, 0.0, 0.0))) // background color
        .add_systems(Startup, spawn_particles)
        .insert_resource(Time::<Fixed>::from_hz(TICK_RATE as f64)) 
        .add_systems(FixedUpdate, (update_particle_data, update_particles, apply_velocity).chain())
        .run()
        ;
}

#[derive(Component, Clone, Copy)]
struct Particle{

}

#[derive(Component)]
struct Velocity(Vec3);

#[derive(Component, Clone, Copy)]
struct ParticleComputationData{
    center: Vec3
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
            Particle{},
            Velocity(Vec3::ZERO),
            ParticleComputationData{
                center: Vec3::default()
            },
            Mesh2d(particle_mesh.clone()),
            MeshMaterial2d(particle_material.clone()),
            Transform::from_translation(Vec3::new(x, y, 1.0)).with_scale(Vec2::splat(5.0).extend(1.))
        )
    });

    commands.spawn_batch(particles);
}

fn apply_velocity(
    objs: Query<(&Velocity, &mut Transform)>
){
    for (velocity, mut transform) in objs{
        transform.translation += velocity.0 * TICK_RATE.recip();
    }
}


fn update_particle_data(
    particles: Query<&mut ParticleComputationData>,
    other_particles: Query<(&Particle, &Transform)>
){
    for mut data in particles{

        let mut count = 0;
        let mut pos = Vec3::default();
        for (p, transform) in other_particles {
            pos += transform.translation;

            count += 1;
        }

        data.center = pos * (1.0 / count as f32);
    }
}

fn update_particles(
    particles: Query<(&mut Particle, &mut Velocity, &Transform, &ParticleComputationData)>
){
    for (mut p, mut vel, transform, data) in particles{
        let direction = (data.center - transform.translation).normalize();

        let acceleration: f32 = 0.5;
        vel.0 += direction * acceleration;
    }
}