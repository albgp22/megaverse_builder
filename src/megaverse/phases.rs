use std::error;
use std::time::Duration;

use super::api_client::builder::ApiClientBuilder;
use super::utils::geo;
use super::{astral::objects::AstralObject, config::handler::Config};

use log::info;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

pub fn phase1(cfg: &Config) -> Result<()> {
    info!("Running phase 1");

    /*
       Custom HTTP Client creation.
       Retries and increased timeout were added to address slow API responses, as some
       objects remained uncreated due to this timeout.
    */
    let api_client = ApiClientBuilder::new()
        .with_config(cfg)
        .with_retries(5)
        .with_timeout(Duration::from_secs(5))
        .build();

    // This phase was solved so this endpoint has changed. Hardcoding the result.
    //let (m,n): (u32,u32) = api_client.get_goal_dims()?;
    let (m, n): (u32, u32) = (11, 11);
    info!("Coordinates for given map are: ({m},{n})");

    let polyanets_to_create = geo::compute_cross_coordinates(m, n)?
        .map(|(x, y)| AstralObject::Polyanet { row: x, column: y });

    //
    for po in polyanets_to_create {
        if let Err(e) = api_client.create_object(&po) {
            log::error!("Error creating {po:?}: {e}");
        }
    }

    Ok(())
}

pub fn phase2(cfg: &Config) -> Result<()> {
    info!("Running phase 2");

    /*
       Custom HTTP Client creation.
       Retries and increased timeout were added to address slow API responses, as some
       objects remained uncreated due to this timeout.
    */
    let api_client = ApiClientBuilder::new()
        .with_config(cfg)
        .with_retries(5)
        .with_timeout(Duration::from_secs(5))
        .build();

    let reference_map = api_client.get_goal_resp();

    // Read the goal map and create corresponding objects.
    for (i, col) in reference_map?.goal.iter().enumerate() {
        for (j, description) in col.iter().enumerate() {
            let position_content =
                AstralObject::build_from_string(i as u32, j as u32, description.to_string());
            if let Some(obj) = position_content {
                if let Err(e) = api_client.create_object(&obj) {
                    log::error!("Error creating {obj:?}: {e}");
                }
            }
        }
    }

    Ok(())
}
