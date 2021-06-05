use futures::{StreamExt, TryStreamExt};
use kube::api::{Api, ResourceExt, ListParams, PostParams, WatchEvent};
use kube::Client;
use k8s_openapi::api::core::v1::Pod;

struct K8sReflector;

impl K8sReflector {
	fn reflect() {
		let nodes: Api<Node> = Api::namespaced(client, &namespace);
		let lp = ListParams::default()
									.labels("beta.kubernetes.io/instance-type=m4.2xlarge");
									
		let store = reflector::store::Writer::<Node>::default();
		let reader = store.as_reader();
		let rf = reflector(store, watcher(nodes, lp));
	}
}