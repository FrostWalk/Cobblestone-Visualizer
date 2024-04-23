//! mod doc
pub mod spyglass {
    use std::cmp::{max, min, Ordering};
    use std::collections::{HashMap, VecDeque};

    use robotics_lib::interface::{Direction, discover_tiles, robot_view, one_direction_view, robot_map, Tools};
    use robotics_lib::runner::Runnable;
    use robotics_lib::utils::LibError;
    use robotics_lib::utils::LibError::{OperationNotAllowed};
    use robotics_lib::world::tile::{Tile};
    use robotics_lib::world::World;
    use crate::spyglass::Operation::{Center, CrossSide, Square};
    use crate::spyglass::SpyglassResult::{Complete, Stopped};

    /// An enumeration which represent the possible results of a spyglass discovery.
    #[derive(Debug)]
    pub enum SpyglassResult {
        /// The discovery is completed correctly without any error. The usize value represents the number of new tiles discovered.
        Complete(usize),
        /// The discovery was interrupted because a tile (or tiles) was found that matched the stop condition. Inside the vector there are the tiles that matched the condition with their coordinates.
        Stopped(Vec<(Tile, usize, usize)>),
        /// The discovery is paused because the robot energy has run out or the budget is reached. This is the only case that allows the discover to be resumed.
        Paused,
        /// The discovery is failed.
        Failed(LibError)
    }

    struct SquareBound {
        center_row: usize,
        center_col: usize,
        distance: usize,
        max_dim: usize,
        min_dim: usize
    }

    impl SquareBound {

        fn get_top_left_bound(&self) -> (usize, usize) {
            let top = match self.center_row.checked_sub(self.distance) {
                None => { self.min_dim }
                Some(t) => {
                    let t = match t.checked_sub(if self.distance % 2 == 0 { 1 } else { 0 }) {
                        None => {self.min_dim}
                        Some(u) => { u }
                    };
                    max(t, self.min_dim)
                }
            };

            let left = match self.center_col.checked_sub(self.distance) {
                None => { self.min_dim }
                Some(t) => {
                    let t = match t.checked_sub(if self.distance % 2 == 0 { 1 } else { 0 }) {
                        None => {self.min_dim}
                        Some(u) => { u }
                    };
                    max(t, self.min_dim)
                }
            };
            (top, left)
        }

        fn get_bottom_right_bound(&self) -> (usize, usize) {
            let try_bottom = self.center_row+self.distance+1+if self.distance % 2 == 0 { 1 } else { 0 };
            let bottom = if try_bottom >= self.max_dim {
                self.max_dim
            }
            else {
                min(try_bottom, self.max_dim)
            };

            let try_right = self.center_col+self.distance+1+if self.distance % 2 == 0 { 1 } else { 0 };
            let right = if try_right >= self.max_dim {
                self.max_dim
            }
            else {
                min(try_right, self.max_dim)
            };

            (bottom, right)
        }

        pub(crate) fn new(center_row: usize, center_col: usize, distance: usize, min_dim: usize, max_dim: usize) -> Self {
            SquareBound {
                center_row,
                center_col,
                distance,
                max_dim,
                min_dim
            }
        }

        pub(crate) fn split(&self) -> (SquareBound, SquareBound, SquareBound, SquareBound) {
            let center = SquareBound::new(
                self.center_row,
                self.center_col,
                self.distance/2,
                self.min_dim,
                self.max_dim
            );

            let (top, left) = center.get_top_left_bound();
            let (bottom, right) = center.get_bottom_right_bound();

            (
                SquareBound::new(top, left, self.distance/2, self.min_dim, self.max_dim),
                SquareBound::new(top, right, self.distance/2, self.min_dim, self.max_dim),
                SquareBound::new(bottom, left, self.distance/2, self.min_dim, self.max_dim),
                SquareBound::new(bottom, right, self.distance/2, self.min_dim, self.max_dim),
            )
        }

        pub(crate) fn is_inside(&self, row: usize, col: usize) -> bool {
            let top = match self.center_row.checked_sub(self.distance) {
                None => { 0 }
                Some(t) => { t }
            };

            let left = match self.center_col.checked_sub(self.distance) {
                None => { 0 }
                Some(t) => { t }
            };

            let right = min(self.center_col+self.distance+1, self.max_dim);
            let bottom = min(self.center_row+self.distance+1, self.max_dim);

            (top <= row && row < bottom) && (left <= col && col < right)
        }
        //
        // pub(crate) fn center_row(&self) -> usize {
        //     self.center_row
        // }
        //
        // pub(crate) fn center_col(&self) -> usize {
        //     self.center_col
        // }
        //
        // pub(crate) fn distance(&self) -> usize {
        //     self.distance
        // }
        //
        // pub(crate) fn max_dim(&self) -> usize {
        //     self.max_dim
        // }
        //
        // pub(crate) fn min_dim(&self) -> usize {
        //     self.min_dim
        // }
    }

    enum Operation {
        Center,
        CrossSide(Direction),
        Square(SquareBound)
    }

    impl Operation {
        fn get_cost(&self, spyglass: &Spyglass, robot: &mut impl Runnable, world: &mut World) -> usize {
            let n_tiles = match self {
                Center => { 0 }
                CrossSide(dir) => {
                    match dir {
                        Direction::Up => {
                            min(spyglass.bounds.distance, robot.get_coordinate().get_row())
                        }
                        Direction::Down => {
                            min(spyglass.bounds.distance,  spyglass.bounds.max_dim - robot.get_coordinate().get_row() - 1)
                        }
                        Direction::Left => {
                            min(spyglass.bounds.distance, robot.get_coordinate().get_col())
                        }
                        Direction::Right => {
                            min(spyglass.bounds.distance,  spyglass.bounds.max_dim - robot.get_coordinate().get_col() - 1)
                        }
                    }
                }
                Square(bound) => { if robot_map(world).unwrap()[bound.center_row][bound.center_col].is_none() { 1 } else { 0 }}
            };

            n_tiles * 3
        }

        fn execute(&self,
                   spyglass: &mut Spyglass,
                   robot: &mut impl Runnable,
                   world: &mut World) -> Result<(Vec<Vec<Tile>>, usize, usize), LibError> {

            match self {
                Center => {
                    let mut res : Vec<Vec<Tile>> = Vec::new();
                    for row in robot_view(robot, world) {
                        let mut tmp = Vec::new();
                        for tile in row {
                            match tile {
                                None => {
                                }
                                Some(t) => {
                                    tmp.push(t);
                                }
                            }
                        }
                        res.push(tmp);
                    }

                    if res.iter().all(|a| a.len() == res[0].len()) { // if all rows have the same length (the returned matrix should be a square or a rectangle)
                        Ok((
                            res,
                            robot.get_coordinate().get_row().checked_sub(1).unwrap_or(0),
                            robot.get_coordinate().get_col().checked_sub(1).unwrap_or(0)
                        ))
                    }
                    else {
                        Err(LibError::OutOfBounds)
                    }
                }
                CrossSide(dir) => {
                    match one_direction_view(robot, world, dir.clone(), spyglass.bounds.distance) {
                        Ok(a) => {
                            // update the queue with the new discovered tiles
                            let robot_world = robot_map(world);
                            for (i, row) in robot_world.unwrap().iter().enumerate() {
                                for (j, tile) in row.iter().enumerate() {
                                    match tile {
                                        None => {}
                                        Some(_) => {
                                            spyglass.already_discovered.insert((i, j), true);
                                        }
                                    }
                                }
                            }

                            let origin = match dir {
                                Direction::Up => {
                                    (
                                        robot.get_coordinate().get_row().checked_sub(spyglass.bounds.distance).unwrap_or(0), // go up to DISTANCE cells or set to zero if out of bounds
                                        robot.get_coordinate().get_col().checked_sub(1).unwrap_or(0) // go left to 1 cell or set to zero if out of bounds
                                    )
                                }
                                Direction::Down => {
                                    (
                                        min(robot.get_coordinate().get_row()+1, spyglass.bounds.max_dim), // go down to DISTANCE cells or set to max_dim if out of bounds
                                        robot.get_coordinate().get_col().checked_sub(1).unwrap_or(0) // go left to 1 cell or set to zero if out of bounds
                                    )
                                }
                                Direction::Left => {
                                    (
                                        robot.get_coordinate().get_row().checked_sub(1).unwrap_or(0), // go up to 1 cell or set to zero if out of bounds
                                        robot.get_coordinate().get_col().checked_sub(spyglass.bounds.distance).unwrap_or(0) // go left to DISTANCE cells or set to 0 if out of bounds
                                    )
                                }
                                Direction::Right => {
                                    (
                                        robot.get_coordinate().get_row().checked_sub(1).unwrap_or(0), // go up to 1 cell or set to zero if out of bounds
                                        min(robot.get_coordinate().get_col()+1, spyglass.bounds.max_dim) // go right to DISTANCE cells or set to max_dim if out of bounds
                                    )
                                }
                            };
                            Ok((a, origin.0, origin.1))
                        }
                        Err(e) => { Err(e) }
                    }
                }
                Square(bounds) => {
                    let mut res = Vec::new();

                    if world.get_discoverable() > 0 {
                        if robot_map(world).unwrap()[bounds.center_row][bounds.center_col].is_none() {
                            match discover_tiles(robot, world, &[(bounds.center_row, bounds.center_col)]) {
                                Ok(r) => {
                                    res = vec![vec![r.get(&(bounds.center_row, bounds.center_col)).and_then(|t| t.clone()).and_then(|tile| Some(tile.clone())).unwrap(); 1]; 1];
                                }
                                Err(e) => {
                                    return Err(e);
                                }
                            }
                        }
                        let (top_left, top_right, bottom_left, bottom_right) = bounds.split();

                        spyglass.push_operation_if_valid(top_left);
                        spyglass.push_operation_if_valid(top_right);
                        spyglass.push_operation_if_valid(bottom_left);
                        spyglass.push_operation_if_valid(bottom_right);
                    }

                    Ok((res, bounds.center_row, bounds.center_col))
                }
            }
        }
    }

    /// The spyglass is a smart tool that allows robots to have an approximated view of the area around them optimizing the energy consumption. It start making a cross from the robot position and then it discover tiles in the most homogeneous way possible, increasing the detail more and more if the energy budget allows it.
    ///
    /// Use cases:
    /// - The spyglass can be used in a lot of different contexts, some of the major use cases are:
    /// - Simulation beginning (empty map): allows the robot to know immediately what is there around it but it is quite expensive in terms of energy because no optimization can be made.
    /// - Partially discovered world: allows the robot to discover as many tiles as possible with a certain amount of energy, focusing on less discovered directions.
    /// - Specific Tile/Content search: allows the robot to search a certain Tile or Content or a combination of them without going there directly. The tool stops to discover new tiles once the target has been found in order to save energy.
    ///
    /// This tool uses the DiscoverTiles interface so it is possible to discover only the 30% of the world using it. It is still possible to use the tool only with the initial cross after the 30% limit is reached.
    pub struct Spyglass {
        view_threshold: f64,
        bounds: SquareBound,
        energy_budget: Option<usize>,
        energy_consumed: usize,
        enable_cross: bool,
        stops_when: fn (&Tile) -> bool,
        operation_queue: VecDeque<Operation>,
        already_discovered: HashMap<(usize, usize), bool>,
        can_resume: bool
    }

    impl Tools for Spyglass {}

    impl Spyglass {
        /// The "new_default" function allows you to create a new instance of Spyglass by setting only the basic parameters
        ///
        /// Code example:
        /// ```
        /// let robot_row = my_robot.get_coordinate().get_row();
        /// let robot_col = my_robot.get_coordinate().get_col();
        ///
        /// let mut spyglass = Spyglass::new_default(
        ///     robot_row,
        ///     robot_col,
        ///     10, // distance
        ///     100 // world_dim
        /// );
        /// ```
        pub fn new_default (center_row: usize, center_column: usize, distance: usize, world_dim: usize) -> Self {
            Spyglass::new(
                center_row,
                center_column,
                distance,
                world_dim,
                 None,
                true,
                0.5,
                |_| false
            )
        }

        /// The "new" function allows you to create a new instance of Spyglass by setting all the parameters. Instead of this function you can use SpyglassBuilder for cleaner code.
        ///
        /// The arguments to instantiate a Spyglass are:
        /// - `center_row: usize`: the coordinate of the row of the tile from which the discovery must start. It must be set to the robot row coordinate for the tool to work properly.
        /// - `center_col: usize`: the coordinate of the column of the tile from which the discovery is to start. It must be set to the coordinate of the robot row for the tool to work properly.
        /// - `distance: usize`: the distance from the center within which the discovery is to take place. For example, if it is set to `10`, discovery will occur in a square of `21x21` around the center.
        /// - `energy_budget: Option<usize>`: with this argument you can tell the tool the maximum amount of energy it can use. When the budget is set to None, the scope will proceed with discovery until it visits all the tiles in the area or until the robot's energy runs out. When the budget is set to some value, the tool will proceed with discovery until the budget is reached.
        /// - `enable_cross: bool`: with this argument you can tell the spyglass whether to discover the cross at the beginning or not.
        /// - `view_threshold: f64`: a value between `0` and `1` that is used to determine whether or not to discover a certain area. Let `tot_tiles` be the number of cells in the area and `discovered_tiles` the number of tiles already discovered, the tool calculates the discovery rate as `discovered_tiles / tot_tiles` and compares it with the threshold. If the discovery rate is less than the threshold, the tool proceeds to view that area. More precisely, this procedure is used to decide whether or not to reveal each edge of the cross and each remaining sub-square.
        /// - `stops_when: fn(_: &Tile) -> bool`: this argument is used to provide the instrument with a stop condition. Instead of proceeding with discovery until it runs out of power, it is possible to stop when a condition is met. The stop condition is a closure that takes a `&Tile` as an argument and returns a boolean.
        ///
        ///    The following code is an example of a condition that tells the scope to stop at the first grass tile.
        ///    ```
        ///    |t| t.tile_type == Grass
        ///    ```
        ///    You can also tell the tool to never stop by passing a function that always returns false:
        ///     ```
        ///     |_| false
        ///     ```
        ///
        /// Here there is a complete example for the new function call:
        /// ```
        /// let mut spyglass = Spyglass::new(
        ///     robot_row, // center_row
        ///     robot_col, // center_col
        ///     10, // distance
        ///     100, // world_dim
        ///     Some(500), // energy_budget
        ///     true, // enable_cross
        ///     0.5, // view_threshold
        ///     |t| t.tile_type == Grass // stops_when
        /// );
        /// ```
        pub fn new(center_row: usize, center_column: usize, distance: usize, world_dim: usize, energy_budget: Option<usize>, enable_cross: bool, view_threshold: f64, stops_when: fn(&Tile) -> bool) -> Self {
            Self {
                view_threshold,
                bounds: SquareBound::new(center_row, center_column, distance, 0, world_dim-1),
                energy_budget,
                energy_consumed: 0,
                enable_cross,
                stops_when,
                operation_queue: VecDeque::new(),
                already_discovered: HashMap::new(),
                can_resume: false
            }
        }

        /// With this method you can set the view_threshold: a value between `0` and `1` that is used to determine whether or not to discover a certain area. Let `tot_tiles` be the number of cells in the area and `discovered_tiles` the number of tiles already discovered, the tool calculates the discovery rate as `discovered_tiles / tot_tiles` and compares it with the threshold. If the discovery rate is less than the threshold, the tool proceeds to view that area. More precisely, this procedure is used to decide whether or not to reveal each edge of the cross and each remaining sub-square.
        pub fn set_view_threshold(&mut self, view_threshold: f64) {
            self.view_threshold = view_threshold;
        }

        /// With this method you can set the energy_budget: the maximum amount of energy the tool can use. When the budget is set to None, the scope will proceed with discovery until it visits all the tiles in the area or until the robot's energy runs out. When the budget is set to a certain value, the tool will proceed with discovery until the budget is reached.
        ///
        /// Setting the budget using this function does not affect the used energy tracking, the budget is updated but the discover start counting with the old used energy amount. Use this function if you want to change the budget before calling resume_discover.
        pub fn set_energy_budget(&mut self, energy_budget: Option<usize>) {
            self.energy_budget = energy_budget;
        }

        /// With this method you can set the energy_budget and reset the consumed energy count. Use this method if you want to change the budget before starting a new discover.
        pub fn reset_energy_budget(&mut self, energy_budget: Option<usize>) {
            self.energy_consumed = 0;
            self.energy_budget = energy_budget;
        }

        /// With this method you can obtain the amount of energy the tool is still allowed to use (`energy_budget-used_energy`). The return value is None if the energy_budget is also None, otherwise it is the energy budget minus the energy used in previous discoveries.
        pub fn get_remaining_energy(&self) -> Option<usize> {
            self.energy_budget.and_then(|x| Some(x.checked_sub(self.energy_consumed).unwrap_or(0)))
        }

        /// With this method you can tell the spyglass whether to discover the cross at the beginning or not.
        pub fn set_cross_enabled(&mut self, val: bool) {
            self.enable_cross = val;
        }

        /// With this method you can provide the spyglass a stop condition. Instead of proceeding with discovery until the robot runs out of energy, it is possible to stop when a condition is met. The stop condition is a closure that takes a `&Tile` as an argument and returns a boolean.
        ///
        /// The following code is an example of a condition that tells the scope to stop at the first grass tile.
        /// ```
        /// |t| t.tile_type == Grass
        /// ```
        /// You can also tell the tool to never stop by passing a function that always returns false:
        /// ```
        /// |_| false
        /// ```
        pub fn set_stop_when(&mut self, condition: fn (&Tile) -> bool) {
            self.stops_when = condition
        }

        /// A value to know whether it is possible or not to resume a previous discover. A discover can be resumed only when paused (previous result was Paused) for missing energy: either the robot energy has run out or the budget has been reached). It is not possible to resume a discover that is in state Complete, Stopped or Failed.
        pub fn can_resume(&self) -> bool {
            self.can_resume
        }

        /// With this method you can start a new discover operation. The possible result values
        pub fn new_discover (&mut self, robot: &mut impl Runnable, world: &mut World) -> SpyglassResult {
            if self.operation_queue.is_empty() {
                self.already_discovered.clear();

                //self.operation_queue.push_back(Center);

                if self.is_one_direction_view_convenient(robot, world, Direction::Up) {
                    self.operation_queue.push_back(CrossSide(Direction::Up));
                }
                if self.is_one_direction_view_convenient(robot, world, Direction::Down) {
                    self.operation_queue.push_back(CrossSide(Direction::Down));
                }
                if self.is_one_direction_view_convenient(robot, world, Direction::Right) {
                    self.operation_queue.push_back(CrossSide(Direction::Right));
                }
                if self.is_one_direction_view_convenient(robot, world, Direction::Left) {
                    self.operation_queue.push_back(CrossSide(Direction::Left));
                }

                let (top_left, top_right, bottom_left, bottom_right) = self.bounds.split();

                let mut tmp = Vec::new();

                if self.is_square_view_convenient(world, &top_left) {
                    tmp.push(top_left);
                }
                if self.is_square_view_convenient(world, &top_right) {
                    tmp.push(top_right);
                }
                if self.is_square_view_convenient(world, &bottom_left) {
                    tmp.push(bottom_left);
                }
                if self.is_square_view_convenient(world, &bottom_right) {
                    tmp.push(bottom_right);
                }

                let map = robot_map(world).unwrap();
                tmp.sort_by(|a, b| {
                    let (top, left) = a.get_top_left_bound();
                    let (bottom, right) = a.get_bottom_right_bound();

                    let mut a_count = 0;
                    for i in top..bottom {
                        for j in left..right {
                            if map[i][j].is_some() {
                                a_count += 1;
                            }
                        }
                    }

                    let (top, left) = b.get_top_left_bound();
                    let (bottom, right) = b.get_bottom_right_bound();

                    let mut b_count = 0;
                    for i in top..bottom {
                        for j in left..right {
                            if map[i][j].is_some() {
                                b_count += 1;
                            }
                        }
                    }

                    if a_count >= b_count {
                        Ordering::Greater
                    } else {
                        Ordering::Less
                    }
                });

                for s in tmp {
                    self.push_operation_if_valid(s);
                }

                self.discover(robot, world)
            }
            else {
                SpyglassResult::Failed(OperationNotAllowed)
            }

        }

        /// With this method you can resumed a Paused discover operation.
        pub fn resume_discover(&mut self, robot: &mut impl Runnable, world: &mut World) -> SpyglassResult {
            self.discover(robot, world)
        }

        fn discover (&mut self, robot: &mut impl Runnable, world: &mut World) -> SpyglassResult {
            let mut n_discover = 0;
            self.can_resume = false;
            while !self.operation_queue.is_empty() { // go until the queue is not empty and the budget (if set) is not zero
                let op = self.operation_queue.pop_front().unwrap(); // pop from the queue the next operation

                let energy = op.get_cost(self, robot, world); // calculates the energy cost of the operation (if the tile is already visited the cost is 0)

                let remaining_energy = self.get_remaining_energy(); // returns the energy that is still available according to the budget (None means that the budget is not set)

                if remaining_energy.is_none() || remaining_energy.unwrap() >= energy { // if the budget is not set or there is enough energy
                    self.use_energy(energy); // use the energy
                    match op.execute(self, robot, world) { // execute the operation
                        Ok((result, row, col)) => { // if the execution is successful
                            // use the stop condition to tell the function to stop if should
                            let mut res = Vec::new();
                            for (i, r) in result.iter().enumerate() {
                                for (j, tile) in r.iter().enumerate() {
                                    if (self.stops_when)(tile) {
                                        res.push((tile.clone(), row + i, col + j))
                                    }
                                }
                            }

                            n_discover += result.len()*(result.get(0).map(|vec| vec.len()).unwrap_or(0));

                            if !res.is_empty() {
                                self.can_resume;
                                return Stopped(res);
                            }
                        }
                        Err(e) => {
                            return match e {
                                LibError::NotEnoughEnergy => { // if failed due to missing energy pause the discover and return
                                    self.operation_queue.push_back(op); // push the failed again at the back of the queue
                                    self.can_resume = true;
                                    SpyglassResult::Paused // return paused
                                }
                                _ => {
                                    SpyglassResult::Failed(e) // if failed due to a generic error return Failed
                                }
                            };
                        }
                    }
                }
            }

            return Complete(n_discover);
        }

        fn push_operation_if_valid(&mut self, new_bound: SquareBound) {
            let discovered = match self.already_discovered.get(&(new_bound.center_row, new_bound.center_col)) {
                None => { false }
                Some(v) => { *v }
            };

            if self.bounds.is_inside(new_bound.center_row, new_bound.center_col) && !discovered {
                self.already_discovered.insert((new_bound.center_row, new_bound.center_col), true);
                self.operation_queue.push_back(Square(new_bound));
            }
        }

        fn use_energy(&mut self, energy: usize) {
            self.energy_consumed += energy;
        }

        fn is_one_direction_view_convenient(&self, robot: &impl Runnable, world: &World, dir: Direction) -> bool {
            let (robot_r, robot_c) = (robot.get_coordinate().get_row(), robot.get_coordinate().get_col());

            let (row_from, row_to, col_from, col_to) = match dir {
                Direction::Up => {
                    (
                        robot_r.checked_sub(1 + self.bounds.distance).unwrap_or(0),
                        robot_r.checked_sub(1).unwrap_or(0),
                        robot_c.checked_sub(1).unwrap_or(0),
                        min(robot_c + 1, self.bounds.max_dim)
                    )
                }
                Direction::Down => {
                    (
                        min(robot_r + 1,  self.bounds.max_dim),
                        min(robot_r + 1 +self.bounds.distance,  self.bounds.max_dim),
                        robot_c.checked_sub(1).unwrap_or(0),
                        min(robot_c + 1,  self.bounds.max_dim),)
                }
                Direction::Left => {
                    (
                        robot_r.checked_sub(1).unwrap_or(0),
                        min(robot_r + 1,  self.bounds.max_dim),
                        robot_c.checked_sub(1 + self.bounds.distance).unwrap_or(0),
                        robot_c.checked_sub(1).unwrap_or(0),
                    )
                }
                Direction::Right => {
                    (
                        robot_r.checked_sub(1).unwrap_or(0),
                        min(robot_r + 1,  self.bounds.max_dim),
                        min(robot_c + 1,  self.bounds.max_dim),
                        min(robot_c + 1 + self.bounds.distance,  self.bounds.max_dim),
                    )
                }
            };

            self.is_view_convenient(world, row_from, row_to, col_from, col_to)
        }

        fn is_square_view_convenient(&self, world: &World, square: &SquareBound) -> bool {
            let (row_from, col_from) = square.get_top_left_bound();
            let (row_to, col_to) = square.get_bottom_right_bound();

            self.is_view_convenient(world, row_from, row_to, col_from, col_to)
        }

        fn is_view_convenient(&self, world: &World, row_from: usize, row_to: usize, col_from: usize, col_to: usize) -> bool {
            let to_discover_count = (row_to - row_from) * (col_to - col_from);
            let mut already_discovered_count: usize = 0;

            let map = robot_map(world).unwrap();
            for i in row_from..row_to {
                for j in col_from..col_to {
                    if map[i][j].is_some() {
                        already_discovered_count += 1;
                    }
                }
            }

            (already_discovered_count as f64) / (to_discover_count as f64) <= self.view_threshold
        }
    }

    pub struct SpyglassBuilder {
        spy_glass: Spyglass
    }

    impl SpyglassBuilder {
        pub fn new(center_row: usize, center_column: usize, distance: usize, world_dim: usize) -> Self {
            SpyglassBuilder {
                spy_glass: Spyglass::new_default(center_row, center_column, distance, world_dim)
            }
        }

        pub fn view_threshold(mut self, view_threshold: f64) -> Self {
            self.spy_glass.set_view_threshold(view_threshold);
            self
        }

        pub fn energy_budget(mut self, energy_budget: Option<usize>) -> Self {
            self.spy_glass.set_energy_budget(energy_budget);
            self
        }

        pub fn stops_when(mut self, function: fn (&Tile) -> bool) -> Self {
            self.spy_glass.set_stop_when(function);
            self
        }

        pub fn enable_cross(mut self) -> Self {
            self.spy_glass.set_cross_enabled(true);
            self
        }

        pub fn disable_cross(mut self) -> Self {
            self.spy_glass.set_cross_enabled(false);
            self
        }

        pub fn build(self) -> Spyglass {
            self.spy_glass
        }


    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use robotics_lib::energy::Energy;
    use robotics_lib::event::events::Event;
    use robotics_lib::interface::{debug, robot_map};
    use robotics_lib::runner::backpack::BackPack;
    use robotics_lib::runner::{Robot, Runnable, Runner};
    use robotics_lib::world::coordinates::Coordinate;
    use spyglass::SpyglassBuilder;
    use super::*;

    use robotics_lib::world::environmental_conditions::{EnvironmentalConditions, WeatherType};
    use robotics_lib::world::tile::{Content, Tile, TileType};
    use robotics_lib::world::tile::TileType::{DeepWater, Grass, Hill, Lava, Mountain, Sand, ShallowWater, Snow, Street, Teleport, Wall};
    use robotics_lib::world::World;
    use robotics_lib::world::world_generator::Generator;
    use crate::spyglass::SpyglassResult;

    fn display_tile (tile_type: TileType){
        print!("{}", match tile_type {
            TileType::DeepWater => { "D" }
            TileType::ShallowWater => { "W" }
            TileType::Sand => { "S" }
            TileType::Grass => { "G" }
            TileType::Street => { "I" }
            TileType::Hill => { "H" }
            TileType::Mountain => { "M" }
            TileType::Snow => { "️O" }
            TileType::Lava => { "L" }
            TileType::Teleport(_) => { "T" }
            TileType::Wall => { "N" }
        })
    }

    struct MyTestRobot {
        robot: Robot,
    }

    impl MyTestRobot {
        fn new(robot: Robot) -> Self {
            MyTestRobot {
                robot,
            }
        }
    }

    impl Runnable for MyTestRobot {
        fn process_tick(&mut self, world: &mut World) {

            if self.get_energy().get_energy_level() >= 1000 {
                let debug_world =  debug(self, world);

                let robot_row = self.get_coordinate().get_row();
                let robot_col = self.get_coordinate().get_col();

                let mut spyglass = SpyglassBuilder::new(
                    robot_row,
                    robot_col,
                    17,
                    debug_world.0.len()
                )
                    .energy_budget(Some(500))
                    .build();

                // if spyglass.can_resume() {
                //     spyglass.resume_discover(self, world);
                // }
                // else {
                    let result = spyglass.new_discover(self, world);
                // }
                match result {
                    SpyglassResult::Complete(discovered_tiles) => {
                        println!("Complete: {discovered_tiles}");
                    }
                    SpyglassResult::Stopped(_) => {}
                    SpyglassResult::Paused => {}
                    SpyglassResult::Failed(_) => {}
                }

                for (_i, row) in robot_map(world).unwrap().iter().enumerate() {
                    for (_j, tile) in row.iter().enumerate() {
                        match tile {
                            None => {
                                print!("-")
                            }
                            Some(t) => {
                                display_tile(t.tile_type)
                            }
                        }
                    }
                    println!();
                }
                println!();
            }
        }

        fn handle_event(&mut self, event: Event) {
            match event {
                Event::Ready => {}
                Event::Terminated => {}
                Event::TimeChanged(_) => {}
                Event::DayChanged(_) => {}
                Event::EnergyRecharged(_) => {}
                Event::EnergyConsumed(_) => {}
                Event::Moved(_, _) => {}
                Event::TileContentUpdated(_, _) => {}
                Event::AddedToBackpack(_, _) => {}
                Event::RemovedFromBackpack(_, _) => {}
            }
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

    struct TestWGen {
        dim: usize
    }

    // basic world generator that spawn tiles in a specific order
    impl TestWGen {
        fn new(dim: usize) -> Self {
            Self {
                dim
            }
        }
    }

    impl Generator for TestWGen {
        fn gen(&mut self) -> (Vec<Vec<Tile>>, (usize, usize), EnvironmentalConditions, f32, Option<HashMap<Content,f32>>) {
            let tile_types = vec![
                DeepWater,
                ShallowWater,
                Sand,
                Grass,
                Street,
                Hill,
                Mountain,
                Snow,
                Lava,
                Teleport(false),
                Wall
            ];

            let mut world : Vec<Vec<Tile>> = Vec::new();
            for i in 0..self.dim {
                let mut row = Vec::new();
                for j in 0..self.dim {
                    row.push(Tile {
                        tile_type: tile_types[(i*self.dim+j) % tile_types.len()],
                        content: Content::None,
                        elevation: 10
                    });
                }
                world.push(row);
            }

            (
                world,
                (self.dim/2, self.dim/2),
                EnvironmentalConditions::new(&[WeatherType::Sunny], 0, 0).unwrap(),
                10.0,
                None
            )
        }
    }

    #[test]
    fn it_works() {
        const DIMENSION: usize = 50;

        let robot = MyTestRobot::new(Robot::new());

        let mut gen = TestWGen::new(DIMENSION);

        let mut r = Runner::new(Box::new(robot), &mut gen).unwrap();
        while r.get_robot().get_energy().get_energy_level() < 1000 {
            r.game_tick().unwrap();
        }
        r.game_tick().unwrap();
        while r.get_robot().get_energy().get_energy_level() < 1000 {
            r.game_tick().unwrap();
        }
        r.game_tick().unwrap();
        while r.get_robot().get_energy().get_energy_level() < 1000 {
            r.game_tick().unwrap();
        }
        r.game_tick().unwrap();
        while r.get_robot().get_energy().get_energy_level() < 1000 {
            r.game_tick().unwrap();
        }
        r.game_tick().unwrap();
        while r.get_robot().get_energy().get_energy_level() < 1000 {
            r.game_tick().unwrap();
        }
        r.game_tick().unwrap();
        while r.get_robot().get_energy().get_energy_level() < 1000 {
            r.game_tick().unwrap();
        }
        r.game_tick().unwrap();
    }
}
