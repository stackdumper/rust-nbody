use crate::components;
use cgmath::MetricSpace;
use cgmath::Vector2;
use specs::{Entities, ReadStorage, System, WriteStorage};

const G: f64 = 6.67e-11f64;

pub struct Gravity;

impl<'a> System<'a> for Gravity {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, components::Position>,
        WriteStorage<'a, components::Velocity>,
    );

    fn run(&mut self, (ent, pos, mut vel): Self::SystemData) {
        use rayon::iter::ParallelIterator;
        use specs::{Join, ParJoin};

        (&ent, &pos, &mut vel)
            .par_join()
            .for_each(|(ent_a, pos_a, vel_a)| {
                (&ent, &pos).join().for_each(|(ent_b, pos_b)| {
                    if ent_a.id() == ent_b.id() {
                        return;
                    }

                    let distance = pos_a.data.distance2(pos_b.data);
                    if distance < 1.0 {
                        return;
                    }

                    let direction = pos_a.data - pos_b.data;
                    let amount = G / distance;
                    let impulse = direction * amount;

                    vel_a.data -= impulse * 100_000_000.0;
                });
            })
    }
}
