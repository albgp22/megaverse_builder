use std::error;
use std::time::Duration;

use super::api_client::builder::ApiClientBuilder;
use super::utils::geo;
use super::{astral::objects::AstralObject, config::handler::Config};

use log::{debug, info};

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

pub fn phase1(cfg: &Config) -> Result<()> {
    info!("Running problem phase 1");

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

    // In the case that a polyanet fails to be created, we log the error but we won't
    // stop the execution.
    for p in polyanets_to_create {
        if let Err(e) = api_client.create_object(&p) {
            log::error!("Error creating {p:?}: {e}");
        }
    }

    Ok(())
}

pub fn phase2(cfg: &Config) -> Result<()> {
    info!("Running problem phase 2");

    let api_client = ApiClientBuilder::new()
        .with_config(cfg)
        .with_retries(5)
        .with_timeout(Duration::from_secs(5))
        .build();

    debug!("Reading reference map from the API");
    let reference_map: crate::megaverse::api_client::types::GoalResponse =
        api_client.get_goal_resp()?;
    info!("Reference map successfully read from server. Creating objects...");

    // Read the goal map and create corresponding objects.
    for (i, col) in reference_map.goal.iter().enumerate() {
        for (j, description) in col.iter().enumerate() {
            let position_content =
                AstralObject::build_from_string(i as u32, j as u32, description.to_string());
            if let Some(obj) = position_content {
                // Avoid creating empty space in the megaverse
                if let Err(e) = api_client.create_object(&obj) {
                    log::error!("Error creating {obj:?}: {e}");
                }
            }
        }
    }

    Ok(())
}
