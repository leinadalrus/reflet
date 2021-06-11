use futures::{StreamExt, TryStreamExt};
use kube::api::{Api, ResourceExt, ListParams, PostParams, WatchEvent};
use kube::Client;
use k8s_openapi::api::core::v1::Pod;

struct K8sOpenObserver;

impl K8sOpenObserver {
	fn observe() -> Result<(), Err> {
		let api = Api::<Pod>::namespaced(client, "default");
		let obsv = watcher(api, ListParams::default());
		let mut apply_events = try_flatten_applied(obsv)
													.boxed_local();

		while let Some(event) = apply_events.try_next().await? {
		    match(event) {
			    WatchEvent::Added(output) => println!("{:?}", output.name()),
			    WatchEvent::Modified(output) => {
			    	let s = output.status.as_ref().expect("Status exists on pod [...]");
			    	let phase = s.phase.clone().unwrap_or_default();
			    	println!("Modified: {} with phase: {}", output.name(), phase);
			    },

			    WatchEvent::Deleted(output) => println!("Deleted {}", output.name()),
			    WatchEvent::Error(e) => println!("Error {}", e),
			    _ => {},
			}
		}

		Ok(())
	}
}