use std::sync::RwLock;

use ratatui::widgets::ListState;

use crate::{
    cli::Query,
    google::{GoogleError, GoogleSearchResponse},
};

pub struct State {
    pub query: Query,
    pub results: RwLock<QueryState>,
    pub list: RwLock<ListState>,
}

impl State {
    pub fn new(query: Query) -> Self {
        Self {
            query,
            results: RwLock::new(QueryState::Searching),
            list: RwLock::new(ListState::default().with_selected(Some(0))),
        }
    }
}

pub enum QueryState {
    Searching,
    Finished(Result<GoogleSearchResponse, GoogleError>),
}

impl QueryState {
    pub fn unwrap_results(&self) -> &GoogleSearchResponse {
        match self {
            QueryState::Finished(Ok(results)) => results,
            QueryState::Finished(Err(error)) => panic!("Tried to unwrap error: {error:?}"),
            QueryState::Searching => panic!("Tried to unwrap results while still searching"),
        }
    }
}
