extern crate rand;

use rand::Rng;
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::ops::Sub;

#[derive(Debug, PartialEq)]
enum CellState {
	PASSAGE,
	BLOCKED,
	LIGHT,
	ENTRY,
	EXIT,
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
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
		let x = rand::thread_rng().gen_range(1, self.width - 1);
		let y = rand::thread_rng().gen_range(1, self.height - 1);

		let x = if x % 2 == 0 { x + 1} else { x };
		let y = if y % 2 == 0 { y + 1} else { y };

		// First frontier cell needs to be odd to guarantee borders
		let cell = CellPos(x, y);
		{
			let entry = self.grid.get_mut(&cell).unwrap();
			*entry = CellState::PASSAGE;
		}

		let mut frontiers = self.get_adjcells(cell, CellState::BLOCKED, 2);
		while !frontiers.is_empty() {
			let index = rand::thread_rng().gen_range(0, frontiers.len());
			let cell = frontiers.swap_remove(index);
			let neighboors = self.get_adjcells(cell, CellState::PASSAGE, 2);

			if neighboors.is_empty() {
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

			let v: Vec<CellPos> = self.get_adjcells(cell, CellState::BLOCKED, 2)
										.into_iter()
										.filter(|value| !frontiers.contains(value))
										.collect();
			frontiers.extend(v.into_iter());
		}

		{
			let entry = self.grid.get_mut(&CellPos(1, 0)).unwrap();
			*entry = CellState::ENTRY;
		}
		{
			let entry = self.grid.get_mut(&CellPos(self.width - 2, self.height - 1)).unwrap();
			*entry = CellState::EXIT;
		}

		self.generate_light();
	}

	fn clear_grid(&mut self) {
		for x in 0..self.width {
			for y in 0..self.height {
				let cell_pos = CellPos(x, y);
				self.grid.insert(cell_pos, CellState::BLOCKED);
			}
		}
	}

	fn get_adjcells(&self, cell_pos: CellPos, cell_state: CellState, dist: i32) -> Vec<CellPos> {
		let mut adjcells = Vec::<CellPos>::new();
		let adjcells_pos = vec![CellPos(cell_pos.0 - dist, cell_pos.1),
								CellPos(cell_pos.0 + dist, cell_pos.1),
								CellPos(cell_pos.0, cell_pos.1 - dist),
								CellPos(cell_pos.0, cell_pos.1 + dist),];

		for pos in adjcells_pos.into_iter() {
			if let Some(entry) = self.grid.get(&pos) {
				match *entry {
					CellState::BLOCKED if cell_state == CellState::BLOCKED => adjcells.push(pos),
					CellState::LIGHT | CellState::PASSAGE => {
						if cell_state != CellState::BLOCKED {
							adjcells.push(pos);
						}
					},
					_ => {}
				}
			}
		}

		adjcells
	}

	fn generate_light(&mut self) {
		let mut check_stack = Vec::<CellPos>::new();
		let mut visited_cells = Vec::<CellPos>::new();
		let mut illuminated_cells = Vec::<CellPos>::new();

		check_stack.push(CellPos(1, 1));

		let mut steps = 0;
		'outter: while !check_stack.is_empty() {
			let check_pos = check_stack.pop().unwrap();
			visited_cells.push(check_pos);

			let neighboors: Vec<CellPos> = self.get_adjcells(check_pos, CellState::PASSAGE, 1)
												.into_iter()
												.filter(|value| !visited_cells.contains(value))
												.collect();
			check_stack.extend(neighboors.iter());
			steps = steps + 1;
			if steps >= 4 && !illuminated_cells.contains(&check_pos) {
				let mut neighboors: Vec<CellPos> = self.get_adjcells(check_pos, CellState::PASSAGE, 1)
									.into_iter()
									.collect();
				self.get_diagonal_cells(&mut neighboors, check_pos);
				let mut v: Vec<CellPos> = self.get_adjcells(check_pos, CellState::PASSAGE, 2);
				let mut remove_v: Vec<usize> = Vec::new();
				for x in 0..v.len() {
					let pos = check_pos - v[x];
					if !neighboors.contains(&pos) {
						remove_v.push(x);
					}
				}

				remove_v.reverse();
				for index in remove_v {
					v.remove(index);
				}

				neighboors.extend(v.into_iter());
				for neighboor in &neighboors {
					if illuminated_cells.contains(neighboor) {
						continue 'outter;
					}
				}

				{
					let entry = self.grid.get_mut(&check_pos).unwrap();
					*entry = CellState::LIGHT;
				}
				illuminated_cells.extend(neighboors.into_iter());
				illuminated_cells.push(check_pos);

				steps = 0;
			}
		}
	}

	fn get_diagonal_cells(&self, neighboors: &mut Vec<CellPos>, center: CellPos) {
		for nx in vec![-1, 1] {
			for ny in vec![-1, 1] {
				let n_pos = CellPos(center.0 + nx, center.1 + ny);
				if let Some(entry) = self.grid.get(&n_pos) {
					if *entry == CellState::PASSAGE {
						neighboors.push(n_pos);
					}
				}
			}
		}
	}
}

fn main() {
	// Maze size needs to be odd to have borders
	let mut maze: Maze = Maze::new(15, 15);
	maze.generate();
	for y in 0..maze.width {
		for x in 0..maze.height {
			let cell_pos = CellPos(x, y);
			if let Entry::Occupied(entry) = maze.grid.entry(cell_pos) {
				match *entry.get() {
					CellState::BLOCKED => print!("#"),
					CellState::PASSAGE => print!(" "),
					CellState::LIGHT => print!("."),
					CellState::ENTRY => print!("E"),
					CellState::EXIT => print!("S"),
				}
			}
		}
		print!("\n");
	}
}
