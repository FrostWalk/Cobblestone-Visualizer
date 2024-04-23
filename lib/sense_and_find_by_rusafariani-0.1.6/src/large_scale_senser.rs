use robotics_lib::interface::{discover_tiles, robot_map};
use robotics_lib::runner::Runnable;
use robotics_lib::utils::LibError;
use robotics_lib::world::tile::Tile;
use robotics_lib::world::World;

pub(crate) struct LargeScaleSenser {
    world_size: Option<usize>,
    initialised: bool,
}

/// permette di effettuare un campionamento della mappa intorno al robot per ottenerne un'approssimazione
impl LargeScaleSenser {
    pub fn new() -> Self {
        LargeScaleSenser {
            world_size: None,
            initialised: false,
        }
    }

    fn initialize_world_size(&mut self, world: &World) {
        if !self.initialised {
            self.world_size = Some(
                robot_map(world)
                    .expect("failed to retrieve robot_map()")
                    .len(),
            )
        }
    }

    /// restituisce una mappa quadrata centrata nella posizione del robot
    pub fn sense_centered_square(
        &mut self,
        l: usize,
        world: &mut World,
        robot: &mut impl Runnable,
        granularity: usize,
    ) -> Result<Vec<Vec<((usize, usize), Tile, bool)>>, LibError> {
        let robot_pos = robot.get_coordinate();
        let center = (robot_pos.get_row(), robot_pos.get_col());

        self.sense_square_by_center(l, world, robot, granularity, center)
    }

    /// restituisce una mappa quadrata centrata nella posizione center
    pub fn sense_square_by_center(
        &mut self,
        l: usize,
        world: &mut World,
        robot: &mut impl Runnable,
        granularity: usize,
        center: (usize, usize),
    ) -> Result<Vec<Vec<((usize, usize), Tile, bool)>>, LibError> {
        self.initialize_world_size(world);
        if l == 0 {
            return Ok(Vec::new());
        }

        //ct_r e ct_c sono gli indici della posizione che il robot avrà nella matrice restituita
        let (ct_r, ct_c) = Self::center_by_edge(l);

        //condition = true se la porzione di mappa che si vuole rilevare non rientra nella mappa del mondo
        let condition = {
            //se il quadrato finisce oltre il limite superiore del mondo
            ((center.0 as i32) - (ct_r as i32)) < 0 ||
                //se il quadrato finisce oltre il limite sinistro del mondo
                ((center.1 as i32) - (ct_c as i32)) < 0 ||
                //se il quadrato finisce oltre quello che "credo" essere il limite inferiore del mondo
                center.0 + l - ct_r > self.world_size.unwrap() ||
                //se il quadrato finisce oltre quello che "credo" essere il limite destro del mondo
                center.1 + l - ct_c > self.world_size.unwrap()
        };

        //se il quadrato rientra nel limite della mappa continuo, altrimenti restituisco un errore
        if condition {
            Err(LibError::OutOfBounds)
        } else {
            //tl_r e tl_c sono rispettivamente riga e colonna corrispondenti alla tile
            //nell'angolo in alto a sinistra della mappa che verrà restituita
            let tl_r = center.0 - ct_r;
            let tl_c = center.1 - ct_c;

            let _br_r = tl_r + l - 1;
            let _br_c = tl_r + l - 1;

            if robot
                .get_energy()
                .has_enough_energy(Self::sense_square_energy_consumption(
                    l,
                    granularity,
                    (center.0, center.1),
                    world,
                ))
            {
                let mut map = robot_map(world).unwrap();
                //caso speciale in cui nella mappa da restituire c'è un solo campione, che, per una granularità troppo elevata potrebbe capitare addirittura all'esterno della mappa
                if l <= granularity + 1 + ((granularity + 1) / 2) {
                    let tile = match map[center.0][center.1].clone() {
                        Some(t) => t,
                        None => {
                            let dt = discover_tiles(robot, world, &[(center.0, center.1)])?;
                            dt.get(&(center.0, center.1))
                                .unwrap()
                                .as_ref()
                                .unwrap()
                                .clone()
                        }
                    };

                    let mut vec = Vec::new();
                    for i in 0..l {
                        vec.push(Vec::new());
                        for j in 0..l {
                            vec[i].push(((tl_r + i, tl_c + j), tile.clone(), false));
                        }
                    }
                    vec[ct_r][ct_c].2 = true;
                    return Ok(vec);
                } else {
                    //caso normale
                    let base = (granularity + 1) / 2;
                    let mut row = base;
                    let mut col = base;
                    let mut vec = vec![vec![None; l]; l];

                    //(l / (granularity + 1) + if l % granularity > (granularity + 1) / 2 { 1 } else { 0 }

                    while row < l
                    /*&& if let Some(x) = self.world_size_range.1 { row + tl_r < x } else { true }*/
                    {
                        while col < l
                        /*&& if let Some(x) = self.world_size_range.1 { col + tl_c < x } else { true }*/
                        {
                            let tile = match map[tl_r + row][tl_c + col].clone() {
                                Some(t) => t,
                                None => {
                                    let dt =
                                        discover_tiles(robot, world, &[(tl_r + row, tl_c + col)])?;

                                    map[tl_r + row][tl_c + col] = Some(dt.get(&(tl_r + row, tl_c + col))
                                        .unwrap()
                                        .as_ref()
                                        .unwrap()
                                        .clone());
                                    map[tl_r + row][tl_c + col].clone().unwrap()
                                }
                            };

                            if granularity == 0 {
                                vec[row][col] = Some(((tl_r + row, tl_c + col), tile.clone(), true))
                            } else {
                                for i in ((row - (granularity + 1) / 2)
                                    ..(row + (granularity + 1) / 2))
                                    .take_while(|x| *x < l)
                                {
                                    for j in ((col - (granularity + 1) / 2)
                                        ..(col + (granularity + 1) / 2))
                                        .take_while(|x| *x < l)
                                    {
                                        let (t, b) = match map[tl_r + i][tl_c + j].clone() {
                                            Some(a) => (a, true),
                                            None => {
                                                let /*mut*/ tt = tile.clone();
                                                (tt, false)
                                            },
                                        };
                                        vec[i][j] = Some(((tl_r + i, tl_c + j), t, b));
                                    }
                                }
                            }

                            col += granularity + 1;
                        }
                        row += granularity + 1;
                        col = base;
                    }

                    for i in vec.iter_mut() {
                        for j in 1..l {
                            if i[j].is_none() {
                                i[j] = i[j - 1].clone().map(|e| {
                                    ((e.0.0, e.0.1 + 1), e.1, false)
                                });
                            }
                        }
                    }
                    for i in 1..l {
                        if vec[i][0].is_none() {
                            vec[i] = vec[i - 1].clone().into_iter().map(|oe| {
                                let e = oe.unwrap();
                                Some(((e.0.0 + 1, e.0.1), e.1, false))
                            }).collect();
                        }
                    }

                    if granularity > 0 {
                        row = base;
                        col = base;

                        while row < l
                        /*&& if let Some(x) = self.world_size_range.1 { row + tl_r < x } else { true }*/
                        {
                            while col < l
                            /*&& if let Some(x) = self.world_size_range.1 { col + tl_c < x } else { true }*/
                            {
                                let tile = vec[row][col].clone().unwrap().1;
                                for i in ((row - (granularity + 1) / 2)
                                    ..(row + (granularity + 1) / 2))
                                    .take_while(|x| *x < l)
                                {
                                    for j in ((col - (granularity + 1) / 2)
                                        ..(col + (granularity + 1) / 2))
                                        .take_while(|x| *x < l)
                                    {
                                        let elevation = {
                                            match vec[i][j].clone().unwrap().2 {
                                                true => {
                                                    vec[i][j].clone().unwrap().1.elevation
                                                },
                                                false => {
                                                    let /*mut*/ tt = tile.clone();
                                                    // qui va calcolato il valore di elevation per interpolazione
                                                    let (q00, q01, q10, q11) = {
                                                        let mut q00: (usize, (i32, i32)) = (tt.elevation, (0, 0));
                                                        let mut q01: (usize, (i32, i32)) = (tt.elevation, (0, 0));
                                                        let mut q10: (usize, (i32, i32)) = (tt.elevation, (0, 0));
                                                        let mut q11: (usize, (i32, i32)) = (tt.elevation, (0, 0));
                                                        if i < row && j < col {
                                                            // in alto a sinistra
                                                            q00.1 = (tl_r as i32 + row as i32 - granularity as i32 - 1, tl_c as i32 + col as i32 - granularity as i32 - 1);
                                                            q01.1 = (tl_r as i32 + row as i32 - granularity as i32 - 1, tl_c as i32 + col as i32);
                                                            q10.1 = (tl_r as i32 + row as i32, tl_c as i32 + col as i32 - granularity as i32 - 1);
                                                            q11.1 = (tl_r as i32 + row as i32, tl_c as i32 + col as i32);

                                                            if row as i32 - granularity as i32 - 1 >= 0 {
                                                                q01.0 = vec[row - granularity - 1][col].clone().unwrap().1.elevation;
                                                            }
                                                            if col as i32 - granularity as i32 - 1 >= 0 {
                                                                q10.0 = vec[row][col - granularity - 1].clone().unwrap().1.elevation;
                                                            }
                                                            if row as i32 - granularity as i32 - 1 >= 0 && col as i32 - granularity as i32 - 1 >= 0 {
                                                                q00.0 = vec[row - granularity - 1][col - granularity - 1].clone().unwrap().1.elevation;
                                                            }
                                                        } else if i < row && j >= col {
                                                            // in alto a destra
                                                            q00.1 = (tl_r as i32 + row as i32 - granularity as i32 - 1, tl_c as i32 + col as i32);
                                                            q01.1 = (tl_r as i32 + row as i32 - granularity as i32 - 1, tl_c as i32 + col as i32 + granularity as i32 + 1);
                                                            q10.1 = (tl_r as i32 + row as i32, tl_c as i32 + col as i32);
                                                            q11.1 = (tl_r as i32 + row as i32, tl_c as i32 + col as i32 + granularity as i32 + 1);
                                                            if row as i32 - granularity as i32 - 1 >= 0 {
                                                                q00.0 = vec[row - granularity - 1][col].clone().unwrap().1.elevation;
                                                            }
                                                            if col as i32 + granularity as i32 + 1 <= l as i32 - 1 {
                                                                q11.0 = vec[row][col + granularity + 1].clone().unwrap().1.elevation;
                                                            }
                                                            if row as i32 - granularity as i32 - 1 >= 0 as i32 && col as i32 + granularity as i32 + 1 <= l as i32 - 1 {
                                                                q01.0 = vec[row - granularity - 1][col + granularity + 1].clone().unwrap().1.elevation;
                                                            }
                                                        } else if i >= row && j < col {
                                                            // in basso a sinistra
                                                            q00.1 = (tl_r as i32 + row as i32, tl_c as i32 + col as i32 - granularity as i32 - 1);
                                                            q01.1 = (tl_r as i32 + row as i32, tl_c as i32 + col as i32);
                                                            q10.1 = (tl_r as i32 + row as i32 + granularity as i32 + 1, tl_c as i32 + col as i32 - granularity as i32 - 1);
                                                            q11.1 = (tl_r as i32 + row as i32 + granularity as i32 + 1, tl_c as i32 + col as i32);
                                                            if col as i32 - granularity as i32 - 1 >= 0 {
                                                                q00.0 = vec[row][col - granularity - 1].clone().unwrap().1.elevation;
                                                            }
                                                            if row as i32 + granularity as i32 + 1 <= l as i32 - 1 {
                                                                q11.0 = vec[row + granularity + 1][col].clone().unwrap().1.elevation;
                                                            }
                                                            if col as i32 - granularity as i32 - 1 >= 0 && row as i32 + granularity as i32 + 1 <= l as i32 - 1 {
                                                                q10.0 = vec[row + granularity + 1][col - granularity - 1].clone().unwrap().1.elevation;
                                                            }
                                                        } else {
                                                            // in basso a destra
                                                            q00.1 = (tl_r as i32 + row as i32, tl_c as i32 + col as i32);
                                                            q01.1 = (tl_r as i32 + row as i32, tl_c as i32 + col as i32 + granularity as i32 + 1);
                                                            q10.1 = (tl_r as i32 + row as i32 + granularity as i32 + 1, tl_c as i32 + col as i32);
                                                            q11.1 = (tl_r as i32 + row as i32 + granularity as i32 + 1, tl_c as i32 + col as i32 + granularity as i32 + 1);
                                                            if col as i32 + granularity as i32 + 1 <= l as i32 - 1 && row as i32 + granularity as i32 + 1 <= l as i32 - 1 {
                                                                q11.0 = vec[row + granularity + 1][col + granularity + 1].clone().unwrap().1.elevation;
                                                            }
                                                            if col as i32 + granularity as i32 + 1 <= l as i32 - 1 {
                                                                q01.0 = vec[row][col + granularity + 1].clone().unwrap().1.elevation;
                                                            }
                                                            if row as i32 + granularity as i32 + 1 <= l as i32 - 1 {
                                                                q10.0 = vec[row + granularity + 1][col].clone().unwrap().1.elevation;
                                                            }
                                                        }
                                                        (q00, q01, q10, q11)
                                                    };
                                                    let sxu = (q01.0 as i32 - q00.0 as i32) as f64 / (granularity + 1) as f64; // left to right
                                                    let sxd = (q11.0 as i32 - q10.0 as i32) as f64 / (granularity + 1) as f64;
                                                    let hxu = q00.0 as f64 + sxu * (j as i32 - q00.1.1 + tl_c as i32) as f64;
                                                    let hxd = q10.0 as f64 + sxd * (j as i32 - q10.1.1 + tl_c as i32) as f64;
                                                    let sy = (hxd - hxu) / (granularity + 1) as f64; // up to down
                                                    let fh = (hxu + sy * (i as i32 - q00.1.0 + tl_r as i32) as f64).round() as usize; // altezza interpolata
                                                    fh
                                                }
                                            }
                                        };
                                        vec[i][j].as_mut().map(|e| e.1.elevation = elevation);
                                    }
                                }

                                col += granularity + 1;
                            }
                            row += granularity + 1;
                            col = base;
                        }
                    }



                    return Ok(vec
                        .into_iter()
                        .map(|v| v.into_iter().map(|w| w.unwrap()).collect())
                        .collect());
                }
            } else {
                return Err(LibError::NotEnoughEnergy);
            }
        }
    }

    ///restituisce una mappa quadrata il cui angolo superiore sinistro corrisponde alla posizione corner
    pub fn sense_square_by_corner(
        &mut self,
        l: usize,
        world: &mut World,
        robot: &mut impl Runnable,
        granularity: usize,
        corner: (usize, usize),
    ) -> Result<Vec<Vec<((usize, usize), Tile, bool)>>, LibError> {
        let (ct_r, ct_c) = Self::center_by_edge(l);

        let center = (corner.0 + ct_r, corner.1 + ct_c);
        self.sense_square_by_center(l, world, robot, granularity, center)
    }

    pub fn sense_square_energy_consumption(
        l: usize,
        granularity: usize,
        center: (usize, usize),
        world: &World,
    ) -> usize {
        if l <= granularity + 1 + ((granularity + 1) / 2) {
            if Self::already_discovered(center, world) {
                0
            } else {
                3
            }
        } else {
            3 * (l / (granularity + 1)
                + if granularity != 0 && l % (granularity + 1) > (granularity + 1) / 2 {
                    1
                } else {
                    0
                })
            .pow(2)
                - Self::already_discovered_count(l, granularity, center, world)
        }
    }

    pub fn sense_square_energy_consumption_by_corner(
        l: usize,
        granularity: usize,
        corner: (usize, usize),
        world: &World,
    ) -> usize {
        let (ct_r, ct_c) = Self::center_by_edge(l);
        let center = (corner.0 + ct_r, corner.1 + ct_c);
        Self::sense_square_energy_consumption(l, granularity, center, world)
    }

    //conta le tile da campionare che sono già state scoperte
    fn already_discovered_count(
        l: usize,
        granularity: usize,
        center: (usize, usize),
        world: &World,
    ) -> usize {
        let (ct_r, ct_c) = Self::center_by_edge(l);

        let tl_r = center.0 - ct_r;
        let tl_c = center.1 - ct_c;
        let base = (granularity + 1) / 2;
        if l <= granularity + 1 + ((granularity + 1) / 2) {
            if Self::already_discovered(center, world) {
                1
            } else {
                0
            }
        } else {
            let mut row = base;
            let mut col = row;
            let map = robot_map(world).unwrap();
            let mut cont = 0;
            while row < l
            /*&& if let Some(x) = self.world_size_range.1 { row + tl_r < x } else { true }*/
            {
                while col < l
                /*&& if let Some(x) = self.world_size_range.1 { col + tl_c < x } else { true }*/
                {
                    /*
                    campiono la tile row, col,
                    riempio le tile intorno,
                    aggiorno col,
                    passo alla prossima
                    */
                    if map[tl_r + row][tl_c + col].is_some() {
                        cont += 1;
                    }
                    col += granularity + 1;
                }
                row += granularity + 1;
                col = base;
            }
            cont
        }
    }

    /// se il lato è dispari restituisce la tile centrale
    /// se il lato è pari restituisce la tile in alto a sinistra delle 4 centrali
    /// la funzione panica se le si passa l = 0
    fn center_by_edge(l: usize) -> (usize, usize) {
        if l == 0 {
            panic!("illegal parameter 0");
        }

        if l % 2 == 1 {
            (l / 2, l / 2)
        } else {
            //facciamo sbucare il robot nella tile in alto a sinistra delle 4 centrali
            (l / 2 - 1, l / 2 - 1)
        }
    }

    fn already_discovered(coordinate: (usize, usize), world: &World) -> bool {
        robot_map(world).unwrap()[coordinate.0][coordinate.1].is_some()
    }
}
