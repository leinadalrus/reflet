use futures::{StreamExt, TryStreamExt};
use kube::api::{Api, ResourceExt, ListParams, PostParams, WatchEvent};
use kube::Client;
use k8s_openapi::api::core::v1::Pod;

struct KubectlConfig;

impl KubectlConfig {
	fn create_controller() {
		Controller::new(root_kind_api, ListParams::default())
															.owns(child_kind_api, ListParams::default())
															.run(reconcile, error_policy, context)
															.for_each(|res| async move {
																match res {
																	Ok(out) => info!("`reconciled` {:?}", out),
																	Err(err) => warn!("`reconcile` {:?}", err),
																}
															})
															.await;
	}
}