use reqwest;
use serde_json::{self};
use std::env;
mod country_names;

// TODO add information flag that prints competitioninfo such as whether reg is still open...

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    //  TODO: Sanitize input
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        list_competitors(args[1].to_string()).await?;
        std::process::exit(1);
    }

    println!("reg: Unexpected number of arguments. Expected 1 but got {}", args.len()-1);
    Ok(())
}

#[derive(Debug)]
struct Person {
    id: u16,
    name: String,
    country_id: String,
    is_competing: bool,
}

fn create_person(id_as_string: String, name: String, country_id: String, is_competing: bool) -> Person {
    Person {
        id: id_as_string.parse().unwrap(),
        name,
        country_id,
        is_competing
    }
}

async fn list_competitors(competition_id: String) -> Result<(), Box<dyn std::error::Error>> {
    let formatted_url: String = format!("https://worldcubeassociation.org/api/v0/competitions/{}/wcif/public", competition_id);
    let url: String = formatted_url;
    println!("Requesting data...");
    let resp = reqwest::get(url).await?;

    if resp.status().is_success() {
        let data: serde_json::Value = resp.json().await?;
        let competition_name = &data["name"];
        println!("\n----------------------------------\n");
        println!("Showing results for {}:\n", competition_name);
        // Create and print persons
        if let Some(persons) = data["persons"].as_array() {
            let mut non_competing_persons: u16 = 0;
            for person in persons {

                let registration = &person["registration"];
                let is_competing: bool = registration["isCompeting"].as_bool().unwrap();

                let person = create_person(
                    person["registrantId"].to_string(),
                    person["name"].to_string().replace("\"", ""),
                    country_names::common_name(&person["countryIso2"].as_str().unwrap()).to_string(),
                    is_competing
                );

                // Remove non-competing organizers and delegates
                if person.is_competing == false {
                    non_competing_persons += 1;
                    continue;
                }

                println!("{}: {}, {}", person.id, person.name, person.country_id);
            }

            // Count registered competitors by taking the amount of people minus the amount of
            // people not registered.
            let competitor_count: u16 = persons.len() as u16 - non_competing_persons;

            // Print a message if there are no competitors
            if competitor_count == 0 {
                println!("No registrations for {}", competition_name);
            }

            println!("\nTotal registrations: {}/{}", competitor_count, data["competitorLimit"]);
        } else {
            println!("'persons' field is not in the expected format or is missing.");
        }
    } else {
        println!("Failed to fetch data: {}", resp.status());
    } 
    Ok(())
}
