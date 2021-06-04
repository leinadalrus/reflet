use actix::prelude::*;
use std::{
	prelude::v1::*,
	process::Command,
}

#[derive(Message)]
#[rtype(result="Result<bool, std::io::Error>")]
struct ClusterListener;

struct Actor4Cluster;

impl Actor for Actor4Cluster {
	type Context = Context<Self>;
}

impl Handler for Actor4Cluster {
	type Result = Result<bool, std::io::Error>;

	fn handle(&mut self, msg: ClusterListener, ctx: &mut Context<Self>) -> Self::Result {
		Ok(true)
	}
}

struct ClusterDockStdin;

impl ClusterDockStdin {
	fn accept_stdin_argsv(&mut argsv: Vec<String>) {
		let &mut argsv = env::args().collect()::<Vec<String>>();
		let stdin = argsv.get(1).unwrap();

		let defaulting_filepath = format!("{:?}.conf", input); // TODO: default to config or shell file-type.
		let saves_into = args.get(2).unwrap_or($defaulting_filepath);
	}

	fn stdin_argsv_and_then() {

	}
}

struct BashscriptArgstdio {
	stdin_argsv: Vec<String>,
}

impl DefmoduleArgsv for BashscriptArgstdio {
	fn convert() {}

	fn parse(stdin: &str) {
		this(self.stdin_argsv)
	}

	fn argstdin() { // "How do I invoke a system command and capture its output?". Available at: https://stackoverflow.com/a/25574952
		Command::new("cd --prefix /src \\ |")
		.arg("./ \\ |")
		.arg("composedockerargsv.sh")
		.spawn()
		.expect("Changing directory\
			then initiating shellscript:\
			\"composedockerargsv.sh\" $=> [<FAILED>].");

		println!("status: {}", output.status);
		println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
		println!("stderr: {}", String::from_utf8_lossy(&output.stderr));

		assert!(output.status.success());
	}

	fn this(self) {

	}
}

trait DefmoduleArgsv {
	fn convert();

	fn parse(stdin: &str);
}

#[derive(Debug, Clone, Copy)]
enum ArgsvOperations {
	Left, Right, 
	Home, End, 
	Newline, CarriageReturn, 
	Backslash, DoNothing,
}
