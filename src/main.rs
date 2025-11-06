use std::{env, sync::Arc, thread};

use clap::Parser;

use crate::{
    cli::Query,
    state::{QueryState, State},
};

mod cli;
mod google;
mod state;
mod tui;

fn main() -> std::io::Result<()> {
    load_env();

    let query = Query::parse();

    let state = Arc::new(State::new(query));

    let thread_state = state.clone();

    thread::spawn(move || {
        let result = google::query(thread_state.query.clone());

        let mut results = thread_state.results.write().unwrap();
        *results = QueryState::Finished(result);
    });

    tui::run(state)
}

fn load_env() {
    if let Some(mut path) = env::home_dir() {
        path.push(".config/nyekis-search/.env");

        dotenvy::from_path(path).ok();
    }

    dotenvy::dotenv().ok();
}
