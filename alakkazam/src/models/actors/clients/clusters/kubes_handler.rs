use futures::{StreamExt, TryStreamExt};
use kube::api::{Api, ResourceExt, ListParams, PostParams, WatchEvent};
use kube::Client;
use k8s_openapi::api::core::v1::Pod;

#[tokio::main]
async fn handle() -> Result<(), Err> {
	let cl = Client::try_default().await?;
	let pods: Api<Pod> = Api::namespaced(cl, "default");
	let pod = serde_json::from_value(serde_json::json!({
		"apiVersion": "v1",
		"kind": "Pod",
		"metadata": {
			"name": "name"
		},

		"spec": { "containers": [{
				"name": "name",
				"image": "s3://dev.deploy.io-state/dev.deploy.io/config"
			},],
		}
	}))?; /*NOTE(Daniel): can this be a commandline argument? Can this be programmatic?*/
}

/*TODO(Daniel): Context, Config and ConfigFactory <- [Templating] Inputs.*/