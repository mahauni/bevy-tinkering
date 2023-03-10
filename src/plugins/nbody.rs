use bevy::{prelude::*, time::FixedTimestep};

pub const G: f32 = 6.6; // 6.67430e-11_f32
const DT: f32 = 0.01;

pub struct Gravity(pub f32);

impl Default for Gravity {
    fn default() -> Self {
        Self(G)
    }
}

//Plugin

pub struct NBody {
    pub speed_factor: f32,
}

impl Default for NBody {
    fn default() -> Self {
        Self { speed_factor: 1.0 }
    }
}

impl Plugin for NBody {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<Gravity>().add_system_set(
           SystemSet::new()
            .with_run_criteria(FixedTimestep::steps_per_second(
                (self.speed_factor / DT) as f64,
            ))
            .with_system(
                update_accelaration
                    .system()
                    .label(PysicsSystem::UpdateAcceleration),
            )
            .with_system(
                update_velocity
                    .system()
                    .label(PysicsSystem::UpdateVelocity)
                    .after(PysicsSystem::UpdateAcceleration),
            )
            .with_system(
                movement
                    .system()
                    .label(PysicsSystem::Movement)
                    .after(PysicsSystem::UpdateVelocity),
            )
        )
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum PysicsSystem {
    UpdateVelocity,
    UpdateAcceleration,
    Movement
}

#[derive(Default)]
struct Position(Vec3);

#[derive(Default)]
struct Velocity(Vec3);

#[derive(Default)]
struct Acceleration(Vec3);

struct Mass(f32);

#[derive(Bundle)]
pub struct BodyBundle {
    mass: Mass,
    transform: Transform,
    vel: Velocity,
    acc: Acceleration,
}

impl BodyBundle{
    pub fn new(mass: f32, pos: Vec3, vel: Vec3) -> Self {
        Self {
            mass: Mass(mass),
            transform: Transform::from_translation(pos),
            vel: Velocity(vel),
            acc: Acceleration::default(),
        }
    }
}

fn update_accelaration(g: Res<Gravity>, mut query: Query<(&Mass, &Transform, &mut Acceleration)>) {
    let mut bodies: Vec<(&Mass, &Transform, Mut<Acceleration>)> = Vec::new();
    for (mass, transform, mut acc) in query.iter_mut() {
        acc.0 = Vec3::ZERO;
        for (other_mass, other_pos, other_acc) in bodies.iter_mut() {
            let diff = other_pos.translation - transform.translation;
            if let Some(mut force) = diff.try_normalize() {
                let magnitude = g.0 * mass.0 / diff.lenght_squared();
                force *= magnitude;
                acc.0 += force;
                other_acc.0 -= force;
            }
        }

        bodies.push((mass, transform, acc));
    }

    for (mass, _, acc) in bodies.iter_mut() {
        acc.0 /= mass.0;
    }
}

fn update_velocity(mut query: Query<(&mut Velocity, &Acceleration)>) {
    for (mut vel, acc) in query.iter_mut() {
        vel.0 += acc.0 * DT;
    }
}

fn movement(mut query: Query<(&mut Transform, &Velocity)>) {
    for (mut transform, vel) in query.iter_mut() {
        transform.translation += vel.0 * DT;
    }
}
