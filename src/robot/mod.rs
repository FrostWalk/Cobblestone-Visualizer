use rand::Rng;
use robotics_lib::energy::Energy;
use robotics_lib::event::events::Event;
use robotics_lib::interface::{go, Direction};
use robotics_lib::runner::backpack::BackPack;
use robotics_lib::runner::{Robot, Runnable};
use robotics_lib::utils::LibError;
use robotics_lib::world::coordinates::Coordinate;
use robotics_lib::world::World;

const AVAILABLE_DIRECTIONS: [Direction; 4] = [
    Direction::Up,
    Direction::Down,
    Direction::Left,
    Direction::Right,
];

pub struct Roomba {
    pub(crate) robot: Robot,
}

impl Runnable for Roomba {
    fn process_tick(&mut self, world: &mut World) {
        let index: usize = rand::thread_rng().gen_range(0..4);

        let dir = AVAILABLE_DIRECTIONS[index].clone();
        println!("going {:?}", dir);

        match go(self, world, dir) {
            Ok(_) => {}
            Err(e) => {
                if e == LibError::NotEnoughEnergy {
                    return;
                }

                println!("{:?}", e);
                self.process_tick(world)
            }
        }
    }

    fn handle_event(&mut self, event: Event) {
        //println!("\nevent: {event}")
    }
    fn get_energy(&self) -> &Energy {
        &self.robot.energy
    }
    fn get_energy_mut(&mut self) -> &mut Energy {
        &mut self.robot.energy
    }
    fn get_coordinate(&self) -> &Coordinate {
        &self.robot.coordinate
    }
    fn get_coordinate_mut(&mut self) -> &mut Coordinate {
        &mut self.robot.coordinate
    }
    fn get_backpack(&self) -> &BackPack {
        &self.robot.backpack
    }
    fn get_backpack_mut(&mut self) -> &mut BackPack {
        &mut self.robot.backpack
    }
}
