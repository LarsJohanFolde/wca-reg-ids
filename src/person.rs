#[derive(Debug)]
pub struct Person {
    pub id: u16,
    pub wca_id: String,
    pub name: String,
    pub country_id: String,
    pub is_competing: bool,
}

pub fn new(
    id_as_string: String,
    name: String,
    wca_id: String,
    country_id: String,
    is_competing: bool,
) -> Person {
    Person {
        id: id_as_string.parse().unwrap(),
        name,
        wca_id,
        country_id,
        is_competing,
    }
}

