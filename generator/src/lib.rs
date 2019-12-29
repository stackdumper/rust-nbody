use specs::{AsyncDispatcher, DispatcherBuilder, World, WorldExt};

pub mod components;
pub mod resources;
pub mod systems;

pub fn create_dispatcher(
    num_entities: usize,
    num_iterations: usize,
) -> AsyncDispatcher<'static, World> {
    // create world
    let mut world = World::new();

    // register components
    world.register::<components::Position>();
    world.register::<components::Velocity>();

    // add resources
    world.insert(resources::Log::new(num_entities, num_iterations));

    // create dispatcher with systems
    let dispatcher = DispatcherBuilder::new()
        .with(systems::Velocity, "velocity", &[])
        .with(systems::Gravity, "gravity", &["velocity"])
        .with(systems::Log, "log", &["velocity"])
        .build_async(world);

    return dispatcher;
}
