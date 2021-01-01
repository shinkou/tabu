use crate::{DBPool, data, db};
use data::{Id, Tabu};
use warp::{Reply, Rejection, http::StatusCode, reject, reply};

type Result<T> = std::result::Result<T, Rejection>;

pub async fn list_tabus_handler(dbpool: DBPool) -> Result<impl Reply>
{
	let tabus = db::list_tabus(&dbpool)
		.await
		.map_err(|e| reject::custom(e))?;
	Ok(reply::json(&tabus))
}

pub async fn create_tabu_handler(tabu: Tabu, dbpool: DBPool)
	-> Result<impl Reply>
{
	let tabu = db::create_tabu(&dbpool, tabu)
		.await
		.map_err(|e| reject::custom(e))?;
	Ok(reply::json(&tabu))
}

pub async fn delete_tabu_handler(id: Id, dbpool: DBPool)
	-> Result<impl Reply>
{
	let tabu = db::delete_tabu(&dbpool, id)
		.await
		.map_err(|e| reject::custom(e))?;
	Ok(reply::json(&tabu))
}

pub async fn update_tabu_handler(tabu: Tabu, dbpool: DBPool)
	-> Result<impl Reply>
{
	let tabu = db::update_tabu(&dbpool, tabu)
		.await
		.map_err(|e| reject::custom(e))?;
	Ok(reply::json(&tabu))
}

pub async fn health_handler(dbpool: DBPool)
	-> Result<impl Reply>
{
	db::check_connection(&dbpool)
		.await
		.map_err(|e| reject::custom(e))?;
	Ok(StatusCode::OK)
}
