use suzuka_full_node::{
	manager::Manager,
	partial::SuzukaPartialNode,
};
use maptos_dof_execution::v1::Executor;
use std::process::ExitCode;

#[tokio::main]
async fn main() -> Result<ExitCode, anyhow::Error> {
	// let runtime = tokio::runtime::Builder::new_multi_thread().enable_all().build()?;

	use tracing_subscriber::EnvFilter;

	tracing_subscriber::fmt()
		.with_env_filter(
			EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
		)
		.init();

	// get the config file
	let dot_movement = dot_movement::DotMovement::try_from_env()?;
	let mut config_file = dot_movement.try_get_or_create_config_file().await?;

	let manager = Manager::<SuzukaPartialNode<Executor>>::new(config_file).await?;
	manager.try_run().await?;

	Ok(ExitCode::SUCCESS)
}