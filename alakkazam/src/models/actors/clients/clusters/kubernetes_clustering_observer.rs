use futures::{StreamExt, TryStreamExt};
use kube::api::{Api, ResourceExt, ListParams, PostParams, WatchEvent};
use kube::Client;
use k8s_openapi::api::core::v1::Pod;

struct K8sOpenObserver;

impl K8sOpenObserver {
	fn observe() {
		let api = Api::<Pod>::namespaced(client, "default");
		let obsv = watcher(api, ListParams::default());
		let mut apply_events = try_flatten_applied(obsv)
													.boxed_local();

		while let Some(event) = apply_events.try_next().await? {
		    println!("Applied: {}", event.name());
		}
	}
}