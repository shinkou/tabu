use mobc_postgres::tokio_postgres;
use serde::Serialize;
use thiserror::Error;
use warp::{Reply, Rejection, http::StatusCode, reject, reply};
use std::convert::Infallible;

#[derive(Error, Debug)]
pub enum Error
{
	#[error("error getting connection from DB pool: {0}")]
	DBPoolError(mobc::Error<tokio_postgres::Error>)
	, #[error("error executing DB query: {0}")]
	DBQueryError(#[from] tokio_postgres::Error)
}

#[derive(Serialize)]
struct ErrorResponse
{
	message: String
}

impl reject::Reject for Error {}

pub async fn handle_rejection(err: Rejection)
	-> std::result::Result<impl Reply, Infallible>
{
	let code;
	let msg;

	if err.is_not_found()
	{
		code = StatusCode::NOT_FOUND;
		msg = "Not Found";
	}
	else if let Some(_)
		= err.find::<warp::filters::body::BodyDeserializeError>()
	{
		code = StatusCode::BAD_REQUEST;
		msg = "Invalid Body";
	}
	else if let Some(e) = err.find::<Error>()
	{
		match e
		{
			Error::DBQueryError(_) =>
			{
				code = StatusCode::BAD_REQUEST;
				msg = "Could not Execute request";
			}
			_ =>
			{
				eprintln!("Unhandled application error: {:?}", err);
				code = StatusCode::INTERNAL_SERVER_ERROR;
				msg = "Internal Server Error";
			}
		}
	}
	else if let Some(_) = err.find::<reject::MethodNotAllowed>()
	{
		code = StatusCode::METHOD_NOT_ALLOWED;
		msg = "Method Not Allowed";
	}
	else
	{
		eprintln!("Unhandled error: {:?}", err);
		code = StatusCode::INTERNAL_SERVER_ERROR;
		msg = "Internal Server Error";
	}

	let json = reply::json(&ErrorResponse {message: msg.into()});

	Ok(reply::with_status(json, code))
}
