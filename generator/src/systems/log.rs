use crate::{components, resources};
use shred::{ResourceId, World};
use specs::{Join, ReadStorage, System, SystemData, Write};

pub struct Log;

#[derive(SystemData)]
pub struct LogData<'a> {
    log: Write<'a, resources::Log>,
    velocity: ReadStorage<'a, components::Velocity>,
    position: ReadStorage<'a, components::Position>,
}

// TODO: par_chunks_mut
impl<'a> System<'a> for Log {
    type SystemData = LogData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        for (position, velocity) in (&data.position, &data.velocity).join() {
            data.log.push(position.data, velocity.data);
        }

        data.log.next();
    }
}
