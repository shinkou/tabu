use std::convert::Infallible;
use mobc::{Connection, Pool};
use mobc_postgres::{tokio_postgres, PgConnectionManager};
use tokio_postgres::NoTls;
use warp::{Filter, Rejection};

mod data;
mod db;
mod error;
mod handler;

use data::{Id, Tabu};

type DBCnx = Connection<PgConnectionManager<NoTls>>;
type DBPool = Pool<PgConnectionManager<NoTls>>;

fn with_db(dbpool: DBPool)
	-> impl Filter<Extract = (DBPool,), Error = Infallible> + Clone
{
	warp::any().map(move || dbpool.clone())
}

fn delete_json() -> impl Filter<Extract = (Id,), Error = Rejection> + Clone
{
	warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

fn post_json() -> impl Filter<Extract = (Tabu,), Error = Rejection> + Clone
{
	warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

#[tokio::main]
async fn main()
{
	let dbpool = db::create_pool().expect("database pool can be created");

	let show_all_tabus = warp::get()
		.and(warp::path::end())
		.and(with_db(dbpool.clone()))
		.and_then(handler::list_tabus_handler);

	let add_one_tabu = warp::post()
		.and(warp::path::end())
		.and(post_json())
		.and(with_db(dbpool.clone()))
		.and_then(handler::create_tabu_handler);

	let delete_one_tabu = warp::delete()
		.and(warp::path::end())
		.and(delete_json())
		.and(with_db(dbpool.clone()))
		.and_then(handler::delete_tabu_handler);

	let update_one_tabu = warp::put()
		.and(warp::path::end())
		.and(post_json())
		.and(with_db(dbpool.clone()))
		.and_then(handler::update_tabu_handler);

	let routes = show_all_tabus
		.or(add_one_tabu)
		.or(delete_one_tabu)
		.or(update_one_tabu)
		.with(warp::cors().allow_any_origin())
		.recover(error::handle_rejection); 

	warp::serve(routes)
		.run(([0, 0, 0, 0], 80))
		.await;
}
