use std::{sync::{Arc, RwLock}, str::FromStr};
use tiny_http::{Server, Response, Header, Method};

use crate::{
    game::check_solved_game,
    piece::{Color, Line, Piece},
    api::{handle_submit, handle_unknown},
};

mod api;
mod game;
mod piece;

fn main() {
    let server = Server::http("0.0.0.0:8000").unwrap();

    for mut request in server.incoming_requests() {
        let response_str = if request.method() == &Method::Post {
            match request.url() {
                "/submit" => handle_submit(&mut request),
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
