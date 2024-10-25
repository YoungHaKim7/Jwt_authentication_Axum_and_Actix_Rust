use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};

use crate::{handler::create_note_handler, AppState};
