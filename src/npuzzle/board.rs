use npuzzle::errors::{IncorrectBoardError, ParseError};
use std::fmt::{Formatter, Display, Error};
use npuzzle::{Tile, Action};

#[derive(Debug, Clone)]
pub struct Board
{
	size:		usize,
	tiles:		Vec<Tile>,
}

impl Board
{
	pub fn new(size: usize) -> Board {
		Board {
			size:	size,
			tiles:	Vec::with_capacity(size * size),
		}
	}

	pub fn new_with_tiles(size: usize, tiles: Vec<Tile>) -> Board {
		Board {
			size:	size,
			tiles:	tiles,
		}
	}

	pub fn append_tiles(&mut self, new_tiles: &mut Vec<Tile>) {
		self.tiles.extend(new_tiles.iter().cloned());
	}

	pub fn get_size(&self) -> usize {
		self.size
	}

	/// Return the number of tile in the npuzzle board including the empty tile.
	pub fn nb_tile(&self) -> usize {
		self.size * self.size
	}

	/// Get the tiles which coordinates are [x, y]
	pub fn get(&self, x: usize, y: usize) -> Tile {
		self.tiles[(y * self.size + x)].clone()
	}

	pub fn is_correct(&self) -> Result<(), IncorrectBoardError> {
		// test number of tile
		if self.tiles.len() != self.nb_tile() as usize {
			return Err(IncorrectBoardError::WrongNumberOfTile{
				found:		self.tiles.len(),
				expected:	self.nb_tile(),
			});
		}

		// test if the tiles are the one expected
		let mut used_numbers : Vec<(i32, bool)> = (0..self.nb_tile())
				.map(|x| (x as i32, false)).collect();
		for i in (0..self.nb_tile()) {
			let tile_nbr = self.tiles[i as usize].to_nbr();
			if tile_nbr as usize > self.nb_tile() - 1 {
				return Err(IncorrectBoardError::OutOfBoundTile{tile: tile_nbr});
			}
			let (_, already_in) = used_numbers[tile_nbr as usize];
			if already_in {
				return Err(IncorrectBoardError::DuplicatedTile{tile: tile_nbr});
			}
			used_numbers[tile_nbr as usize] = (tile_nbr, true);
		}

		//every thing is ok !
		Ok(())
	}

	/// Return true it the tiles of this board are align as expected.
	pub fn is_complete(&self) -> bool {
		for i in (1..self.nb_tile()) {
			if self.tiles[i - 1].to_nbr() != i as i32 {
				return false;
			}
		}
		true
	}
}

impl Display for Board
{
	fn fmt(&self, f: &mut Formatter) -> Result<(), Error>
	{
		let mut to_return = Ok(());
		to_return = to_return.and(write!(f, "size : {}\n", self.size));
		for y in (0..self.size) {
			for x in (0..self.size) {
				to_return = to_return.and(write!(f, "{:<4} ", self.get(x, y)));
			}
			to_return = to_return.and(write!(f, "\n"));
		}
		to_return
	}
}