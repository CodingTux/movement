use crate::config::common::{
	default_celestia_rpc_connection_hostname, default_celestia_rpc_connection_port,
	default_celestia_websocket_connection_hostname, default_celestia_websocket_connection_port,
};
use serde::{Deserialize, Serialize};

/// The inner configuration for the local Celestia Bridge Runner
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Config {
	/// The URL of the Celestia RPC
	#[serde(default = "default_celestia_rpc_connection_hostname")]
	pub celestia_rpc_connection_hostname: String,

	/// The port of the Celestia RPC
	#[serde(default = "default_celestia_rpc_connection_port")]
	pub celestia_rpc_connection_port: u16,

	/// The hostname of the Celestia Node websocket
	#[serde(default = "default_celestia_websocket_connection_hostname")]
	pub celestia_websocket_connection_hostname: String,

	/// The port of the Celestia Node websocket
	#[serde(default = "default_celestia_websocket_connection_port")]
	pub celestia_websocket_connection_port: u16,

	/// The celestia app path for when that is being orchestrated locally
	/// This does not have a default because if it is needed, a default is generally not appropriate.
	pub celestia_bridge_path: Option<String>,
}

impl Default for Config {
	fn default() -> Self {
		Self {
			celestia_rpc_connection_hostname: default_celestia_rpc_connection_hostname(),
			celestia_rpc_connection_port: default_celestia_rpc_connection_port(),
			celestia_websocket_connection_hostname: default_celestia_websocket_connection_hostname(
			),
			celestia_websocket_connection_port: default_celestia_websocket_connection_port(),
			celestia_bridge_path: None,
		}
	}
}
