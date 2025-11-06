use std::{sync::Arc, thread};

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
    dotenvy::dotenv().ok();

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
