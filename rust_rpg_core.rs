use std::io;

// remake of rpg_core in rust (direct remake)

fn attack(x: i32, y: i32, z: i32)
{
	// i dont think im doing this right xd
	let return_value : i32 = 0

	let return_value = x-y

	let return_value = z - return_value

	return return_value

}



struct enemy
{
	pub mut health: i32,
	pub mut ac: i32,
	pub mut damage: i32
}

impl enemy
{
	fn get_health(&self) -> i32;
	{
		self.health;
	}
	fn get_ac(&self) -> i32;
	{
		self.ac;
	}
	fn get_damage(&self) -> i32;
	{
		self.damage;
	}
}

struct quests
{
	pub mut diff: int32,
	pub mut reward_money: int32,
	pub mut reward_xp: int32,
	pub mut isActive: bool32 

	fn onStart()
	{
		let isActive = true
	}
	fn onEnd(starting_money: i32, starting_xp: i32)
	{
		let isActive = false
		mut return_money: i32
		mut return_xp: i32

		let return_money = starting_money + reward_money
		let return_xp = starting_xp + reward_xp
	}
}

impl quests
{
	fn get_diff(&self) -> i32;
	{
		self.diff;
	}fn get_reward_money(&self) -> i32;
	{
		self.reward_money;
	}fn get_reward_xp(&self) -> i32;
	{
		self.reward_xp;
	}fn get_activeness(&self) -> b32;
	{
		self.isActive;
	}
}
