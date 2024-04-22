use std::sync::Mutex;
use std::thread;

use lazy_static::lazy_static;
use rand::Rng;
use robotics_lib::energy::Energy;
use robotics_lib::event::events::Event;
use robotics_lib::interface::{Direction, go};
use robotics_lib::runner::{Robot, Runnable, Runner};
use robotics_lib::runner::backpack::BackPack;
use robotics_lib::utils::LibError;
use robotics_lib::world::coordinates::Coordinate;
use robotics_lib::world::World;
use robotics_lib::world::world_generator::Generator;
use crate::world_generator::get_generator;

lazy_static! {
    static ref KEEP_RUNNING :Mutex<bool> = Mutex::new(true);
}

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

pub(crate) fn get_runner(generator: &mut impl Generator) -> Runner {
    Runner::new(Box::new(Roomba { robot: Robot::new() }), generator).unwrap()
}

pub(crate) fn start(){
    let mut runner = get_runner(&mut get_generator(8,0));
    let mut keep_running = *KEEP_RUNNING.try_lock().unwrap();
    
    while keep_running {
        keep_running = *KEEP_RUNNING.try_lock().unwrap();
        match runner.game_tick() {
            Ok(_) => {}
            Err(e) => { println!("{:?}", e); }
        }
        thread::sleep(std::time::Duration::from_millis(500));

    }
}