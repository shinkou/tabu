use crate::{DBCnx, DBPool, data, error};
use data::{Id, Tabu};
use error::Error::*;
use mobc::Pool;
use mobc_postgres::{tokio_postgres, PgConnectionManager};
use tokio_postgres::{Config, Error, NoTls, row::Row};
use std::str::FromStr;
use std::time::Duration;

type Result<T> = std::result::Result<T, error::Error>;

const DBPOOL_MAX_OPEN: u64 = 32;
const DBPOOL_MAX_IDLE: u64 = 8;
const DBPOOL_TIMEOUT_SECONDS: u64 = 15;

pub fn create_pool() -> std::result::Result<DBPool, mobc::Error<Error>>
{
	let config = Config::from_str
	(
		"postgres://postgres:postgres@postgres:5432/postgres"
	)?;

	let manager = PgConnectionManager::new(config, NoTls);

	Ok
	(
		Pool::builder()
			.max_open(DBPOOL_MAX_OPEN)
			.max_idle(DBPOOL_MAX_IDLE)
			.get_timeout(Some(Duration::from_secs(DBPOOL_TIMEOUT_SECONDS)))
			.build(manager)
	)
}

pub async fn get_dbcnx(dbpool: &DBPool) -> Result<DBCnx>
{
	dbpool.get().await.map_err(DBPoolError)
}

pub async fn list_tabus(dbpool: &DBPool) -> Result<Vec<Tabu>>
{
	let dbcnx = get_dbcnx(dbpool).await?;
	let rs = dbcnx
		.query("SELECT words, reason FROM tabu ORDER BY words", &[])
		.await.map_err(DBQueryError)?;
	Ok(rs.iter().map(|r| row_to_tabu(&r)).collect())
}

pub async fn create_tabu(dbpool: &DBPool, tabu: Tabu) -> Result<Tabu>
{
	let dbcnx = get_dbcnx(dbpool).await?;
	let row = dbcnx.query_one
		(
			"INSERT INTO tabu (words, reason) VALUES ($1, $2) RETURNING *"
			, &[&tabu.words, &tabu.reason]
		)
		.await
		.map_err(DBQueryError)?;
	Ok(row_to_tabu(&row))
}

pub async fn delete_tabu(dbpool: &DBPool, id: Id) -> Result<Tabu>
{
	let dbcnx = get_dbcnx(dbpool).await?;
	let row = dbcnx.query_one
		(
			"DELETE FROM tabu WHERE words = $1 RETURNING *"
			, &[&id.words]
		)
		.await
		.map_err(DBQueryError)?;
	Ok(row_to_tabu(&row))
}

pub async fn update_tabu(dbpool: &DBPool, tabu: Tabu) -> Result<Tabu>
{
	let dbcnx = get_dbcnx(dbpool).await?;
	let row = dbcnx.query_one
		(
			"UPDATE tabu SET reason = $2 WHERE words = $1 RETURNING *"
			, &[&tabu.words, &tabu.reason]
		)
		.await
		.map_err(DBQueryError)?;
	Ok(row_to_tabu(&row))
}

fn row_to_tabu(row: &Row) -> Tabu
{
	let words: String = row.get(0);
	let reason: String = row.get(1);
	Tabu {words, reason}
}
