extern crate getopts;

use std::env;
use getopts::Options;

fn print_usage(progname: &str, opts: Options)
{
	println!("{}", opts.usage(&format!("Usage: {} [options]", progname)));
}

pub fn do_args() -> (String,)
{
	let mut cnxstr = String::from
	(
		"postgres://postgres:postgres@postgres:5432/postgres"
	);
	let args: Vec<String> = env::args().collect();
	let progname: &str = args[0].as_str();

	let mut opts = Options::new();
	opts.optflag("h", "help", "Show this help menu.");
	opts.optopt
	(
		"c"
		, "connection-url"
		, "Connection URL. \
			(default: \"postgres://postgres:postgres@postgres:5432/postgres\")"
		, "URL"
	);
	let matches = match opts.parse(&args[1..])
	{
		Ok(m) => m
		, Err(e) => panic!(e.to_string())
	};

	if matches.opt_present("h")
	{
		print_usage(&progname, opts);
	}
	else
	{
		if let Some(s) = matches.opt_str("c")
		{
			cnxstr = s;
		}
	}

	(cnxstr,)
}
