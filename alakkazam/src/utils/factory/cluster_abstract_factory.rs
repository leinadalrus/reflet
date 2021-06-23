ClusterAbstractFactory;

impl ClusterAbstractFactory {
	pub fn new() -> ClusterAbstractFactory {
		ClusterAbstractFactory
	}

	async fn 
}

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
