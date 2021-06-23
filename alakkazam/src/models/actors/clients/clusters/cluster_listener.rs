use alloc::string::String;
use std::{
	prelude::v1::*,
	process::Command,
	path::Path,
};
use actix::prelude::*;
use bollard::{
	API_DEFAULT_VERSION,
	Config,
	container::{InspectContainerOptions, ListContainerOptions, StatsOptions, TopOptions},
	CreateContainerOptions,
	Docker,
	network::{ConnectNetworkOptions, DisconnectNetworkOptions, InspectNetworkOptions, ListNetworksOptions},
	service::{
		Body1/*::force_new_cluster*/,
		ClusterInfo, SwarmInfo/*::cluster*/, SystemInfo/*..::cluster_advertise, ..::cluster_store*/,
		ListServicesOptions, InspectServiceOptions,
	},
};
use futures::{
	future::TryFutureExt,
	stream::{StreamExt, TryStreamExt},
};
use serde_json::{
	ser::Formatter,
	Serializer,
	value::Value,
};

trait ClusterListener {
	fn new() -> bollard::Docker;
	fn listen(cluster_info: bollard::service::ClusterInfo);
	fn handle(context: &str) -> String;
}

impl ClusterListener {
	pub fn new() -> bollard::Docker {
		bollard::Docker
	}

	fn listen(cluster_info: bollard::service::ClusterInfo) {}

	fn handle(context: &str) -> String {}
}

struct ClusterHandler;

impl ClusterHandler {
	fn http_connect() {
		conn = Docker::connect_with_http_defaults().unwrap();
		conn.ping()
			.map_ok(|_| Ok::<_, ()>(println!("[HTTP] Connection establish with HTTP defaults. (Unwrapped.)")));
	}

	fn ssl_connect() {
		let conn = Docker::connect_with_ssl_defaults().unwrap();
		conn.ping()
			.map_ok(|_| Ok::<_, ()>(println!("[SSL] Connection establish with SSL defaults. (Unwrapped.)")));
	}

	fn local_machine_connect() {
		let conn = Docker::connect_with_local_defaults().unwrap();
		conn.ping()
			.map_ok(|_| Ok::<_, ()>(println!("[Local Machine] Connection establish with Local Machine defaults. (Unwrapped.)")));
	}

	fn unix_connect() {
		let conn = Docker::connect_with_unix_defaults().unwrap();
		conn.ping()
			.map_ok(|_| Ok::<_, ()>(println!("[UNIX] Connection establish with UNIX defaults. (Unwrapped.)")));
	}
}

/** NOTE(Daniel): SwarmInfo, Option<ClusterInfo>
* Need to get ClusterInfo, and handle object(s) via attribute(s) I/O.
* 
*/


