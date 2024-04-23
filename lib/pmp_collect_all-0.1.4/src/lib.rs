use std::collections::HashMap;
use std::collections::HashSet;

use robotics_lib::interface;
use robotics_lib::interface::Direction;
use robotics_lib::interface::Direction::{Down, Left, Right, Up};
use robotics_lib::interface::Tools;
use robotics_lib::runner::Runnable;
use robotics_lib::world::tile::{Content, Tile};
use robotics_lib::world::World;

/*
                     ,     ,
                    (\____/)
                     (_oo_)
                       (O)
                     __||__    \)
                  []/______\[] /
                  / \______/ \/
                 /    /__\
                (\   /____\
                _________  _________
                | ___ \  \/  || ___ \
                | |_/ / .  . || |_/ /
                |  __/| |\/| ||  __/
                | |   | |  | || |
                \_|   \_|  |_/\_|
            - Pattern-Matching Pioneers -
     Helping robots live in square, two-dimensional worlds
                    Est. 2023
====================================================================================================
                AUTHORIZED COPY OF
                 Collect All
                      v0.1.4
====================================================================================================
             SOFTWARE LICENSE AGREEMENT
             By using our software, the customer (you) agrees to:
             - never copy and distribute the source code to unauthorized third parties
             - thank Angelo at least once a month for making such good pizzas
             - send us pictures if you make any cool streets!
====================================================================================================
             THANK YOU FOR USING OUR SOFTWARE!
             Be sure to read the documentation.
             If you need any help, do not hesitate to
             contact our Customer Service reps
             for this tool on Telegram:
                 - @poss03251
                 - @Sim02R
====================================================================================================
             CUSTOMER FEEDBACK CHANNEL
             https://t.me/patternmatchingpioneers
====================================================================================================
*/

/// ToolCollectAll struct / Tool
/// This tool is used for finding and collecting everything that is available in the
/// current range of the robot (unless there are obstacles), until you reach a certain energy threshold
/// or the required content has been collected.
/// It has also the ability to locate the content currently available in his sight.
/// If no requirement is specified in the catching phase, it tries to get all the available resources.
///
/// # Import
/// ```
/// use collect_all::CollectAll;
/// ```
///
/// # Example
/// ```rust
/// let mut requirements = HashMap::new();
/// requirements.insert(Content::Coin(0), 1); // Insert all your requirements in here
///
/// // To retrieve the coordinates of all specified resources and their quantities
/// let range = 5;
/// let positions = CollectAll::detect_items(self, world, range, requirements);
///
/// // To retrieve the coordinates of all resources and their quantities
/// let positions = CollectAll::detect_all(self, world, range);
///
/// // To collect all the specified resources that are in the reachable range until all requirements are met
/// let _ = CollectAll::collect_items(self, world, range, requirements);
///
/// // To collect all the resources that are in the reachable range until all requirements are met
/// let _ = CollectAll::collect_all(self, world, range);
/// ```
///
type TileInfo = HashMap<Content, HashSet<(usize, (usize, usize))>>;

pub struct CollectAll;
impl Tools for CollectAll {}

impl CollectAll {
    // Collect all specified resources in the area
    #[allow(dead_code)]
    pub fn collect_items(
        robot: &mut impl Runnable,
        world: &mut World,
        range: usize,
        requirements: HashMap<Content, usize>, // If set to zero it will be interpreted as inifite
    ) {
        // Retrieve all available specified resources
        let resources = Self::detect_items(robot, world, range, requirements);

        // Collect all available specified resources
        Self::collect_resources(robot, world, &resources);
    }

    // Detect any specified content in the range of the robot
    #[allow(dead_code)]
    pub fn detect_items(
        robot: &mut impl Runnable,
        world: &mut World,
        range: usize,
        requirements: HashMap<Content, usize>, // If set to zero it will be interpreted as inifite
    ) -> TileInfo {
        // Retrieve all available resources
        let mut resources = Self::detect_all(robot, world, range);
        let mut unecessary_content: Vec<Content> = vec![];

        // Filter resources
        for (content, tiles) in &mut resources {
            // Remove non required content
            if let Some(requirement) = requirements.get(&content.to_default()) {
                let required_quantity = *requirement as i32;

                // Remove unecessary resources if required quantity is exceeded
                if required_quantity != 0 {
                    let mut unecessary_tiles: Vec<(usize, (usize, usize))> = vec![];

                    // Give priority to tile with more content
                    let mut sorted_tiles: Vec<_> = tiles.iter().collect();
                    sorted_tiles.sort_by(|first, second| second.cmp(&first));

                    let mut sum = 0;
                    for tile in sorted_tiles {
                        if sum >= required_quantity {
                            unecessary_tiles.push(*tile);
                            continue;
                        }

                        sum += tile.0 as i32;
                    }

                    for tile in unecessary_tiles {
                        tiles.remove(&tile);
                    }
                }
            } else {
                unecessary_content.push(content.clone());
            }
        }

        for content in unecessary_content {
            resources.remove(&content);
        }

        resources
    }

    // Collect all resources in the area
    #[allow(dead_code)]
    pub fn collect_all(robot: &mut impl Runnable, world: &mut World, range: usize) {
        // Retrieve all available resources
        let resources = Self::detect_all(robot, world, range);

        // Collect all available resources
        Self::collect_resources(robot, world, &resources);
    }

    // Detect any content in the range of the robot
    pub fn detect_all(robot: &mut impl Runnable, world: &mut World, range: usize) -> TileInfo {
        let mut result: TileInfo = HashMap::new();
        let (robot_x, robot_y) = Self::get_coordinates(robot);

        // Scan the world in every direction
        let directions = vec![Up, Down, Left, Right];
        for direction in directions.into_iter() {
            if let Ok(view) = interface::one_direction_view(robot, world, direction.clone(), range)
            {
                if view.len() > 0 {
                    Self::scan_tiles(&view, &mut result, direction.clone(), (robot_x, robot_y));
                }
            }
        }

        result
    }

    // Collect all the provided resources
    fn collect_resources(robot: &mut impl Runnable, world: &mut World, resources: &TileInfo) {
        for (_, tiles) in resources.iter() {
            // Sort in descending order to give priority to tiles with more content
            let mut sorted_tiles: Vec<_> = tiles.iter().collect();
            sorted_tiles.sort_by(|first, second| second.cmp(&first));

            // Retrieve content by destroying the tile
            for tile in sorted_tiles {
                let (_, tile_coords) = tile;
                Self::destroy_tile(robot, world, *tile_coords);
            }
        }
    }

    // Scan the tiles and store the content coordinates and quantity
    fn scan_tiles(
        input: &Vec<Vec<Tile>>,
        result: &mut TileInfo,
        direction: Direction,
        mut coords: (usize, usize),
    ) {
        let max_x = input.len() as i32 - 1;
        let max_y = input[0].len() as i32 - 1;

        // Compute the relative coordinates
        coords = match coords {
            (0, 0) => match direction {
                Up => (coords.0, coords.1),
                Down => (coords.0 + 1, coords.1),
                Left => (coords.0, coords.1),
                Right => (coords.0, coords.1 + 1),
            },
            (0, _) => match direction {
                Up => (coords.0, coords.1 - 1),
                Down => (coords.0 + 1, coords.1 - 1),
                Left => (coords.0, coords.1 - 1),
                Right => (coords.0, coords.1 + 1),
            },
            (_, 0) => match direction {
                Up => (coords.0 - 1, coords.1),
                Down => (coords.0 + 1, coords.1),
                Left => (coords.0 - 1, coords.1),
                Right => (coords.0 - 1, coords.1 + 1),
            },
            (max_x, _) => match direction {
                Up => (coords.0 - 1, coords.1 - 1),
                Down => (coords.0, coords.1 - 1),
                Left => (coords.0 - 1, coords.1 - 1),
                Right => (coords.0 - 1, coords.1 + 1),
            },
            (_, max_y) => match direction {
                Up => (coords.0 - 1, coords.1 - 1),
                Down => (coords.0 + 1, coords.1 - 1),
                Left => (coords.0 - 1, coords.1 - 1),
                Right => (coords.0 - 1, coords.1),
            },
            (max_x, max_y) => match direction {
                Up => (coords.0 - 1, coords.1 - 1),
                Down => (coords.0, coords.1 - 1),
                Left => (coords.0 - 1, coords.1 - 1),
                Right => (coords.0 - 1, coords.1),
            },
        };

        for (offset_x, row) in input.iter().enumerate() {
            for (offset_y, tile) in row.iter().enumerate() {
                // Compute the coordinate, and store the content if is not None
                if tile.content != Content::None {
                    // Compute tile coordinates
                    let tile_coords = match direction {
                        Up => (coords.0 - offset_x, coords.1 + offset_y),
                        Down => (coords.0 + offset_x, coords.1 + offset_y),
                        Left => (coords.0 + offset_x, coords.1 - offset_y),
                        Right => (coords.0 + offset_x, coords.1 + offset_y),
                    };
                    let tuple = (tile.content.get_value().0.unwrap_or_else(|| 0), tile_coords);

                    // Insert the coordinates of the tile and quantity of its content in its set
                    if let Some(entry) = result.get_mut(&tile.content.to_default()) {
                        entry.insert(tuple);
                    } else {
                        result.insert(tile.content.to_default(), HashSet::from([tuple]));
                    }
                }
            }
        }
    }

    // Collect the content by destroying the tile
    fn destroy_tile(robot: &mut impl Runnable, world: &mut World, tile_coords: (usize, usize)) {
        let (tile_x, tile_y) = tile_coords;
        let (robot_x, robot_y) = Self::get_coordinates(robot);

        // Calculate the coordinates difference between the tile and the robot
        let (mut delta_x, mut delta_y) = (
            robot_x as i64 - tile_x as i64,
            robot_y as i64 - tile_y as i64,
        );

        // Try to reach the tile until the robot is next to it
        while delta_x.abs() + delta_y.abs() != 1 {
            // Calculate the direction
            let mut direction = Down;
            if delta_x.abs() != 1 {
                if delta_x < 0 {
                    direction = Down;
                } else if delta_x >= 0 {
                    direction = Up;
                }
            } else {
                if delta_y < 0 {
                    direction = Right;
                } else if delta_y >= 0 {
                    direction = Left;
                }
            }

            if direction == Up || direction == Down {
                // If the robot cannot proceed try another direction first
                if interface::go(robot, world, direction.clone()).is_err() {
                    if delta_y < 0 {
                        direction = Right;
                    } else if delta_y >= 0 {
                        direction = Left;
                    }

                    if interface::go(robot, world, direction.clone()).is_err() {
                        // If the robot cannot proceed return
                        return;
                    } else {
                        delta_y += if direction == Right { 1 } else { -1 };
                    }
                } else {
                    delta_x += if direction == Down { 1 } else { -1 };
                }
            } else {
                // If the robot cannot proceed try another direction first
                if interface::go(robot, world, direction.clone()).is_err() {
                    if delta_x < 0 {
                        direction = Down;
                    } else if delta_x >= 0 {
                        direction = Up;
                    }

                    if interface::go(robot, world, direction.clone()).is_err() {
                        // If the robot cannot proceed return
                        return;
                    } else {
                        delta_x += if direction == Down { 1 } else { -1 };
                    }
                } else {
                    delta_y += if direction == Right { 1 } else { -1 };
                }
            }
        }

        let (robot_x, robot_y) = Self::get_coordinates(robot);

        // Destroy the tile
        let mut direction = Down;
        if robot_x + 1 == tile_x && robot_y == tile_y {
            direction = Down;
        } else if robot_x > 0 && robot_x - 1 == tile_x && robot_y == tile_y {
            direction = Up;
        } else if robot_x == tile_x && robot_y + 1 == tile_y {
            direction = Right;
        } else if robot_x == tile_x && robot_y > 0 && robot_y - 1 == tile_y {
            direction = Left;
        }

        let _ = interface::destroy(robot, world, direction);
    }

    // Get the robot coordinates as a tuple
    fn get_coordinates(robot: &impl Runnable) -> (usize, usize) {
        (
            robot.get_coordinate().get_row(),
            robot.get_coordinate().get_col(),
        )
    }
}
