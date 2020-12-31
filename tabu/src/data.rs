use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Id
{
	pub words: String
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Tabu
{
	pub words: String
	, pub reason: String
}
