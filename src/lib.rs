use lambda_flows::{request_received, send_response};

#[no_mangle]
pub fn run() {
    if let Some((_qry, _body)) = request_received() {
        send_response(
            200,
            vec![(String::from("content-type"), String::from("text/html"))],
            "ok".as_bytes().to_vec(),
        );
    }
}
