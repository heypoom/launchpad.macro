use rand::Rng;

#[derive(Clone, Debug)]
pub struct Flappy {
	pub score: u32,
	pub is_dead: bool,

	pub jumping: bool,
	pub char_y: u32,

	pub tick: u32,

	// Vec<Time, TopHeight(1, 3), BottomHeight(1, 3)>
	pub obstacles: Vec<(u8, u8, u8)>,
}

impl Flappy {
	pub fn new() -> Flappy {
		Flappy {
			is_dead: false,
			score: 0,
			jumping: false,
			obstacles: vec![],
			tick: 0,
			char_y: 3,
		}
	}

	pub fn reset(&mut self) {
		self.is_dead = false;
		self.score = 0;
		self.jumping = false;
		self.obstacles = vec![];
		self.tick = 0;
		self.char_y = 3;
	}

	pub fn jump(&mut self) {
		if self.char_y < 2 {
			return;
		}

		self.char_y -= 1;
	}

	pub fn tick(&mut self) {
		let mut rng = rand::thread_rng();

		// Gravity pulls down character
		if self.jumping {
			self.jump();
		} else if self.char_y < 7 {
			self.char_y = self.char_y + 1;
		}

		// Generate new obstacles
		if self.tick % 4 == 0 {
			let top_h = rng.gen_range(1..5);
			let bottom_h = rng.gen_range(1..(6 - top_h));

			self.obstacles.push((9, top_h, bottom_h));

			println!("new obstacle: {:?}", self.obstacles);
		}

		self.tick += 1;

		// Clear empty obstacles
		self.obstacles = self
			.obstacles
			.clone()
			.into_iter()
			.filter(|(time, _, _)| time != &0)
			.collect();

		// Update obstacles
		for (id, obstacle) in self.obstacles.clone().into_iter().enumerate() {
			let (time, top_h, bottom_h) = obstacle;

			if time == 0 {
				continue;
			}

			self.obstacles[id] = (time - 1, top_h, bottom_h);
		}

		self.score = self.score + 1;
	}
}
