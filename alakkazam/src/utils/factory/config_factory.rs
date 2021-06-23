use alloc::string::String;
use std::{ prelude::v1::*,
	collections::HashMap,
	default::Default,
	io::Write,
	process::Command,
}
use actix::prelude::*;
use bollard::{
	CreateContainerOptions,
	Config,
	Docker,
};
use futures::{StreamExt, TryStreamExt};
use serde_json::{
	ser::Formatter,
	Serializer,
	value::Value,
};

struct ConfigFactoryI<W, F>/*(alloc::string::String, serde_json::value::Value)*/;

impl<W, F> Serializer for ConfigFactoryI<W, F>
	where W: Write,
			F: Formatter {
	pub fn new() -> ConfigFactoryI<W, F> {
		ConfigFactoryI
	}

	fn configure_resource_creation() {
		Docker::connect_with_http_defaults();

		let ops = Some(CreateContainerOptions {
			name: format!("{:?}", handled_route)
		});

		let conf = Config {
			image: Some("{ops.name}"),
			cmd: Some(vec!["/configs/"]),
			..Default::default()
		}

		docker.create_container(ops, conf);
	}
}