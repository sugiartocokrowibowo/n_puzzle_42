#[derive(Debug, Clone)]
pub enum Action {
    Up,
    Down,
    Left,
    Right,
    No,
}

impl Action {
	// Return how much the action move on the x axis.
	pub fn impact_x(&self) -> i32 {
		match *self {
			Action::Up		=> 0,
			Action::Down	=> 0,
			Action::Left	=> -1,
			Action::Right	=> 1,
			Action::No		=> 0,
		}
	}

	// Return how much the action move on the y axis.
	pub fn impact_y(&self) -> i32 {
		match *self {
			Action::Up		=> -1,
			Action::Down	=> 1,
			Action::Left	=> 0,
			Action::Right	=> 0,
			Action::No		=> 0,
		}
	}
}

