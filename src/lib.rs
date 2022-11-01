use std::{sync::Arc, ops::Deref};

use matrix::{prelude::Conventional, Element};
use rand::Rng;
use std::sync::RwLock;

const CELLS: usize = 25; 

#[derive(PartialEq, PartialOrd)]
pub enum Directions {
    North = 1,
    South = 2,
    East = 3,
    West = 4,
}

pub struct Scene {
    cells: Arc<RwLock<Vec<Cell>>>,
}

impl Scene {
    fn new() -> Scene {
        let cells = Arc::new(RwLock::new(Vec::with_capacity(CELLS)));

        Scene { cells }
    }

    fn create_maze() -> Scene {
        // let scene = Scene::new();

        todo!()
    }

    fn choose_index(&self) -> u16 {
        todo!()
    }

    fn choose_next(&self) {
        //choose from random spaces in the immediate neighbouring cells
        let spaces: Vec<Directions> = Vec::new();

    }

    fn delete_cell(&self, index: usize) {
        self.cells.write().map(
            |mut cells| {
                cells.remove(index)
            }
        );
    }

    fn empty(&self) -> bool {
        let status = self.cells.read().map(
            |cells| {
                cells.is_empty()
            }
        );

        status.unwrap()
    }
}

impl Scene {
    fn passage(&self) {

    }

    fn get_position(&self) -> (u16, u16) {
        //check for available spaces in the scene
        //generate index from that
        todo!()
    }
}

#[derive(Copy, Clone, PartialEq)]
struct Cell (u16, u16);

impl Element for Cell {
    fn zero() -> Self {
        Cell(0, 0)
    }
}

impl Cell {
    fn new(scene_height: u16) -> Cell {
        let x = generate_num(scene_height);
        let y = generate_num(scene_height);

        Cell (x, y)
    }

    fn get_index(&self, scene: Scene) -> u16 {
        todo!()
    }

    fn cell_move(&self, direction: Directions) {

    }
}

fn generate_num(max: u16) -> u16 {
    let num: u16 = rand::thread_rng().gen_range(0..max);

    num
}

fn initiate_matrix (size: usize, values: Vec<Cell>) -> Conventional<Cell> {
    Conventional::from_vec(size, values)
}