use reqwest;
use serde_json::{self};
use std::env;
mod country_names;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    //  TODO: Sanitize input
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        list_competitors(args[1].to_string()).await?;
        std::process::exit(0);
    }

    println!("reg: Unexpected number of arguments. Expected 1 but got {}", args.len()-1);
    Ok(())
}


async fn list_competitors(competition_id: String) -> Result<(), Box<dyn std::error::Error>> {
    let formatted_url: String = format!("https://worldcubeassociation.org/api/v0/competitions/{}/wcif/public", competition_id);
    let url: &str = &String::from(formatted_url);
    println!("Requesting data...");
    let resp = reqwest::get(url).await?;

    if resp.status().is_success() {
        let data: serde_json::Value = resp.json().await?;
        if let Some(competition_name) = data["name"].as_str() {
            let competition_name = competition_name;
            println!("\n----------------------------------\n");
            println!("Showing results for {}:\n", competition_name);
        }
        if let Some(persons) = data["persons"].as_array() {
            let mut null_count: u32 = 0;
            for person in persons {

                // Remove people who are not registered.
                if format!("{}", person["registrantId"]) == "null" {
                    null_count += 1;
                    continue;
                }

                // Print competitors to stdout.
                if let Some(name) = person["name"].as_str() {
                    if let Some(country_iso2) = person["countryIso2"].as_str() {
                        println!("{}: {}, {}",
                                 person["registrantId"],
                                 name,
                                 country_names::common_name(country_iso2));
                    }
                }
            }
            // Count registered competitors by taking the amount of people minus the amount of
            // people not registered.
            println!("\nTotal registrations: {}/{}", persons.len() as u32 - null_count, data["competitorLimit"]);
        } else {
            println!("'persons' field is not in the expected format or is missing.");
        }
    } else {
        println!("Failed to fetch data: {}", resp.status());
    } 
    Ok(())
}
