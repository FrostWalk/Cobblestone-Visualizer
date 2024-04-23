use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use robotics_lib::interface::{Direction, Tools};
use robotics_lib::runner::Runnable;
use robotics_lib::world::tile::{Content, TileType};
use robotics_lib::world::World;
use robotics_lib::interface::where_am_i;
use robotics_lib::interface::robot_view;
use robotics_lib::interface::robot_map;
use robotics_lib::interface::go;
use robotics_lib::world::tile::Tile;
use robotics_lib::utils::LibError;
use robotics_lib::runner::{Robot};
use robotics_lib::event::events::Event;
use robotics_lib::energy::Energy;
use robotics_lib::world::coordinates::Coordinate;
use robotics_lib::runner::backpack::BackPack;
use robotics_lib::world::world_generator::Generator;
use robotics_lib::world::environmental_conditions::EnvironmentalConditions;
use robotics_lib::world::environmental_conditions::WeatherType;
use robotics_lib::runner::Runner;
use rand::Rng;

pub struct StreetExplorer; //nome del tool

impl Tools for StreetExplorer {} //il tool deve implementare il trait Tools

impl<'a> StreetExplorer{ //definisco le funzionalità del tool

    fn handle_error_for_explorer(res: Result<(Vec<Vec<Option<Tile>>>, (usize, usize)), LibError>) -> Result<String, LibError>{
        match res {
            Err(e) => {
                match e {
                    LibError::NotEnoughEnergy => { Err(e) },
                    _ => { Ok("Altro errore da ignorare".to_string()) },
                }
            },
            Ok(res) => {
                Ok("Non sono un errore quindi posso essere ignorato".to_string())
            }
        }
    }

    fn am_i_near_the_content_i_am_looking_for(robot_view: &Vec<Vec<Option<Tile>>>, content: &Option<Content>) -> bool{
        if let Some(content) = content{
            for i in 0..robot_view.len(){
                for j in 0..robot_view.len(){
                    if let Some(tile) = &robot_view[i][j]{
                        if tile.content == *content && ((i == 0 && j == 1) || (i == 1 && j == 0) || (i == 1 && j == 2) || (i == 2 && j == 1) ){
                            return true;
                        }
                    }
                }
            }
            false
        }
        else {
            false
        }
    }

    fn move_down(robot: &'a mut impl Runnable, world: &'a mut World, robot_view: &Vec<Vec<Option<Tile>>>, visited: &mut Vec<Vec<bool>>, content: &Option<Content>, direction: &Option<Direction>, i: usize, j: usize) -> Result<String, LibError>{
        if let Some(tile) = &robot_view[2][1] { //DOWN
            if tile.tile_type == TileType::Street && visited[i + 1][j] == false {
                visited[i + 1][j] = true;
                Self::handle_error_for_explorer(go(robot, world, Direction::Down))?;
                let res = Self::recursive_explore(robot, world, visited, content, direction)?;
                if res == "FOUND_CONTENT"{
                    return Ok(res);
                }
                Self::handle_error_for_explorer(go(robot, world, Direction::Up))?;
            }
            Ok("VISITED_COMPLETED".to_string())
        }
        else {
            Ok("VISITED_COMPLETED".to_string())
        }
    }

    fn move_up(robot: &'a mut impl Runnable, world: &'a mut World, robot_view: &Vec<Vec<Option<Tile>>>, visited: &mut Vec<Vec<bool>>, content: &Option<Content>, direction: &Option<Direction>, i: usize, j: usize) -> Result<String, LibError>{
        if let Some(tile) = &robot_view[0][1] { //UP
            if tile.tile_type == TileType::Street && i > 0 && visited[i - 1][j] == false {
                visited[i - 1][j] = true;
                Self::handle_error_for_explorer(go(robot, world, Direction::Up))?;
                let res = Self::recursive_explore(robot, world, visited, content, direction)?;
                if res == "FOUND_CONTENT"{
                    return Ok(res);
                }
                Self::handle_error_for_explorer(go(robot, world, Direction::Down))?;
            }
            Ok("VISITED_COMPLETED".to_string())
        }
        else {
            Ok("VISITED_COMPLETED".to_string())
        }
    }

    fn move_left(robot: &'a mut impl Runnable, world: &'a mut World, robot_view: &Vec<Vec<Option<Tile>>>, visited: &mut Vec<Vec<bool>>, content: &Option<Content>, direction: &Option<Direction>, i: usize, j: usize) -> Result<String, LibError>{
        if let Some(tile) = &robot_view[1][0] { //LEFT
            if tile.tile_type == TileType::Street && j > 0 && visited[i][j - 1] == false {
                visited[i][j - 1] = true;
                Self::handle_error_for_explorer(go(robot, world, Direction::Left))?;
                let res = Self::recursive_explore(robot, world, visited, content, direction)?;
                if res == "FOUND_CONTENT"{
                    return Ok(res);
                }
                Self::handle_error_for_explorer(go(robot, world, Direction::Right))?;
            }
            Ok("VISITED_COMPLETED".to_string())
        }
        else{
            Ok("VISITED_COMPLETED".to_string())
        }
    }

    fn move_right(robot: &'a mut impl Runnable, world: &'a mut World, robot_view: &Vec<Vec<Option<Tile>>>, visited: &mut Vec<Vec<bool>>, content: &Option<Content>, direction: &Option<Direction>, i: usize, j: usize) -> Result<String, LibError>{
        if let Some(tile) = &robot_view[1][2] { //RIGHT
            if tile.tile_type == TileType::Street && visited[i][j + 1] == false {
                visited[i][j + 1] = true;
                Self::handle_error_for_explorer(go(robot, world, Direction::Right))?;
                let res = Self::recursive_explore(robot, world, visited, content, direction)?;
                if res == "FOUND_CONTENT"{
                    return Ok(res);
                }
                Self::handle_error_for_explorer(go(robot, world, Direction::Left))?;
            }
            Ok("VISITED_COMPLETED".to_string())
        }
        else {
            Ok("VISITED_COMPLETED".to_string())
        }
    }

    fn recursive_explore(robot: &'a mut impl Runnable, world: &'a mut World, visited: &mut Vec<Vec<bool>>, content: &Option<Content>, direction: &Option<Direction>) -> Result<String, LibError>{
        let (robot_view, robot_position) = where_am_i(robot, world);
        let (i, j) = (robot_position.0, robot_position.1);

        if !Self::am_i_near_the_content_i_am_looking_for(&robot_view, content) {
            //faccio un match di quelli belli sulla direzione per impostare le diverse priorità
            match &direction{
                Some(d) => {
                    match d{
                        Direction::Left => {
                            let left_res = Self::move_left(robot, world, &robot_view, visited, content, direction, i, j)?;
                            if left_res == "FOUND_CONTENT"{
                                return Ok(left_res);
                            }
                            let right_res = Self::move_right(robot, world, &robot_view, visited, content, direction, i, j)?;
                            if right_res == "FOUND_CONTENT"{
                                return Ok(right_res);
                            }
                            let up_res = Self::move_up(robot, world, &robot_view, visited, content, direction, i, j)?;
                            if up_res == "FOUND_CONTENT"{
                                return Ok(up_res);
                            }
                            let down_res = Self::move_down(robot, world, &robot_view, visited, content, direction, i, j)?;
                            if down_res == "FOUND_CONTENT"{
                                return Ok(down_res);
                            }
                        },
                        Direction::Right => {
                            let right_res = Self::move_right(robot, world, &robot_view, visited, content, direction, i, j)?;
                            if right_res == "FOUND_CONTENT"{
                                return Ok(right_res);
                            }
                            let left_res = Self::move_left(robot, world, &robot_view, visited, content, direction, i, j)?;
                            if left_res == "FOUND_CONTENT"{
                                return Ok(left_res);
                            }
                            let up_res = Self::move_up(robot, world, &robot_view, visited, content, direction, i, j)?;
                            if up_res == "FOUND_CONTENT"{
                                return Ok(up_res);
                            }
                            let down_res = Self::move_down(robot, world, &robot_view, visited, content, direction, i, j)?;
                            if down_res == "FOUND_CONTENT"{
                                return Ok(down_res);
                            }
                        },
                        Direction::Up => {
                            let up_res = Self::move_up(robot, world, &robot_view, visited, content, direction, i, j)?;
                            if up_res == "FOUND_CONTENT"{
                                return Ok(up_res);
                            }
                            let left_res = Self::move_left(robot, world, &robot_view, visited, content, direction, i, j)?;
                            if left_res == "FOUND_CONTENT"{
                                return Ok(left_res);
                            }
                            let right_res = Self::move_right(robot, world, &robot_view, visited, content, direction, i, j)?;
                            if right_res == "FOUND_CONTENT"{
                                return Ok(right_res);
                            }
                            let down_res = Self::move_down(robot, world, &robot_view, visited, content, direction, i, j)?;
                            if down_res == "FOUND_CONTENT"{
                                return Ok(down_res);
                            }
                        },
                        Direction::Down => {
                            let down_res = Self::move_down(robot, world, &robot_view, visited, content, direction, i, j)?;
                            if down_res == "FOUND_CONTENT"{
                                return Ok(down_res);
                            }
                            let left_res = Self::move_left(robot, world, &robot_view, visited, content, direction, i, j)?;
                            if left_res == "FOUND_CONTENT"{
                                return Ok(left_res);
                            }
                            let right_res = Self::move_right(robot, world, &robot_view, visited, content, direction, i, j)?;
                            if right_res == "FOUND_CONTENT"{
                                return Ok(right_res);
                            }
                            let up_res = Self::move_up(robot, world, &robot_view, visited, content, direction, i, j)?;
                            if up_res == "FOUND_CONTENT"{
                                return Ok(up_res);
                            }
                        },
                    }
                },
                None => {
                    let left_res = Self::move_left(robot, world, &robot_view, visited, content, direction, i, j)?;
                    if left_res == "FOUND_CONTENT"{
                        return Ok(left_res);
                    }
                    let right_res = Self::move_right(robot, world, &robot_view, visited, content, direction, i, j)?;
                    if right_res == "FOUND_CONTENT"{
                        return Ok(right_res);
                    }
                    let up_res = Self::move_up(robot, world, &robot_view, visited, content, direction, i, j)?;
                    if up_res == "FOUND_CONTENT"{
                        return Ok(up_res);
                    }
                    let down_res = Self::move_down(robot, world, &robot_view, visited, content, direction, i, j)?;
                    if down_res == "FOUND_CONTENT"{
                        return Ok(down_res);
                    }
                }
            }
            Ok("VISITED_COMPLETED".to_string())
        }
        else{
            Ok("FOUND_CONTENT".to_string())
        }
    }

    ///# StreetExplorer
    ///Walks trough streets until all energy is consumed or all streets are explored or desired objective is met.
    ///You can pass a content to look for or a preferred direction to freely explore.

    ///# Input
    /// - `&'a mut impl Runnable`: The robot.
    /// - `&'a mut World`: The world.
    /// - `Option<Content>`: An optional content to search.
    /// - `Option<Direction>`: An optional direction in which to prioritize to move.

    ///# Output
    ///The output consists of a data structure of type `Result<String, LibError>`, where the possible outputs are respectively:
    /// - `Ok("VISITED_COMPLETED".to_string())`: The robot explored all the avenues it could explore, and in the case it needed to find content, it failed to find it.
    /// - `Ok("FOUND_CONTENT".to_string())`: The robot found the desired content and stopped near it.
    /// - `LibError::NotEnoughEnergy`: The robot did not have enough energy to complete the task.
    /// - `LibError::OperationNotAllowed`: It is not possible for the robot to use this tool because it is not physically on a road.

    ///# Usage
    /// 1) No content to search and no direction specified.
    /// ```
    /// fn process_tick(&mut self, world: &mut World) {
    ///     let res = StreetExplorer::explore_street(self, world, None, None);
    ///     match res{
    ///         Ok(s) => {
    ///             println!("success: {}", s);
    ///         },
    ///         Err(err) => {
    ///             println!("error: {:?}", err);
    ///         }
    ///     }
    /// }
    /// ```
    ///
    /// 2) Specified only the content.
    /// ```
    /// fn process_tick(&mut self, world: &mut World) {
    ///     let res = StreetExplorer::explore_street(self, world, Some(Content::Coin(1)), None);
    ///     match res{
    ///         Ok(s) => {
    ///             println!("success: {}", s);
    ///         },
    ///         Err(err) => {
    ///             println!("error: {:?}", err);
    ///         }
    ///     }
    /// }
    /// ```
    ///
    /// 3) Specified only the direction.
    /// ```
    /// fn process_tick(&mut self, world: &mut World) {
    ///     let res = StreetExplorer::explore_street(self, world, None, Some(Direction::UP));
    ///     match res{
    ///         Ok(s) => {
    ///             println!("success: {}", s);
    ///         },
    ///         Err(err) => {
    ///             println!("error: {:?}", err);
    ///         }
    ///     }
    /// }
    /// ```
    ///
    /// 4) Specified both content and direction.
    /// ```
    /// fn process_tick(&mut self, world: &mut World) {
    ///     let res = StreetExplorer::explore_street(self, world, Some(Content::Coin(1)), Some(Direction::UP));
    ///     match res{
    ///         Ok(s) => {
    ///             println!("success: {}", s);
    ///         },
    ///         Err(err) => {
    ///             println!("error: {:?}", err);
    ///         }
    ///     }
    /// }
    /// ```

    pub fn explore_street(robot: &'a mut impl Runnable, world: &'a mut World, content: Option<Content>, direction: Option<Direction>) -> Result<String, LibError>{
        robot_view(robot, world);

        let robot_map = robot_map(world);
        let robot_coordinates = robot.get_coordinate();
        let (i, j) = (robot_coordinates.get_row(), robot_coordinates.get_col());

        //se il robot non si trova su una strada non puoi usare il tool
        match &robot_map{
            None => {
                return Err(LibError::OperationNotAllowed);
            },
            Some(robot_map) => {
                match &robot_map[i][j] {
                    None => { return Err(LibError::OperationNotAllowed); },
                    Some(tile) => {
                        if tile.tile_type != TileType::Street{
                            return Err(LibError::OperationNotAllowed);
                        }
                    }
                }
            }
        }

        let size = robot_map.unwrap().len();
        let mut visited = vec![vec![false; size]; size]; //matrice che mi serve per non visitare la stessa strada due volte
        visited[i][j] = true;

        //esploro ricorsivamente la rete di strade alla ricerca del content desiderato e dando priorità alla direzione specificata.
        //nel caso non venga specificata alcuna direzione il robot, da di default priorità prima alle direzioni left, right, up, down
        Self::recursive_explore(robot, world, &mut visited, &content, &direction)

    }
}


