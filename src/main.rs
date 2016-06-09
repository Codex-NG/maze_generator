extern crate rand;

use rand::Rng;
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::ops::Sub;

enum CellState {
	PASSAGE,
	BLOCKED,
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct CellPos(i32, i32);

impl Sub for CellPos {
	type Output = CellPos;
	fn sub(self, other: CellPos) -> CellPos {
		let x = (other.0 - self.0)/2 + self.0;
		let y = (other.1 - self.1)/2 + self.1;
		CellPos(x as i32, y as i32)
	}
}

struct Maze {
	height: i32,
	width: i32,
	grid: HashMap<CellPos, CellState>,
}

impl Maze {
	fn new(height: i32, width: i32) -> Maze {
		Maze {
			height: height,
			width: width,
			grid: HashMap::new(),
		}
	}

	fn generate(&mut self) {
		self.clear_grid();
		let x = rand::thread_rng().gen_range(0, self.width);
		let y = rand::thread_rng().gen_range(0, self.height);

		let cell = CellPos(x, y);
		{
			let entry = self.grid.get_mut(&cell).unwrap();
			*entry = CellState::PASSAGE;
		}

		let mut frontiers = self.get_frontiers(cell);
		while !frontiers.is_empty() {
			let index = rand::thread_rng().gen_range(0, frontiers.len());
			let cell = frontiers.swap_remove(index);
			let neighboors = self.get_neighboors(cell);

			if neighboors.len() == 0 {
				continue;
			}

			let index = rand::thread_rng().gen_range(0, neighboors.len());
			let neighboor = neighboors.get(index).unwrap();
			let passage = cell - *neighboor;
			{
				let entry = self.grid.get_mut(&passage).unwrap();
				*entry = CellState::PASSAGE;
			}
			{
				let entry = self.grid.get_mut(&cell).unwrap();
				*entry = CellState::PASSAGE;
			}
			frontiers.extend(self.get_frontiers(cell).into_iter());
		}
	}

	fn clear_grid(&mut self) {
		for x in 0..self.width {
			for y in 0..self.height {
				let cell_pos = CellPos(x, y);
				self.grid.insert(cell_pos, CellState::BLOCKED);
			}
		}
	}

	fn get_frontiers(&self, cell_pos: CellPos) -> Vec<CellPos> {
		let mut frontiers = Vec::<CellPos>::new();
		let frontiers_pos = vec![CellPos(cell_pos.0 - 2, cell_pos.1),
								CellPos(cell_pos.0 + 2, cell_pos.1),
								CellPos(cell_pos.0, cell_pos.1 - 2),
								CellPos(cell_pos.0, cell_pos.1 + 2),];

		for pos in frontiers_pos.into_iter() {
			if let Some(entry) = self.grid.get(&pos) {
				match *entry {
					CellState::BLOCKED => frontiers.push(pos),
					_ => {},
				}
			}
		}

		frontiers
	}

	fn get_neighboors(&self, cell_pos: CellPos) -> Vec<CellPos> {
		let mut neighboors = Vec::<CellPos>::new();
		let neighboors_pos = vec![CellPos(cell_pos.0 - 2, cell_pos.1),
								CellPos(cell_pos.0 + 2, cell_pos.1),
								CellPos(cell_pos.0, cell_pos.1 - 2),
								CellPos(cell_pos.0, cell_pos.1 + 2),];

		for pos in neighboors_pos.into_iter() {
			if let Some(entry) = self.grid.get(&pos) {
				match *entry {
					CellState::PASSAGE => neighboors.push(pos),
					_ => {},
				}
			}
		}

		neighboors
	}
}

fn main() {
	let mut maze: Maze = Maze::new(80, 80);
	maze.generate();
	for y in 0..maze.width {
		for x in 0..maze.height {
			let cell_pos = CellPos(x, y);
			if let Entry::Occupied(entry) = maze.grid.entry(cell_pos) {
				match *entry.get() {
					CellState::BLOCKED => print!("#"),
					CellState::PASSAGE => print!("."),
				}
			}
		}
		print!("\n");
	}
}
