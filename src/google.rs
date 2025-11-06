use serde::Deserialize;

use crate::cli::Query;

const GOOGLE_API_URL: &str = "https://www.googleapis.com/customsearch/v1";

#[derive(Debug, thiserror::Error)]
pub enum GoogleError {
    #[error("Could not query the Google API: {0}")]
    RequestFailed(#[from] reqwest::Error),

    #[error("Google API returned an error: {0}")]
    ApiError(String),
}

pub fn query(query: Query) -> Result<GoogleSearchResponse, GoogleError> {
    let url = build_request_url(&query);

    let request = reqwest::blocking::get(url).map_err(|e| e.without_url())?;

    if !request.status().is_success() {
        return Err(GoogleError::ApiError(request.text()?));
    }

    let response: GoogleSearchResponse = request.json().map_err(|e| e.without_url())?;

    Ok(response)
}

fn build_request_url(query: &Query) -> String {
    let encoded_query = urlencoding::encode(&query.query);
    let safe = if query.safe { "active" } else { "off" };

    format!(
        "{GOOGLE_API_URL}?key={}&cx={}&q={}&safe={}",
        query.api_key, query.cx, encoded_query, safe
    )
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GoogleSearchResponse {
    pub search_information: GoogleSearchInformation,
    pub items: Vec<GoogleSearchResult>,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GoogleSearchInformation {
    pub total_results: String,
    pub formatted_total_results: String,
    pub search_time: f64,
    pub formatted_search_time: String,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GoogleSearchResult {
    pub title: String,
    pub html_title: String,
    pub link: String,
    pub display_link: String,
    pub snippet: String,
    pub formatted_url: String,
}
