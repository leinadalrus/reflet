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

struct ResourceDocumentConf {
	keyIdentifier: serde_json::Map<alloc::string::String/*.as_bytes()*/>,
	resultingValue: serde_json::Map<serde_json::value::Value/*.as_bytes()*/>,
} // NOTE(Daniel): Data-structure must be able to cascade; become nested-arrays/layer atop another.

struct ConfigFactoryI<W, F>;

impl<W, F> Serializer for ConfigFactoryI<W, F>
	where W: Write,
			F: Formatter {
	fn template_json(ctx: &str) -> serde_json::Serializer {
		let mut thruput_step = Vec::<String>::new();

		for res in ctx/*.bytes()*/ {
			let thruput_step = match res {
				res => ResourceDocumentConf {
					key => ResourceDocumentConf.keyIdentifier,
					val => ResourceDocumentConf.resultingValue, // NOTE(Daniel): to-rework'n'refactor.
				},
				_ => None
			}
		}
	}

	fn configure_resource_creation() {
		Docker::connect_with_http_defaults();

		let ops = Some(CreateContainerOptions {
			name: "com.dev.deploy-container"
		});

		let conf = Config {
			image: Some("com.dev.deploy-state"),
			cmd: Some(vec!["/configs/"]),
			..Default::default()
		}

		docker.create_container(ops, conf);
	}
}