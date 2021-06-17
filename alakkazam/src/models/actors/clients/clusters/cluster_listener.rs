use alloc::string::String;
use std::{
	prelude::v1::*,
	process::Command,
	path::Path,
};
use actix::prelude::*;
use bollard::{
	CreateContainerOptions,
	Config,
	Docker,
	service::{Body1/*::force_new_cluster*/, ClusterInfo, SwarmInfo/*::cluster*/, SystemInfo/*..::cluster_advertise, ..::cluster_store*/},
};
use futures::{StreamExt, TryStreamExt};
use serde_json::{
	ser::Formatter,
	Serializer,
	value::Value,
};

enum ConfFiletypesE {
	DOCKERFILE, DOCKER_COMPOSE, SHELLSCRIPT, JSON, YAML,
}

struct MConfStdio;

impl MConfStdio {
	fn convert() -> serde_json::Serializer {}

	fn parse(stdin: &str) serde_json::Map<alloc::string::String, serde_json::value::Value> {
		
	}

	fn stdinArgs() { // "How do I invoke a system command and capture its output?". Available at: https://stackoverflow.com/a/25574952
		let cmd = Command::new("|")
		.arg("./ \\")
		.arg("deployment.sh"|"service.sh")
		.spawn()
		.expect("deployment.sh"|
			"service.sh");

		let mtc = match cmd {
			Some (patt) if patt == Path::new(".") => {
				".json" => ConfFiletypesE::JSON,
				".yaml" => ConfFiletypesE::YAML,
				".sh" => ConfFiletypesE::SHELLSCRIPT,
			},
			"Dockerfile" => ConfFiletypesE::DOCKERFILE,
			"docker-compose" => ConfFiletypesE::DOCKER_COMPOSE,
			_ => None,
		};

		mtc
	}
}

/** NOTE(Daniel): SwarmInfo, Option<ClusterInfo>
* Need to get ClusterInfo, and handle object(s) via attribute(s) I/O.
* 
*/


