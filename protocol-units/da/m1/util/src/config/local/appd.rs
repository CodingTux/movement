use crate::config::common::{
	default_celestia_namespace, default_celestia_rpc_listen_hostname,
	default_celestia_rpc_listen_port, default_celestia_websocket_listen_hostname,
	default_celestia_websocket_listen_port,
};

use celestia_types::nmt::Namespace;
use serde::{Deserialize, Serialize};

/// The inner configuration for the local Celestia Appd Runner
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Config {
	/// The URL of the Celestia RPC
	#[serde(default = "default_celestia_rpc_listen_hostname")]
	pub celestia_rpc_listen_hostname: String,

	/// The port of the Celestia RPC
	#[serde(default = "default_celestia_rpc_listen_port")]
	pub celestia_rpc_listen_port: u16,

	/// The hostname of the Celestia Node websocket
	#[serde(default = "default_celestia_websocket_listen_hostname")]
	pub celestia_websocket_listen_hostname: String,

	/// The port of the Celestia Node websocket
	#[serde(default = "default_celestia_websocket_listen_port")]
	pub celestia_websocket_listen_port: u16,

	/// The auth token for the Celestia node
	pub celestia_auth_token: Option<String>,

	/// The namespace for the Celestia node
	#[serde(default = "default_celestia_namespace")]
	pub celestia_namespace: Namespace,

	/// The celestia app path for when that is being orchestrated locally
	/// This does not have a default because if it is needed, a default is generally not appropriate.
	pub celestia_path: Option<String>,

	/// The celestia validator address for when that is being orchestrated locally
	/// This does not have a default because if it is needed, a default is generally not appropriate.
	pub celestia_validator_address: Option<String>,
}

impl Default for Config {
	fn default() -> Self {
		Self {
			celestia_rpc_listen_hostname: default_celestia_rpc_listen_hostname(),
			celestia_rpc_listen_port: default_celestia_rpc_listen_port(),
			celestia_websocket_listen_hostname: default_celestia_websocket_listen_hostname(),
			celestia_websocket_listen_port: default_celestia_websocket_listen_port(),
			celestia_auth_token: None,
			celestia_namespace: default_celestia_namespace(),
			celestia_path: None,
			celestia_validator_address: None,
		}
	}
}
