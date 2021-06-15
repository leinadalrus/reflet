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

struct SwarmHandler;

