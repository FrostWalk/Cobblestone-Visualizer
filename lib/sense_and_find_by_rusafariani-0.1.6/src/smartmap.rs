use priority_queue::PriorityQueue;
use robotics_lib::utils::LibError;
use robotics_lib::world::tile::Content::*;
use robotics_lib::world::tile::TileType::*;
use robotics_lib::world::tile::{Content, Tile, TileType};
use std::cmp::{Ordering, Reverse};
use std::collections::{HashMap, VecDeque};

/// An enum that indicate the actions that must be taken go in a certain direction
///
/// Cardinal directions have been chosen so that it doesn't create a conflict with
/// the Common Crate Direction enum.
///
/// # Variants
/// - `North` => Common Crate's `Up`
/// - `South` => Common Crate's `Down`
/// - `West` => Common Crate's `Left`
/// - `East` => Common Crate's `Right`
/// - `Teleport(x: usize, y: usize)` => Teleport to the (x,y) coordinates
#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub enum Action {
    North,
    South,
    West,
    East,
    Teleport(usize, usize),
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
struct Pair {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone)]
struct Elems {
    value: Reverse<usize>,
    dir: Action,
}

impl Elems {
    fn new(value: Reverse<usize>, dir: Action) -> Elems {
        Elems { value, dir }
    }
}

impl Eq for Elems {}

impl PartialEq<Self> for Elems {
    fn eq(&self, other: &Self) -> bool {
        self.value.eq(&other.value)
    }
}

impl PartialOrd<Self> for Elems {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl Ord for Elems {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
    }
}

impl Pair {
    fn new(x: usize, y: usize) -> Pair {
        Pair { x, y }
    }
}

pub(crate) struct SmartMap {
    map: Vec<Vec<Option<Tile>>>,
    cost: HashMap<Pair, (usize, Action)>,
    content: HashMap<Content, Vec<Pair>>,
    tiletype: HashMap<TileType, Vec<Pair>>,
    start: Pair,
    teleports: Vec<(Pair, bool)>,
}

impl SmartMap {
    pub fn new() -> SmartMap {
        SmartMap {
            map: vec![],
            cost: HashMap::new(),
            content: HashMap::new(),
            tiletype: HashMap::new(),
            start: Pair::new(0, 0),
            teleports: vec![],
        }
    }

    pub(crate) fn update_map(&mut self, map: &Vec<Vec<Option<Tile>>>) {
        self.map = map.clone();
        self.teleports = SmartMap::search_teleports(&self.map);
    }

    fn inner_update_cost(
        &mut self,
        row: usize,
        col: usize,
        percentage: usize,
        max_cost: usize,
    ) -> Result<(), LibError> {
        self.check_borders(row, col)?;
        let total_tiles =
            ((self.map.len() * self.map[row].len()) as f64 * (percentage as f64 / 100.0)) as usize;
        let mut pq = PriorityQueue::new();
        self.reset();
        self.start = Pair::new(row, col);
        // dummy teleport, it doesn't mean anything
        pq.push_decrease(
            Pair::new(row, col),
            Elems::new(Reverse(0usize), Action::Teleport(row, col)),
        );

        while !pq.is_empty() {
            let temp = pq.pop().unwrap();
            let (xc, yc, cost, act) = (temp.0.x, temp.0.y, temp.1.value.0, temp.1.dir.clone());
            self.cost.insert(Pair::new(xc, yc), (cost, act.clone()));
            if cost > max_cost || self.cost.len() > total_tiles {
                break;
            }
            let opt = self.map[xc][yc].clone();

            if opt.is_some() {
                let tile = opt.unwrap();
                let pair = Pair::new(xc, yc);
                self.assign_tiletype(&tile.tile_type, pair.clone());
                self.assign_content(&tile.content, pair.clone());
                if !tile.tile_type.properties().walk() {
                    continue;
                }
                match tile.tile_type {
                    Teleport(_) => self.teleports.iter().for_each(|(p, b)| {
                        if (p.x != xc || p.y != yc) && self.cost.get(&p).is_none() && *b {
                            pq.push_increase(
                                p.clone(),
                                Elems::new(Reverse(cost + 30), Action::Teleport(xc, yc)),
                            );
                        }
                    }),
                    _ => {}
                }
                let elev = tile.elevation;
                if xc != 0 {
                    self.check(&mut pq, xc - 1, yc, cost, elev, Action::North);
                }
                self.check(&mut pq, xc + 1, yc, cost, elev, Action::South);
                if yc != 0 {
                    self.check(&mut pq, xc, yc - 1, cost, elev, Action::West);
                }
                self.check(&mut pq, xc, yc + 1, cost, elev, Action::East);
            }
        }

        Ok(())
    }

    pub(crate) fn update_cost(&mut self, row: usize, col: usize) -> Result<(), LibError> {
        self.inner_update_cost(row, col, 100, usize::MAX)
    }

    pub(crate) fn update_cost_constrained_cost(
        &mut self,
        row: usize,
        col: usize,
        max_cost: usize,
    ) -> Result<(), LibError> {
        self.inner_update_cost(row, col, 100, max_cost)
    }

    pub(crate) fn update_cost_constrained_percentage(
        &mut self,
        row: usize,
        col: usize,
        percentage: usize,
    ) -> Result<(), LibError> {
        if percentage > 100 {
            panic!("The percentage can't be greater than 100, inserted value {percentage}")
        }
        self.inner_update_cost(row, col, percentage, usize::MAX)
    }

    pub(crate) fn update_cost_constrained(
        &mut self,
        row: usize,
        col: usize,
        percentage: usize,
        max_cost: usize,
    ) -> Result<(), LibError> {
        if percentage > 100 {
            panic!("The percentage can't be greater than 100, inserted value {percentage}")
        }
        self.inner_update_cost(row, col, percentage, max_cost)
    }

    fn reset(&mut self) {
        self.cost = HashMap::new();
        self.tiletype = HashMap::from([
            (DeepWater, Vec::new()),
            (ShallowWater, Vec::new()),
            (Sand, Vec::new()),
            (Grass, Vec::new()),
            (Street, Vec::new()),
            (Hill, Vec::new()),
            (Mountain, Vec::new()),
            (Snow, Vec::new()),
            (Lava, Vec::new()),
            (Teleport(true), Vec::new()),
            (Teleport(false), Vec::new()),
            (Wall, Vec::new()),
        ]);
        self.content = HashMap::from([
            (Rock(0), Vec::new()),
            (Tree(0), Vec::new()),
            (Garbage(0), Vec::new()),
            (Fire, Vec::new()),
            (Coin(0), Vec::new()),
            (Bin(0..0), Vec::new()),
            (Crate(0..0), Vec::new()),
            (Bank(0..0), Vec::new()),
            (Water(0), Vec::new()),
            (Market(0), Vec::new()),
            (Fish(0), Vec::new()),
            (Building, Vec::new()),
            (Bush(0), Vec::new()),
            (JollyBlock(0), Vec::new()),
            (Scarecrow, Vec::new()),
            (None, Vec::new()),
        ]);
    }

    fn assign_tiletype(&mut self, tiletype: &TileType, pair: Pair) {
        let tt;
        match tiletype {
            DeepWater => tt = DeepWater,
            ShallowWater => tt = ShallowWater,
            Sand => tt = Sand,
            Grass => tt = Grass,
            Street => tt = Street,
            Hill => tt = Hill,
            Mountain => tt = Mountain,
            Snow => tt = Snow,
            Lava => tt = Lava,
            Teleport(true) => tt = Teleport(true),
            Teleport(false) => tt = Teleport(false),
            Wall => tt = Wall,
        }
        let a = self.tiletype.get_mut(&tt);
        if a.is_none() {
            return;
        }
        a.unwrap().push(pair)
    }
    fn assign_content(&mut self, content: &Content, pair: Pair) {
        let c;
        match content {
            Rock(_) => c = Rock(0),
            Tree(_) => c = Tree(0),
            Garbage(_) => c = Garbage(0),
            Fire => c = Fire,
            Coin(_) => c = Coin(0),
            Bin(_) => c = Bin(0..0),
            Crate(_) => c = Crate(0..0),
            Bank(_) => c = Bank(0..0),
            Water(_) => c = Water(0),
            Market(_) => c = Market(0),
            Fish(_) => c = Fish(0),
            Building => c = Building,
            Bush(_) => c = Bush(0),
            JollyBlock(_) => c = JollyBlock(0),
            Scarecrow => c = Scarecrow,
            None => c = None,
        }
        let a = self.content.get_mut(&c);
        if a.is_none() {
            return;
        }
        a.unwrap().push(pair);
    }

    fn search_teleports(map: &Vec<Vec<Option<Tile>>>) -> Vec<(Pair, bool)> {
        let mut v = Vec::new();
        for (x, i) in map.iter().enumerate() {
            i.iter().enumerate().for_each(|(y, a)| {
                let t = a.as_ref();
                if let Some(z) = t {
                    match z.tile_type {
                        Teleport(b) => v.push((Pair::new(x, y), b)),
                        _ => {}
                    }
                }
            })
        }
        v
    }

    fn check(
        &mut self,
        pq: &mut PriorityQueue<Pair, Elems>,
        xc: usize,
        yc: usize,
        cost: usize,
        el: usize,
        act: Action,
    ) {
        if self.check_borders(xc, yc).is_err() {
            return;
        }

        let maybe = self.map[xc][yc].as_ref();
        if maybe.is_none() {
            return;
        }

        let tile = maybe.unwrap();
        let paio = Pair::new(xc, yc);

        let min = if tile.tile_type.properties().walk() {
            let add = if tile.elevation > el {
                (tile.elevation - el).pow(2)
            } else {
                0
            };
            cost + tile.tile_type.properties().cost() + add
        } else {
            cost
        };
        if self.cost.get(&paio).is_none() {
            pq.push_increase(paio, Elems::new(Reverse(min), act));
        }
    }

    fn check_borders(&self, x: usize, y: usize) -> Result<(), LibError> {
        if x >= self.map.len() || y >= self.map[x].len() {
            Err(LibError::OutOfBounds)
        } else {
            Ok(())
        }
    }

    pub(crate) fn get_cost(&self, xc: usize, yc: usize) -> Option<usize> {
        if self.check_borders(xc, yc).is_err() {
            return Option::None;
        }
        self.cost.get(&Pair::new(xc, yc)).map(|(a, _)| *a)
    }

    pub(crate) fn get_cost_and_action(&self, xc: usize, yc: usize) -> Option<(usize, Action)> {
        if self.check_borders(xc, yc).is_err() {
            return Option::None;
        }
        self.cost
            .get(&Pair::new(xc, yc))
            .map(|(a, b)| (*a, b.clone()))
    }

    pub(crate) fn get_tiletype_vec(&self, tiletype: &TileType) -> Vec<(usize, usize)> {
        let a;
        match tiletype {
            DeepWater => a = self.tiletype.get(&DeepWater),
            ShallowWater => a = self.tiletype.get(&ShallowWater),
            Sand => a = self.tiletype.get(&Sand),
            Grass => a = self.tiletype.get(&Grass),
            Street => a = self.tiletype.get(&Street),
            Hill => a = self.tiletype.get(&Hill),
            Mountain => a = self.tiletype.get(&Mountain),
            Snow => a = self.tiletype.get(&Snow),
            Lava => a = self.tiletype.get(&Lava),
            Teleport(true) => a = self.tiletype.get(&Teleport(true)),
            Teleport(false) => a = self.tiletype.get(&Teleport(false)),
            Wall => a = self.tiletype.get(&Wall),
        };
        match a {
            Some(v) => v.iter().map(|p| (p.x, p.y)).collect(),
            Option::None => Vec::new()
        }
        //a.unwrap().iter().map(|p| (p.x, p.y)).collect()
    }

    pub(crate) fn get_content_vec(&self, content: &Content) -> Vec<(usize, usize)> {
        let a;
        match content {
            Rock(_) => a = self.content.get(&Rock(0)),
            Tree(_) => a = self.content.get(&Tree(0)),
            Garbage(_) => a = self.content.get(&Garbage(0)),
            Fire => a = self.content.get(&Fire),
            Coin(_) => a = self.content.get(&Coin(0)),
            Bin(_) => a = self.content.get(&Bin(0..0)),
            Crate(_) => a = self.content.get(&Crate(0..0)),
            Bank(_) => a = self.content.get(&Bank(0..0)),
            Water(_) => a = self.content.get(&Water(0)),
            Market(_) => a = self.content.get(&Market(0)),
            Fish(_) => a = self.content.get(&Fish(0)),
            Building => a = self.content.get(&Building),
            Bush(_) => a = self.content.get(&Bush(0)),
            JollyBlock(_) => a = self.content.get(&JollyBlock(0)),
            Scarecrow => a = self.content.get(&Scarecrow),
            None => a = self.content.get(&None),
        };
        match a {
            Some(v) => v.iter().map(|p| (p.x, p.y)).collect(),
            Option::None => Vec::new()
        }
        //a.unwrap().iter().map(|p| (p.x, p.y)).collect()
    }

    pub(crate) fn get_action_vec(&self, xc: usize, yc: usize) -> Result<Vec<Action>, LibError> {
        self.check_borders(xc, yc)?;
        let mut p = Pair::new(xc, yc);
        if self.cost.get(&p).is_none() {
            return Ok(vec![]);
        }
        let mut v = VecDeque::new();
        while p.x != self.start.x || p.y != self.start.y {
            let (_, a) = self.cost.get(&p).unwrap();
            match a {
                Action::North => {
                    p.x += 1;
                    v.push_front(Action::North)
                }
                Action::South => {
                    p.x -= 1;
                    v.push_front(Action::South)
                }
                Action::West => {
                    p.y += 1;
                    v.push_front(Action::West)
                }
                Action::East => {
                    p.y -= 1;
                    v.push_front(Action::East)
                }
                Action::Teleport(row, col) => {
                    v.push_front(Action::Teleport(p.x, p.y));
                    p.x = *row;
                    p.y = *col;
                }
            }
        }
        Ok(Vec::from(v))
    }
}

//Questa funzione non è utile perchè la mappa interna di Smart Map è poco utile da sola,
// dato che molte tra le funzioni killer hanno bisogno delle altre strutture, inoltre avere
// l'action associata alla Tile è utile solo quando poi sai ricostruire facilmente
// il percorso, cosa chè ho già implementato
/*/// Presa una mappa e una coordinata calcola i cammini minimi dalla coordinata ad ogni altra `Tile` e restituisce una mappa avente costo e "predecessore" per ogni `Tile`.
///
/// # input
/// * `map: &Vec<Vec<Tile>>` la mappa su cui verranno calcolati i cammini minimi
/// * `start: (usize, usize)` la coordinata di partenza `(row, col)`
///
/// # output
/// `Result<Vec<Vec<(Tile, Option<(usize, Action)>)>>, LibError>`
///
/// In caso start ecceda i limiti della mappa la funzione restituisce `Err(LibError::OutOfBounds)`,
/// altrimenti restituisce `Ok(Vec<Vec<(Tile, Option<usize, Action>)>>)`, se una `Tile` nella matrice/mappa
/// non è raggiungibile sarà accoppiata a `None`, altrimenti sarà accoppiata a `Some((usize, Action))` in cui `usize`
/// è il costo del percorso fino alla tile e `Action` è l'azione necessaria per passare dalla `Tile`
/// precedente a quella del caso.
pub fn calculate_min_paths(&mut self, map: &Vec<Vec<Tile>>, start: (usize, usize)) -> Result<Vec<Vec<(Tile, Option<(usize, Action)>)>>, LibError> {
    /*if start.0 >= map.len() || start.1 >= map[start.0].len() {
        Err(LibError::OutOfBounds)
    } else {*/ //non dovrebbe essere necessario
        let mut sm = SmartMap::new();
        sm.update_map(&map.clone().into_iter().map(|v| v.into_iter().map(|t| Some(t)).collect()).collect());
        sm.update_cost(start.0, start.1)?;
        Ok(map.clone().into_iter().enumerate().map(|(r, v)| v.into_iter().enumerate().map(|(c, t)| (t, self.sm.get_cost_and_action(r, c))).collect()).collect())
    /*}*/
}*/
