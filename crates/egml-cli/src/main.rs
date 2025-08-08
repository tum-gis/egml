mod cli;

use anyhow::Result;
use egml::model::base::{Gml, Id};
use egml::model::geometry::{DirectPosition, LinearRing};
use tracing::info;

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    info!("Hello, world!");

    let gml = Gml::new(Id::try_from("test_id")?);
    let point_a = DirectPosition::new(0.0, 0.0, 0.0)?;
    let point_b = DirectPosition::new(1.0, 1.0, 1.0)?;
    let point_c = DirectPosition::new(0.0, 0.0, 2.0)?;
    LinearRing::new(gml, vec![point_a, point_b, point_c])?;

    Ok(())
}
