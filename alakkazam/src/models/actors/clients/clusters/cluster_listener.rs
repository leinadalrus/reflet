use std::{
	prelude::v1::*,
	process::Command,
};

use actix::prelude::*;
use bollard::{
	CreateContainerOptions,
	Config,
	Docker,
	service::{Body1/*::force_new_cluster*/, ClusterInfo, SwarmInfo/*::cluster*/, SystemInfo/*..::cluster_advertise, ..::cluster_store*/},
};

struct ConfStdio {
	stdin_argsv: Vec<String>,
}

impl ConfDefmodule for ConfStdio {
	fn convert() {}

	fn parse(stdin: &str) {
		this(stdin_argsv)
	}

	fn argstdin() { // "How do I invoke a system command and capture its output?". Available at: https://stackoverflow.com/a/25574952
		Command::new("cd --prefix /src \\ |")
		.arg("./ \\ |")
		.arg("composedockerfile.sh")
		.spawn()
		.expect("Changing directory\
			then initiating shellscript:\
			\"composedockerfile.sh\" $=> [<FAILED>].");

		println!("status: {}", output.status);
		println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
		println!("stderr: {}", String::from_utf8_lossy(&output.stderr));

		assert!(output.status.success());
	}

	fn this(self) {

	}
}

trait ConfDefmodule {
	fn convert();

	fn parse(stdin: &str);
}

/** NOTE(Daniel): SwarmInfo, Option<ClusterInfo>
* Need to get ClusterInfo, and handle object(s) via attribute(s) I/O.
* 
*/


