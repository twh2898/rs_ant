
mod util;
mod world;

use util::console::clear_console;
use world::World;

fn main() {
    // clear_console();
    let mut world = World::new();
    world.generate();

    for _ in 0..200 {
        world.step();
        clear_console();
        println!("{}", world);
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
}
