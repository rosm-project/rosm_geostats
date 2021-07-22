use rosm_geostats::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let census_json = include_str!("census.json");

    let tilestats: Tilestats = serde_json::from_str(census_json)?;

    println!("{:#?}", tilestats);

    Ok(())
}
