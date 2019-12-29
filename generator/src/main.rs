use astropixels::{components, create_dispatcher, resources};
use cgmath::Vector2;
use rand::{thread_rng, Rng};
use specs::{Builder, WorldExt};
use std::io::prelude::*;

const NUM_ENTITIES: usize = 1_000;
const NUM_ITERATIONS: usize = 30 * 30;

fn main() {
    let mut dispatcher = create_dispatcher(NUM_ENTITIES, NUM_ITERATIONS);

    let mut rng = thread_rng();
    let distance = 100.0;
    let speed = 2.0;
    let galaxies = vec![
        // ((distance / 2.0, distance / 2.0), (0.0, 0.0)),
        ((distance / 2.0, 0.0), (speed, 0.0)),
        ((distance, distance / 2.0), (0.0, speed)),
        ((distance / 2.0, distance), (-speed, 0.0)),
        ((0.0, distance / 2.0), (0.0, -speed)),
    ];

    for ((gx, gy), (gvx, gvy)) in &galaxies {
        for _ in 0..(NUM_ENTITIES / galaxies.len()) {
            let x = rng.gen_range(-25.0, 25.0);
            let y = rng.gen_range(-25.0, 25.0);
            let vx = rng.gen_range(-0.001, 0.001);
            let vy = rng.gen_range(-0.001, 0.001);

            dispatcher
                .world_mut()
                .create_entity()
                .with(components::Position {
                    data: Vector2::new(gx + x, gy + y),
                })
                .with(components::Velocity {
                    data: Vector2::new(gvx + vx, gvy + vy),
                })
                .build();
        }
    }

    // let mut rng = thread_rng();
    // for _ in 0..NUM_GALAXIES {
    //     let gx = rng.gen_range(-250.0, 250.0);
    //     let gy = rng.gen_range(-250.0, 250.0);
    //     let gvx = rng.gen_range(-0.2, 0.2);
    //     let gvy = rng.gen_range(-0.2, 0.2);

    //     for _ in 0..(NUM_ENTITIES / NUM_GALAXIES) {
    //         let x = rng.gen_range(-25.0, 25.0);
    //         let y = rng.gen_range(-25.0, 25.0);
    //         let vx = rng.gen_range(-0.001, 0.001);
    //         let vy = rng.gen_range(-0.001, 0.001);

    //         dispatcher
    //             .world_mut()
    //             .create_entity()
    //             .with(components::Position {
    //                 data: Vector2::new(gx + x, gy + y),
    //             })
    //             .with(components::Velocity {
    //                 data: Vector2::new(gvx + vx, gvy + vy),
    //             })
    //             .build();
    //     }
    // }

    for _ in 0..NUM_ITERATIONS {
        dispatcher.dispatch();
    }

    dispatcher
        .world_mut()
        .get_mut::<resources::Log>()
        .unwrap()
        .file
        .flush()
        .unwrap();
}
