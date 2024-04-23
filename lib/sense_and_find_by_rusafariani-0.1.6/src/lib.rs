//! # Lssf
//! Short for "Large Scal Sensing and Find", it's the tool that is in charge
//! of mainly two jobs, to efficiently estimate the composition of the map,
//! giving the user the ability to know how the world looks like, and to
//! search for the shortest paths to the various entities of the section of
//! the map that users already know
//!
//! # Large Scale Sensing
//! The tool's main functionality is sampling the map at regular intervals to return
//! an estimated map.
//! TileType and Content are estimated using nearest-neighbor interpolation, the elevation is
//! estimated using bilinear interpolation to get a smoother result, making it possible to
//! get precise estimations of path's cost when passing the map to a pathfinding tool.
//!
//! The sensing part of the tool offers the following three methods:
//! * `sense_raw_centered_square()`: it senses a square portion of the map centered on the position of the robot
//! * `sense_raw_square_by_center()`: it senses a square portion of the map centered on the coordinate taken as input
//! * `sense_raw_square_by_corner()`: it senses a square portion of the map having it's top left corner positioned on
//! the coordinate taken as input
//!
//! Each of these methods return a `Result<Vec<Vec<((usize, usize), Tile, bool)>>, LibError>`
//! where the tuple `(usize, usize)` represents the coordinate of `World` to which the element of the returned
//! matrix corresponds and the `bool` is true if the `Tile` was actually sensed (so if you can be sure it is of the given tiletype)
//! and `false` if the `Tile` was "estimated".
//!
//! These three methods check if the robot has enough energy before starting the sensing process and return `LibErro::NotEnoughEnergy` if it doesn't.
//!
//! To check whether the robot has enough energy or not the tool checks which `Tiles` have already been discovered so that it doesn't waste energy
//! to "discover" an already discovered `Tile`.
//!
//! You can check yourself the energy consumption by calling the methods:
//! * `sense_square_energy_consumption()`
//! * `sense_square_energy_consumption_by_corner()`
//! consult the methods docs for detailed information about these methods.
//!
//! These methods are pretty expansive to call since they call `robot_map()`, so we suggest calling them once
//! and storing the result for as long as necessary.
//!
//! The last method in the sensing side of the tool is `smart_sensing_centered()`, it is a mashup between
//! the two macro-functionalities of the tool (sensing and find).
//!
//! This method senses a square map centered on the position of the robot (as `sense_raw_centered_square()`) and instead
//! of returning the map it stores it inside the tool making it possible to use the pathfinding functionalities on it.
//! This method also automatically keep track of the offset of the map.
//!
//! Another way of using the pathfinding functionalities on sensed maps is to pass the maps to `update_map()`, but be
//! aware that you'll have to save and properly use the "offset" of the map.
//!
//! # Find
//! This section of the tool works with an inner map, and its usage is divided
//! in three steps, to maximize flexibility:
//! - Map initialization: Lssf memorizes the map
//! - Cost initialization: Lssf calculates the costs given the start coordinates
//! - Result Retrieval: Lssf has a series of function of give the user useful specific
//! pieces of information
//!
//! It has been chosen to use this model because it gives the user the maximum amount
//! of flexibility and control of the actions you actually want the tool to execute.
//! In fact, you can choose to initialize the map once and only then update the costs
//! multiple times on the same map, and every time you update the costs you can choose
//! to use all the family of `get()` functions multiple times on different parameters
//!
//! This way you can avoid repeating the same action if you want to use this functionality
//! on slightly different parameters
//!
//! Note that the map only tell the minimal costs, regardless of the Environmental Conditions,
//! so please note that the actual energy spent may be higher since the tool supposes that
//! the Weather Conditional are optimal and don't require more energy than the energy needed
//! to move on a specific tile
//!
//! ## What can you do?
//! - `update_map()`: updates the map, which can be `robot_map()`, or even your custom map
//! - `update_cost()` functions: calculate the costs from a given center
//! - `get_cost()`: receives the optional cost to get from the center chosen in one of the
//! `update_cost()` functions to the given coordinates
//! - `get_tiletype_vec()`: returns all the found instances of the given tiletypes in order
//! - `get_content_vec()`: returns all the found instances of the given content in order
//! - `get_action_vec()`: return the vector of actions that must be taken to reach a tile
//!
//! # Uniting the functionalities
//! The function `smart_sensing_centered()` unites the capabilities of the tool, so that
//! Lssf is now able to both sense the map and calculate the approximate costs in order
//! to explore the unknown as best as possible
//!
//! # Notes on the Find functionality
//! The functions that use the Find functionality rely on the correct setting, so knowing how
//! it works is fundamental. From now on I will refer to "Map Initialization" as MAP,
//! "Cost initialization" as COST and "Result Retrieval" as GET.
//!
//! The dependency graph looks like this:
//!
//! MAP -> COST -> GET
//!
//! where A -> B means "A is needed for B"
//!
//! This means that using one of the functions that does MAP, will invalidate both COST and
//! GET, while the ones that do COST will invalidate GET.
//! This is the the list of the functions that provide the Find functionality with their
//! respective action associated with them:
//! - `update_map()`: MAP
//! - `smart_sensing_centered()`: MAP + COST
//! - `update_cost()`: COST
//! - `update_cost_constrained()`: COST
//! - `update_cost_constrained_cost()`: COST
//! - `update_cost_constrained_percentage()`: COST
//! - `get_cost()`: GET
//! - `get_tiletype_vec()`: GET
//! - `get_content_vec()`: GET
//! - `get_action_vec()`: GET
//!
//! # About the offset
//!
//! When passing a map to `update_map()` the tool can't know the mapping `position_in_the_map -> coordinate_in_World`
//! unless you save the map using `smart_sensing_centered()`.
//!
//! If you don't use `smart_sensing_centered()` you will have to manually add/subtract the offset to pass from `position_in_the_map` to `coordinate_in_World`
//! and vice versa, otherwise, when you use `smart_sensing_centered()` the tool will be able to calculate the offset on it's own and
//! automatically add/subtract it to let you use real `World`'s coordinates.
//!
//! ## Calculating the offset
//!
//! To calculate the offset you can:
//! * If you sense the map using `sense_raw_square_by_corner()`: use `corner`
//! * If you sense the map using `sense_raw_square_by_center()` or `sense_raw_centered_square`: use the tuple of `(usize, usize)`
//! in the first position (`[0][0]`) of the returned matrix, namely `returned_matrix[0][0].0`
//!
//! More information about the offset and how to use it in `smart_sensing_centered()` and in the pathfinding methods docs.
//!

use crate::large_scale_senser::LargeScaleSenser;
pub use crate::smartmap::Action;
use crate::smartmap::SmartMap;
use robotics_lib::interface::Tools;
use robotics_lib::runner::Runnable;
use robotics_lib::utils::LibError;
use robotics_lib::world::tile::{Content, Tile, TileType};
use robotics_lib::world::World;
pub(crate) mod large_scale_senser;
pub(crate) mod smartmap;

pub struct Lssf {
    lss: LargeScaleSenser,
    sm: SmartMap,
    offset: Option<(usize, usize)>,
}

impl Lssf {
    /// Restituisce un nuovo Lssf (Large Scale Sensing and Find)
    pub fn new() -> Self {
        Lssf {
            lss: LargeScaleSenser::new(),
            sm: SmartMap::new(),
            offset: None,
        }
    }

    /// # input:
    /// * `l`: the side of the square map to be sampled
    /// * `world`: no need to explain
    /// * `robot`: do I need to repeat myself?
    /// * `granularity`: it is equal to the side of the square obtained from a single sample - 1, the lower it is, the higher the resolution increases... along with the cost
    ///
    /// # output:
    /// `Result<Vec<Vec<((usize, usize), Tile, bool)>>, LibError>` ... oh well, things don't always go well.
    ///
    /// The tool checks whether there is enough energy before even detecting the first sample, if the energy is not sufficient, it returns `Err(LibError::NotEnoughEnergy)`.
    ///
    /// The tool checks that the square to be detected falls within the limits of the map, otherwise it returns `Err(LibError::OutOfBounds)`.
    ///
    /// If everything goes well, the function returns `Ok(Vec<Vec<((usize, usize), Tile, bool)>>)` where the tuple of `usize` associated with each `Tile` represents the real coordinate to which the `Tile` corresponds
    /// and the boolean is `true` if the tile is present in robot_map (so if there is certainty that the returned tile corresponds to the real one).
    ///
    /// The returned map is centered on the position of the robot, in case `l` is odd, the center is simply located at position `[l / 2][l / 2]` of the matrix,
    /// in case `l` is even, the top-left "cell" of the four that make up the center is considered as the center, that is `[l / 2 - 1][l / 2 - 1]`, with the exception of the case `l == 0` in which the function returns the "matrix" `vec![]`.
    ///
    /// For example, for `l = 6` the situation would be as follows:
    ///
    /// | |0|1|2|3|4|5|
    /// |-|-|-|-|-|-|-|
    /// |0| | | | | | |
    /// |1| | | | | | |
    /// |2| | |O|X| | |
    /// |3| | |X|X| | |
    /// |4| | | | | | |
    /// |5| | | | | | |
    ///
    /// with the robot in position `[2][2]`.
    pub fn sense_raw_centered_square(
        &mut self,
        l: usize,
        world: &mut World,
        robot: &mut impl Runnable,
        granularity: usize,
    ) -> Result<Vec<Vec<((usize, usize), Tile, bool)>>, LibError> {
        self.lss.sense_centered_square(l, world, robot, granularity)
    }

    /// # input:
    /// * `l`: the side of the square map to be sampled
    /// * `world`: no need to explain
    /// * `robot`: do I need to repeat myself?
    /// * `granularity`: it is equal to the side of the square obtained from a single sample - 1, the lower it is, the higher the resolution increases... along with the cost
    /// * `center`: the coordinate of the map on which the matrix will be centered
    ///
    /// # output:
    /// `Result<Vec<Vec<((usize, usize), Tile, bool)>>, LibError>` ... oh well, things don't always go well.
    ///
    /// The tool checks whether there is enough energy before even detecting the first sample, if the energy is not sufficient, it returns `Err(LibError::NotEnoughEnergy)`.
    ///
    /// The tool checks that the square to be detected falls within the limits of the map, otherwise it returns `Err(LibError::OutOfBounds)`.
    ///
    /// If everything goes well, the function returns `Ok(Vec<Vec<((usize, usize), Tile, bool)>>)` where the tuple of `usize` associated with each `Tile` represents the real coordinate to which the `Tile` corresponds
    /// and the boolean is `true` if the tile is present in robot_map (so if there is certainty that the returned tile corresponds to the real one).
    ///
    /// The returned map is centered on `center`, in case `l` is odd, the center is simply located at position `[l / 2][l / 2]` of the matrix,
    /// in case `l` is even, the top-left "cell" of the four that make up the center is considered as the center, that is `[l / 2 - 1][l / 2 - 1]`, with the exception of the case `l == 0` in which the function returns the "matrix" `vec![]`.
    ///
    /// For example, for `l = 6` the situation would be as follows:
    ///
    /// | |0|1|2|3|4|5|
    /// |-|-|-|-|-|-|-|
    /// |0| | | | | | |
    /// |1| | | | | | |
    /// |2| | |O|X| | |
    /// |3| | |X|X| | |
    /// |4| | | | | | |
    /// |5| | | | | | |
    ///
    /// with the center at position `[2][2]`.
    pub fn sense_raw_square_by_center(
        &mut self,
        l: usize,
        world: &mut World,
        robot: &mut impl Runnable,
        granularity: usize,
        center: (usize, usize),
    ) -> Result<Vec<Vec<((usize, usize), Tile, bool)>>, LibError> {
        self.lss
            .sense_square_by_center(l, world, robot, granularity, center)
    }

    /// # input:
    /// * `l`: the side of the square map to be sampled
    /// * `world`: no need to explain
    /// * `robot`: do I need to repeat myself?
    /// * `granularity`: it is equal to the side of the square obtained from a single sample - 1, the lower it is, the higher the resolution increases... along with the cost
    /// * `corner`: the coordinate of the map that will correspond to the top-left corner of the matrix
    ///
    /// # output:
    /// `Result<Vec<Vec<((usize, usize), Tile, bool)>>, LibError>` ... oh well, things don't always go well.
    ///
    /// The tool checks whether there is enough energy before even detecting the first sample, if the energy is not sufficient, it returns `Err(LibError::NotEnoughEnergy)`.
    ///
    /// The tool checks that the square to be detected falls within the limits of the map, otherwise it returns `Err(LibError::OutOfBounds)`.
    ///
    /// If everything goes well, the function returns `Ok(Vec<Vec<((usize, usize), Tile, bool)>>)` where the tuple of `usize` associated with each `Tile` represents the real coordinate to which the `Tile` corresponds
    /// and the boolean is `true` if the tile is present in robot_map (so if there is certainty that the returned tile corresponds to the real one).
    ///
    /// The `Tile` at position `[0][0]` in the matrix corresponds to the `Tile` in `World` with coordinates `corner`.
    ///
    /// In case `l == 0`, the function returns the "matrix" `vec![]`.
    pub fn sense_raw_square_by_corner(
        &mut self,
        l: usize,
        world: &mut World,
        robot: &mut impl Runnable,
        granularity: usize,
        corner: (usize, usize),
    ) -> Result<Vec<Vec<((usize, usize), Tile, bool)>>, LibError> {
        self.lss
            .sense_square_by_corner(l, world, robot, granularity, corner)
    }

    

    /// The function detects a portion of the map as `sense_raw_centered_square()` would,
    /// but instead of returning the detected map, it calculates the minimum paths from the robot to the other
    /// tiles and saves it in the tool.
    ///
    /// Information can then be "extracted," such as:
    /// * The cost to reach a coordinate: `get_cost()`
    /// * The cost and the last action required to reach a coordinate: `get_cost_and_action()`
    /// * Given a `TileType`, a vector containing the coordinates where the `Tile`s have that `TileType` is obtained, ordered in ascending order of cost to reach the coordinate: `get_tiletype_vec()`
    /// * Given a `Content`, a vector containing the coordinates where the `Tile`s contain that `Content` is obtained, ordered in ascending order of cost to reach the coordinate: `get_content_vec()`
    /// * Given a coordinate, a vector containing the `Action` to "execute" to reach that coordinate is obtained: `get_action_vec()`
    ///
    /// It should be noted that the coordinates passed to `get_cost()`, `get_cost_and_action()`, `get_tiletype_vec()`, `get_content_vec()`, and `get_action_vec()` are the actual coordinates that the `Tile` has in `World`
    /// if and only if the map has been saved through the use of this method; calling `update_map()` manually will return us to the situation where the coordinates passed to the methods mentioned above are relative to the map saved in the tool and not to `World`.
    ///
    /// To know if it is possible to use coordinates relative to `World` instead of those relative to the map saved in the tool, `is_offset_valid()` can be called, which
    /// returns `true` if real coordinates can be used and `false` if the offset must be taken into account.
    ///
    /// # input:
    /// * `l`: the side of the square map to be sampled
    /// * `world`: no need to explain
    /// * `robot`: do I need to repeat myself?
    /// * `granularity`: it is equal to the side of the square obtained from a single sample - 1, the lower it is, the higher the resolution increases... along with the cost
    ///
    /// # output:
    /// `Result<Option<(usize, usize)>, LibError>`
    ///
    /// The tool checks whether there is enough energy before even detecting the first sample, if the energy is not sufficient, it returns `Err(LibError::NotEnoughEnergy)`.
    ///
    /// The tool checks that the square to be detected falls within the limits of the map, otherwise it returns `Err(LibError::OutOfBounds)`.
    ///
    /// If everything goes well, the function returns `Ok(Option<(usize, usize)>)` where the tuple of `usize` represents the coordinate of `World` corresponding to the element `[0][0]` in the detected map, and the `Option` will be `None` in case `l` is 0.
    ///
    /// The detected map is centered on the position of the robot, in case `l` is odd, the center is simply located at position `[l / 2][l / 2]` of the matrix,
    /// in case `l` is even, the top-left "cell" of the four that make up the center is considered as the center, that is `[l / 2 - 1][l / 2 - 1]`, with the exception of the case `l == 0` in which the function "detects" the "matrix" `vec![]`.
    ///
    /// For example, for `l = 6` the situation would be as follows:
    ///
    /// | |0|1|2|3|4|5|
    /// |-|-|-|-|-|-|-|
    /// |0| | | | | | |
    /// |1| | | | | | |
    /// |2| | |O|X| | |
    /// |3| | |X|X| | |
    /// |4| | | | | | |
    /// |5| | | | | | |
    ///
    /// with the robot in position `[2][2]`.
    pub fn smart_sensing_centered(
        &mut self,
        l: usize,
        world: &mut World,
        robot: &mut impl Runnable,
        granularity: usize,
    ) -> Result<(), LibError> {
        let raw_map = self
            .lss
            .sense_centered_square(l, world, robot, granularity)?;
        let coord: Option<(usize, usize)> = if raw_map.len() > 0 {
            Some(raw_map[0][0].0)
        } else {
            None
        };
        let map = raw_map
            .into_iter()
            .map(|r| r.into_iter().map(|c| Some(c.1)).collect())
            .collect();
        self.sm.update_map(&map);
        let rc = robot.get_coordinate();
        match coord.clone() {
            Some(c) => {
                self.sm.update_cost(rc.get_row() - c.0, rc.get_col() - c.1)?;
            },
            None => {}
        }
        self.offset = coord;
        Ok(())
    }

    /// # input
    /// * `l: usize` the side of the square to be detected
    /// * `granularity: usize` the interval between one sample and the next - 1
    /// * `center: (usize, usize)` the coordinate that will be the center of the detected square
    /// * `world: &World` you know
    ///
    /// # output
    /// `usize` the energy cost related to the detection of the square
    ///
    /// The method calculates the energy cost of the detection by counting the number of `Tile`s to be detected
    /// and subtracting the number of `Tile`s already present in `robot_map()`. It is precisely because
    /// the method excludes already detected `Tile`s from the count that specifying a center is necessary.
    ///
    /// For more information on the detected square or the center, refer to the documentation of `sense_raw_square_by_center()`.
    ///
    /// It is advisable to limit the use of the method to the essential minimum since each call involves
    /// a call to `robot_map()` (which can be quite heavy with large maps).
    pub fn sense_square_energy_consumption(&self, l: usize, granularity: usize, center: (usize, usize), world: &World) -> usize {
        LargeScaleSenser::sense_square_energy_consumption(l, granularity, center, world)
    }

    /// # input
    /// * `l: usize` the side of the square to be detected
    /// * `granularity: usize` the interval between one sample and the next - 1
    /// * `corner: (usize, usize)` the coordinate that will correspond to the top-left corner of the detected square
    /// * `world: &World` you know
    ///
    /// # output
    /// `usize` the energy cost related to the detection of the square
    ///
    /// The method calculates the energy cost of the detection by counting the number of `Tile`s to be detected
    /// and subtracting the number of `Tile`s already present in `robot_map()`. It is precisely because
    /// the method excludes already detected `Tile`s from the count that specifying a corner is necessary.
    ///
    /// For more information on the detected square or the use of the corner, refer to the documentation of `sense_raw_square_by_center()`.
    ///
    /// It is advisable to limit the use of the method to the essential minimum since each call involves
    /// a call to `robot_map()` (which can be quite heavy with large maps).
    pub fn sense_square_energy_consumption_by_corner(&self, l: usize, granularity: usize, corner: (usize, usize), world: &World) -> usize {
        LargeScaleSenser::sense_square_energy_consumption_by_corner(l, granularity, corner, world)
    }

    /// Updates the tool's internal map
    ///
    /// It is given as a separate function to the update cost, so that it is
    /// possible to update the map once, and update the costs multiple times
    /// using different coordinates.
    ///
    /// # Input
    /// - A reference to a `Vec<Vec<Option<Tile>>>`, and this can be obtained with
    /// the Common Crate's `robot_map()`
    ///
    /// # Notes
    /// If you want to use the other functions related to path finding, which are
    /// `update_cost`, `get_cost()`, `get_cost_and_action()`, `get_tiletype_vec()`,
    /// `get_content_vec()` and `get_acton_vec()`, it is required to use this function.
    ///
    /// If not:
    /// - You may be searching paths in an uninitialized internal map, and functions
    /// will return `[]`, `None` or `Err(OutOfBounds)`, depending on the error
    /// - You may be searching paths in an outdated version of the map, and this makes
    /// the functions mentioned above return misleading results
    ///
    /// Calling this method makes `is_offset_valid()` return `false`, so, the coordinated given to
    /// and taken by the pathfinding methods will be considered as relative to the internal map instead
    /// of the real map (`World`).
    ///
    /// To make `is_offset_valid()` return `true` again, and so, to make the pathfinding methods work
    /// with real coordinates again, it is necessary to call the method `smart_sensing_centered()`, but it will
    /// change the internal map.
    pub fn update_map(&mut self, map: &Vec<Vec<Option<Tile>>>) {
        self.sm.update_map(map);
        self.offset = None;
    }

    ///Calculate costs from a given coordinate
    ///
    /// It is possible to use this function multiple times on the same map and save
    /// the results using  `get_cost()`, `get_cost_and_action()`, `get_tiletype_vec()`,
    /// `get_content_vec()` and `get_acton_vec()`
    ///
    /// It is equal to `update_cost_constrained(row,col,100,usize::MAX)`
    ///
    /// # Input
    /// - `row` and `col` as in the coordinates `(row,col)`
    ///
    /// # Output
    /// - `Ok(())` if the coordinates given are valid in the current inner map,
    /// otherwise it returns `Err(OutOfBounds)`
    ///
    /// # Notes
    /// It is required to use `update_map()` before using this function, otherwise you will
    /// always get Err(OutOfBounds), since the internal map would have length of zero
    ///
    /// If `is_offset_valid()` returns `true` then the coordinate given to this method as input will be considered as
    /// relative to `World`, otherwise it will be considered as relative to the inner map.
    pub fn update_cost(&mut self, mut row: usize, mut col: usize) -> Result<(), LibError> {
        self.subtract_offset(&mut row, &mut col)?;
        self.sm.update_cost(row, col)
    }

    ///Calculate costs from a given coordinate and with the restraint of cost and number
    /// of tiles
    ///
    /// It is possible to use this function multiple times on the same map and save
    /// the results using  `get_cost()`, `get_cost_and_action()`, `get_tiletype_vec()`,
    /// `get_content_vec()` and `get_acton_vec()`
    ///
    /// This is the most generic variant of the `update_cost()` variants
    ///
    /// # Input
    /// - `row` and `col` as in the coordinates `(row,col)`
    /// - `percentage`: a `usize` ranged 0-100 to indicate the percentage of the total map to
    /// search for minimal paths, calculated on map dimensions
    /// - `max_cost`: a `usize` that indicates the maximum cost of the paths before it stops
    ///
    /// # Output
    /// - `Ok(())` if the coordinates given are valid in the current inner map,
    /// otherwise it returns `Err(OutOfBounds)`
    ///
    /// # Panics
    /// This function will panic if the percentage is above 100
    ///
    /// # Notes
    /// It is required to use `update_map()` before using this function, otherwise you will
    /// always get Err(OutOfBounds), since the internal map would have length of zero
    ///
    /// If `is_offset_valid()` returns `true` then the coordinate given to this method as input will be considered as
    /// relative to `World`, otherwise it will be considered as relative to the inner map.
    pub fn update_cost_constrained(
        &mut self,
        mut row: usize,
        mut col: usize,
        percentage: usize,
        max_cost: usize,
    ) -> Result<(), LibError> {
        self.subtract_offset(&mut row, &mut col)?;
        self.sm
            .update_cost_constrained(row, col, percentage, max_cost)
    }

    ///Calculate costs from a given coordinate and with the restraint of cost
    ///
    /// It is possible to use this function multiple times on the same map and save
    /// the results using  `get_cost()`, `get_cost_and_action()`, `get_tiletype_vec()`,
    /// `get_content_vec()` and `get_acton_vec()`
    ///
    /// It is equal to `update_cost_constrained(row,col,100,max_cost)`
    ///
    /// # Input
    /// - `row` and `col` as in the coordinates `(row,col)`
    /// - `max_cost`: a `usize` that indicates the maximum cost of the paths before it stops
    ///
    /// # Output
    /// - `Ok(())` if the coordinates given are valid in the current inner map,
    /// otherwise it returns `Err(OutOfBounds)`
    ///
    /// # Notes
    /// It is required to use `update_map()` before using this function, otherwise you will
    /// always get Err(OutOfBounds), since the internal map would have length of zero
    ///
    /// If `is_offset_valid()` returns `true` then the coordinate given to this method as input will be considered as
    /// relative to `World`, otherwise it will be considered as relative to the inner map.
    pub fn update_cost_constrained_cost(
        &mut self,
        mut row: usize,
        mut col: usize,
        max_cost: usize,
    ) -> Result<(), LibError> {
        self.subtract_offset(&mut row, &mut col)?;
        self.sm.update_cost_constrained_cost(row, col, max_cost)
    }

    ///Calculate costs from a given coordinate and with the restraint of number of tiles
    ///
    /// It is possible to use this function multiple times on the same map and save
    /// the results using  `get_cost()`, `get_cost_and_action()`, `get_tiletype_vec()`,
    /// `get_content_vec()` and `get_acton_vec()`
    ///
    /// It is equal to `update_cost_constrained(row,col,percentage,usize::MAX)`
    ///
    /// # Input
    /// - `row` and `col` as in the coordinates `(row,col)`
    /// - `percentage`: a `usize` ranged 0-100 to indicate the percentage of the total map to
    /// search for minimal paths, calculated on map dimensions
    ///
    /// # Output
    /// - `Ok(())` if the coordinates given are valid in the current inner map,
    /// otherwise it returns `Err(OutOfBounds)`
    ///
    /// # Panics
    /// This function will panic if the percentage is above 100
    ///
    /// # Notes
    /// It is required to use `update_map()` before using this function, otherwise you will
    /// always get Err(OutOfBounds), since the internal map would have length of zero
    ///
    /// If `is_offset_valid()` returns `true` then the coordinate given to this method as input will be considered as
    /// relative to `World`, otherwise it will be considered as relative to the inner map.
    pub fn update_cost_constrained_percentage(
        &mut self,
        mut row: usize,
        mut col: usize,
        percentage: usize,
    ) -> Result<(), LibError> {
        self.subtract_offset(&mut row, &mut col)?;
        self.sm
            .update_cost_constrained_percentage(row, col, percentage)
    }

    /// Return the cost to go from the coordinate specified in `update_cost()` to the
    /// given coordinate
    ///
    /// # Input
    /// - `xc` and `yc`, as in `(xc,yc)`
    ///
    /// # Output
    /// - `Some(usize)` if the coordinate given to the function is valid, with the `usize`
    /// inside being the cost to get to the specified coordinates
    /// - `None` if one of three cases happen:
    ///     - The tile is discovered but is unreachable
    ///     - The tile is still undiscovered
    ///     - The coordinates point to a nonexistent tile
    ///
    /// If `is_offset_valid()` returns `true` then the coordinate given to this method as input will be considered as
    /// relative to `World`, otherwise it will be considered as relative to the inner map.
    pub fn get_cost(&self, mut xc: usize, mut yc: usize) -> Option<usize> {
        self.subtract_offset(&mut xc, &mut yc).ok()?;
        self.sm.get_cost(xc, yc)
    }

    /// Return the cost and the last action to go from the coordinate specified in
    /// `update_cost()` to the given coordinate
    ///
    /// # Input
    /// - `xc` and `yc`, as in `(xc,yc)`
    ///
    /// # Output
    /// - `Some(usize,Action)` if the coordinate given to the function is valid, with the `usize`
    /// inside being the cost to get to the specified coordinates and the `Action` being the last
    /// action made to reach the coordinates, or in other words the action made from the ancestor
    /// in the minimal path to reach the coordinates
    /// - `None` if one of three cases happen:
    ///     - The tile is discovered but is unreachable
    ///     - The tile is still undiscovered
    ///     - The coordinates point to a nonexistent tile
    ///
    /// If `is_offset_valid()` returns `true`, if the `Action` returned is of type `Action::Teleport(usize, usize)` the coordinate inside `Teleport`
    /// will be relative to `World`, otherwise it will be relative to the inner map.
    pub fn get_cost_and_action(&self, mut xc: usize, mut yc: usize) -> Option<(usize, Action)> {
        self.subtract_offset(&mut xc, &mut yc).ok()?;
        self.sm.get_cost_and_action(xc, yc)
    }

    /// Returns the coordinates of all the tiles that have been reached and correspond
    /// to the specified `Tiletype`in increasing order of cost
    ///
    /// # Input
    /// - `tiletype`: a reference to a `Tiletype` variant
    ///
    /// # Output
    /// - A `Vec<(usize,usize)>` containing in increasing order of cost, all coordinates
    /// that contain the specified `Tiletype`
    ///
    /// # Examples
    /// ```rust
    /// use robotics_lib::world::tile::TileType;
    /// use rustafariani_sense_and_find::*;
    /// let mut lssf = Lssf::new();
    /// // initialization left to the user
    ///
    /// //Taking the first three elements
    /// let tiles: Vec<_> = lssf.get_tiletype_vec(&TileType::Street)
    ///     .into_iter()
    ///     .take(3)
    ///     .collect();
    ///
    /// //Taking coordinates costing less than MAX_COST
    /// const MAX_COST: usize = 30;
    /// let tiles: Vec<_> = lssf.get_tiletype_vec(&TileType::Snow)
    ///     .into_iter()
    ///     .take_while(|(x,y)| {
    ///         let temp = lssf.get_cost(*x,*y);
    ///         if temp.is_none() {
    ///             return false;
    ///         }
    ///         let temp = temp.unwrap();
    ///         temp < MAX_COST
    ///     })
    ///     .collect();
    ///
    /// ```
    ///
    /// # Notes
    /// The return value can be an empty `Vec`, and this happens in two cases:
    /// - It has not been found any tiles with the specified `Tiletype`
    /// - The map has not been initialized, or in other words neither `update_map()`
    /// nor any `update_cost()` function have been called before
    ///
    /// `Teleport(true)` and `Teleport(false)` are considered different variants, so
    /// when given as input, this function will return different result.
    ///
    /// Since the behaviour for walkable and unwalkable tiletypes is the same (in other
    /// words
    ///
    /// If `is_offset_valid()` returns `true` then the coordinates returned will be relative to `World` (so corresponding to the real coordinates),
    /// otherwise they will be relative to the inner map.
    pub fn get_tiletype_vec(&self, tiletype: &TileType) -> Vec<(usize, usize)> {
        self.sm
            .get_tiletype_vec(tiletype)
            .into_iter()
            .map(|mut c| {
                if self.is_offset_valid() {
                    c.0 += self.offset.unwrap().0;
                    c.1 += self.offset.unwrap().1;
                }
                (c.0, c.1)
            })
            .collect()
    }

    /// Returns the coordinates of all the tiles that have been reached and correspond
    /// to the specified `Content`in increasing order of cost
    ///
    /// # Input
    /// - `content`: a reference to a `Content` variant
    ///
    /// # Output
    /// - A `Vec<(usize,usize)>` containing in increasing order of cost, all coordinates
    /// that contain the specified `Content`
    ///
    /// # Example
    /// ```rust
    /// use robotics_lib::world::tile::{Content, TileType};
    /// use rustafariani_sense_and_find::*;
    /// let mut lssf = Lssf::new();
    /// // initialization left to the user
    ///
    /// //Taking the first three elements
    /// let contents: Vec<_> = lssf.get_content_vec(&Content::Coin(3))
    /// // It will find every tile with some quantity of Coin
    ///     .into_iter()
    ///     .take(3)
    ///     .collect();
    ///
    /// //Taking coordinates costing less than MAX_COST
    /// const MAX_COST: usize = 30;
    /// let contents: Vec<_> = lssf.get_content_vec(&Content::Bin(2..4))
    /// // It will find every tile with any range of Bin
    ///     .into_iter()
    ///     .take_while(|(x,y)| {
    ///         let temp = lssf.get_cost(*x,*y);
    ///         if temp.is_none() {
    ///             return false;
    ///         }
    ///         let temp = temp.unwrap();
    ///         temp < MAX_COST
    ///     })
    ///     .collect();
    ///
    /// ```
    ///
    /// # Notes
    /// The return value can be an empty `Vec`, and this happens in two cases:
    /// - It has not been found any tiles with the specified `Content`
    /// - The map has not been initialized, or in other words neither `update_map()`
    /// nor any `update_cost()` function have been called before
    ///
    /// Any `Content` with the same name is considered the same, regardless of its internal
    /// value, eg. `Rock(3)` is the same as `Rock(7)`
    ///
    /// If `is_offset_valid()` returns `true` then the coordinates returned will be relative to `World` (so corresponding to the real coordinates),
    /// otherwise they will be relative to the inner map.
    pub fn get_content_vec(&self, content: &Content) -> Vec<(usize, usize)> {
        self.sm
            .get_content_vec(content)
            .into_iter()
            .map(|mut c| {
                if self.is_offset_valid() {
                    c.0 += self.offset.as_ref().unwrap().0;
                    c.1 += self.offset.as_ref().unwrap().1;
                }
                (c.0, c.1)
            })
            .collect()
    }

    /// Returns the `Vec` of actions that must be taken in order to arrive to the destination
    ///
    /// # Input
    /// - `xc` and `yc`, as in `(xc,yc)`
    ///
    /// # Output
    /// - `Ok(Vec<Action>)` if the inserted coordinates are valid, otherwise `Err(OutOfBound)`
    /// is returned
    ///
    /// # Example
    /// ```rust
    /// use robotics_lib::interface::{Direction};
    /// use rustafariani_sense_and_find::*;
    ///
    /// fn move_robot(dir: Direction) {
    ///     //moves the robot
    /// }
    /// fn teleport(i: usize, j: usize) {
    ///     // use a teleport if you are on top of one
    /// }
    /// fn next_move(dir: Action) -> (usize,usize){
    ///     // indicates what coordinate the robot will move
    ///     (1,1) // dummy result
    /// }
    /// fn interact(dir: Action) {
    ///     // interacts with the tile found after the action
    /// }
    ///
    /// let mut lssf = Lssf::new();
    ///
    /// //Initialization left to the user
    ///
    /// //Example for reaching a walkable tile
    /// let act = lssf.get_action_vec(4,7).unwrap();
    /// for action in act.iter() {
    ///     match action {
    ///         Action::North => move_robot(Direction::Up),
    ///         Action::South => move_robot(Direction::Down),
    ///         Action::West => move_robot(Direction::Left),
    ///         Action::East => move_robot(Direction::Right),
    ///         Action::Teleport(i,j) => teleport(*i,*j)
    ///     }
    /// }
    ///
    /// // Example for reaching an unwalkable tile
    /// let act = lssf.get_action_vec(5,6).unwrap();
    /// for action in act.iter() {
    ///     if next_move(action.clone()) == (5,6) {
    ///         break;
    ///     }
    ///     match action {
    ///         Action::North => move_robot(Direction::Up),
    ///         Action::South => move_robot(Direction::Down),
    ///         Action::West => move_robot(Direction::Left),
    ///         Action::East => move_robot(Direction::Right),
    ///         Action::Teleport(i,j) => teleport(*i,*j)
    ///     }
    /// }
    /// interact(act.last().unwrap().clone())
    /// ```
    /// # Notes
    /// The return value can be an empty `Vec`, and this happens in three cases:
    /// - the coordinates are not reachable
    /// - the tile is still unknown
    /// - The map has not been initialized, or in other words neither `update_map()`
    /// nor any `update_cost()` function have been called before
    ///
    /// If the coordinates point to an unwalkable tile, the last action represent the
    /// direction in which you will find the unwalkable tile, so to interact with it
    ///
    /// If `is_offset_valid()` returns `true` then the coordinate given as input will be considered as
    /// relative to `World`, otherwise it will be considered as relative to the inner map.
    ///
    /// If `is_offset_valid()` returns `true`, the `Action`s returned having type `Action::Teleport(usize, usize)`
    /// will contain coordinates relative to `World`, otherwise they will be relative to the inner map.
    pub fn get_action_vec(&self, xc: usize, yc: usize) -> Result<Vec<Action>, LibError> {
        Ok(self
            .sm
            .get_action_vec(xc, yc)?
            .into_iter()
            .map(|/*mut*/ a| {
                if self.is_offset_valid() {
                    match a {
                        Action::Teleport(r, c) => {
                            Action::Teleport(r + self.offset.unwrap().0, c + self.offset.unwrap().1)
                        }
                        b => b,
                    }
                } else {
                    a
                }
            })
            .collect())
    }

    /// The function returns `true` if the tool was able to calculate an offset to use in automatically associating real coordinates (those of `World`)
    /// with the map saved inside it; otherwise, it returns `false`.
    ///
    /// When the function returns `true` and therefore the tool was able to calculate the offset mentioned above, the methods
    /// `update_cost()`, `get_cost()`, `get_action()`, `get_cost_and_action()`, `get_tiletype_vec()`, `get_content_vec()`, and `get_action_vec()`
    /// handle real coordinates (the coordinate `(0, 0)` corresponds to `World[0][0]`) both as input and output (for more details, refer to the documentation of the methods),
    /// otherwise, they treat the coordinates as relative to the map saved in the tool; therefore, the coordinate `(0, 0)` will correspond to `map_saved_in_the_tool[0][0]`.
    pub fn is_offset_valid(&self) -> bool {
        self.offset.is_some()
    }

    fn subtract_offset(&self, row: &mut usize, col: &mut usize) -> Result<(), LibError> {
        if self.is_offset_valid() {
            if *row >= self.offset.unwrap().0 && *col >= self.offset.unwrap().1 {
                *row -= self.offset.unwrap().0;
                *col -= self.offset.unwrap().1;
                Ok(())
            } else {
                Err(LibError::OutOfBounds)
            }
        } else {
            Ok(())
        }
    }

    /*
    /// Taken an `usize` `l` as input the function returns the position that the center of a square having side `l`
    /// would have with respect to the top-left corner.
    pub fn center_by_edge(l: usize) -> (usize, usize) {
        LargeScaleSenser::center_by_edge(l)
    }
    */
}

impl Tools for Lssf {}

#[cfg(test)]
mod tests;
