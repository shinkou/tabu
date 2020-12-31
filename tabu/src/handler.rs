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

pub async fn create_tabu_handler(body: Tabu, dbpool: DBPool)
	-> Result<impl Reply>
{
	let tabu = db::create_tabu(&dbpool, body)
		.await
		.map_err(|e| reject::custom(e))?;
	Ok(reply::json(&tabu))
}

pub async fn delete_tabu_handler(id: Id, dbpool: DBPool)
	-> Result<impl Reply>
{
	let cnt = db::delete_tabu(&dbpool, id)
		.await
		.map_err(|e| reject::custom(e))?;
	if 0 < cnt
	{
		Ok(StatusCode::OK)
	}
	else
	{
		Ok(StatusCode::NOT_FOUND)
	}
}
