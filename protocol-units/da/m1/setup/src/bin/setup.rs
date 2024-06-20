use m1_da_light_node_setup::setup;
use m1_da_light_node_util::config::M1DaLightNodeConfig;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
	use tracing_subscriber::EnvFilter;

	tracing_subscriber::fmt()
		.with_env_filter(
			EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
		)
		.init();

	let dot_movement = dot_movement::DotMovement::try_from_env()?;
	let config = dot_movement.try_get_config_from_json::<M1DaLightNodeConfig>()?;

	let config = setup(dot_movement.clone(), config).await?;

	dot_movement.try_write_config_to_json(&config)?;

	Ok(())
}
