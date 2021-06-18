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

struct ConfStdioM;

impl ConfStdioM {
	fn convert() -> serde_json::Serializer {}

	fn parse(stdin: &str) serde_json::Map<alloc::string::String, serde_json::value::Value> {
		
	}

	fn conf_yaml() {

	}
}

/** NOTE(Daniel): SwarmInfo, Option<ClusterInfo>
* Need to get ClusterInfo, and handle object(s) via attribute(s) I/O.
* 
*/


