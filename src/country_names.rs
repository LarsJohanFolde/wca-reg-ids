use rust_iso3166;

/// Returns the commonly used name of a country from a country ISO2 string.
/// This function also supports XK for Kosovo.
///
/// # Example
/// 
/// ```rust 
/// common_name("GB") -> "United Kingdom"
/// common_name("XK") -> "Kosovo"
/// ```
pub fn common_name(iso2_string: &str) -> String {
    match iso2_string {
        "GB" => "United Kingdom".to_string(),
        "RU" => "Russia".to_string(),
        "US" => "USA".to_string(),
        "KR" => "South Korea".to_string(),
        "KP" => "North Korea".to_string(),
        "XK" => "Kosovo".to_string(),
        _ => format!("{}", rust_iso3166::from_alpha2(iso2_string).unwrap().name)
    }
}
