#![feature(proc_macro_hygiene, decl_macro)]

use anyhow::Result;
use lazy_static::lazy_static;
use rocket::{post, routes};
use structopt::StructOpt;

use std::env;
use std::path::PathBuf;
use std::process::{Command, ExitStatus};

#[derive(Debug, StructOpt)]
struct Cla {
	/// port
	#[structopt(short, long, default_value = "8888")]
	port: u16,
	/// script
	#[structopt(parse(from_os_str))]
	script: PathBuf,
}

#[post("/")]
fn base() -> Result<String> {
	let c = Command::new(
		[&PathBuf::from("./"), &OPT.script]
			.iter()
			.collect::<PathBuf>(),
	)
	.spawn()?
	.wait()?;
	Ok(format!("{:?}", c))
}

lazy_static! {
	static ref OPT: Cla = Cla::from_args();
}

fn main() {
	env::set_var("ROCKET_PORT", format!("{}", OPT.port));

	rocket::ignite().mount("/", routes![base]).launch();
}
