use crate::components;
use shred::{ResourceId, World};
use specs::{Join, ReadStorage, System, SystemData, WriteStorage};

pub struct Velocity;

#[derive(SystemData)]
pub struct VelocityData<'a> {
    velocity: ReadStorage<'a, components::Velocity>,
    position: WriteStorage<'a, components::Position>,
}

// TODO: par_chunks_mut
impl<'a> System<'a> for Velocity {
    type SystemData = VelocityData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        for (position, velocity) in (&mut data.position, &data.velocity).join() {
            position.data += velocity.data;
        }
    }
}
