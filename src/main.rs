use bevy::math::NormedVectorSpace;
use bevy::prelude::*;
use bevy::{app::{App, Startup}, asset::Assets, color::Color, core_pipeline::core_2d::Camera2d, ecs::{component::Component, system::{Commands, Query, ResMut}}, math::{primitives::Circle, Vec2, Vec3}, render::mesh::{Mesh, Mesh2d}, sprite::{ColorMaterial, MeshMaterial2d}, transform::components::Transform, DefaultPlugins};
use rand::Rng;

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
    center: Vec3,
    heading: Vec3,
    avoidance_dir: Vec3,
}


fn rand_vec() -> Vec3{
    let angle:f32 = rand::rng().random_range(0.0..360.0);
    let s = angle.sin_cos();
    Vec3::new(s.0, s.1, 0.0)
}

fn spawn_particles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
){
    commands.spawn(Camera2d);


    let particle_mesh = meshes.add(Circle::default());
    let particle_mesh2 = meshes.add(Circle::default());
    let particle_material = materials.add(Color::srgb(255.0, 0.0, 0.0));
    let particle_material2 = materials.add(Color::srgb(0.0, 255.0, 0.0));

    let radius = 75.0;
    let cnt  = 360;
    let particles = (0..36).map(move |i|{
        let angle = i as f32 / cnt as f32 * 360.0;
        let (y, x) = angle.sin_cos();
        let x = x*radius -100.0;
        let y = y*radius;
        (
            Particle{},
            Velocity(rand_vec()),
            ParticleComputationData{
                center: Vec3::default(),
                heading: Vec3::default(),
                avoidance_dir: Vec3::default(),
            },
            Mesh2d(particle_mesh.clone()),
            MeshMaterial2d(particle_material.clone()),
            Transform::from_translation(Vec3::new(x, y, 1.0)).with_scale(Vec2::splat(5.0).extend(1.))
        )
    });

    let particles = particles.chain((0..36).map(move |i|{
        let angle = i as f32 / cnt as f32 * 360.0;
        let (y, x) = angle.sin_cos();
        let x = x*radius + 100.0;
        let y = y*radius;
        (
            Particle{},
            Velocity(rand_vec()),
            ParticleComputationData{
                center: Vec3::default(),
                heading: Vec3::default(),
                avoidance_dir: Vec3::default(),
            },
            Mesh2d(particle_mesh2.clone()),
            MeshMaterial2d(particle_material2.clone()),
            Transform::from_translation(Vec3::new(x, y, 1.0)).with_scale(Vec2::splat(5.0).extend(1.))
        )
    }));

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
    particles: Query<(&mut ParticleComputationData, &Transform)>,
    other_particles: Query<(&Particle, &Velocity, &Transform)>
){
    for (mut data, transform) in particles{

        let mut count = 0;
        let mut proximity_count = 0;
        let mut pos = Vec3::default();
        let mut heading = Vec3::default();

        let mut avoidance_dir = Vec3::default();
        let mut avoidance_count = 0;
        for (p, velocity, trans) in other_particles {
            let distance = transform.translation.distance(trans.translation);

            if distance > 75.0{
                continue;
            }
            pos += trans.translation;
            count += 1;

            heading += velocity.0.normalize_or_zero();
            proximity_count += 1;

            if distance < 20.0{
                avoidance_dir += (transform.translation - trans.translation).normalize_or_zero();
                avoidance_count += 1;
            }
        }

        data.center = (pos * (1.0 / count as f32));
        data.heading = (heading * (proximity_count as f32).recip()).normalize_or_zero();
        data.avoidance_dir = avoidance_dir * (avoidance_count as f32).recip();
    }
}

fn update_particles(
    particles: Query<(&mut Particle, &mut Velocity, &Transform, &ParticleComputationData)>
){
    let mut avoidance_counter = 0;
    for (mut p, mut vel, transform, data) in particles{

        let fixed_center_cohesion = -transform.translation.normalize_or_zero();

        let cohesion = (data.center - transform.translation).normalize_or_zero();

        let avoidance = data.avoidance_dir.normalize_or_zero();
        if avoidance.length_squared() == 0.0{
            avoidance_counter += 1;
        }
        let direction = (0.3* cohesion + 1.0*data.heading + 2.0* avoidance + 0.7 * fixed_center_cohesion);

        let direction = direction.normalize();

        let acceleration: f32 = 5.0;
        let max_speed:f32 = 100.0;
        vel.0 += direction * acceleration;

        vel.0 = vel.0.clamp_length_max(max_speed);
    }
}