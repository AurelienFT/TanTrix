use tiny_http::Request;
use crate::piece::PieceExport;

pub fn handle_submit(request: &mut Request) -> String {
    let mut content = String::new();
    request.as_reader().read_to_string(&mut content).unwrap();
    println!("body = {:#?}", content);
    serde_json::from_str::<Vec<PieceExport>>(&content).unwrap();
    String::from("ok")
}

pub fn handle_unknown(_request: &mut Request) -> String {
    String::from("bad request")
}