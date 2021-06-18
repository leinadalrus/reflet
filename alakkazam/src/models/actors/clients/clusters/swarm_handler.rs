use std::{
	prelude::v1::*,
	collections::HashMap,
	default::Default,
	process::Command,
};
use bollard::{
	CreateContainerOptions,
	Config,
	Docker,
};
use serde_json::{
	ser::Formatter,
	StreamDeserializer,
	value::Value,
};

struct SwarmHandler;

impl SwarmHandler {
	pub fn new() {}
	fn handle() {}
}