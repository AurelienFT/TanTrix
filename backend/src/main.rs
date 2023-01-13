use api::handle_get_daily_game;
use std::{
    str::FromStr,
    sync::{Arc, RwLock},
};
use tiny_http::{Header, Method, Response, Server};

use crate::{
    api::{handle_submit, handle_unknown},
    game::check_solved_game,
    piece::{Color, Line, Piece},
};

mod api;
mod game;
mod generation;
mod piece;

fn main() {
    let server = Server::http("0.0.0.0:8000").unwrap();

    for mut request in server.incoming_requests() {
        let response_str = if request.method() == &Method::Post {
            match request.url() {
                "/submit" => handle_submit(&mut request),
                //TODO: Fix use the full structure
                "/getDailyGame" => handle_get_daily_game(&mut request),
                _ => handle_unknown(&mut request),
            }
        } else {
            String::from("")
        };
        let mut response = Response::from_string(response_str);
        response.add_header(Header::from_str("Access-Control-Allow-Origin:*").unwrap());
        response.add_header(Header::from_str("Access-Control-Allow-Headers:*").unwrap());
        request.respond(response).unwrap();
    }
}
