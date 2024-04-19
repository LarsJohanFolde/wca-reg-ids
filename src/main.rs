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

#[derive(Debug)]
struct Person {
    id: u16,
    name: String,
    country_id: String,
}

fn create_person(id_as_string: String, name: String, country_id: String) -> Person {
    Person {
        id: id_as_string.parse().unwrap(),
        name,
        country_id
    }
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

                let id_as_string = person["registrantId"].to_string();

                if format!("{id_as_string}") == "null" {
                    null_count += 1;
                    continue;
                }

                let person = create_person(
                    person["registrantId"].to_string(),
                    person["name"].to_string().replace("\"", ""),
                    country_names::common_name(&person["countryIso2"].as_str().unwrap()).to_string()
                );

                println!("{}: {}, {}", person.id, person.name, person.country_id);
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
