use serde_json::error::Error as ErrorDe;

use crate::models::websocket::{CombinedStreamEvent, WebSocketEventResponse};
pub fn deser_websocket(msg: &str) -> Result<CombinedStreamEvent<WebSocketEventResponse>, ErrorDe> {
    let string = msg.to_string();

    serde_json::from_str::<CombinedStreamEvent<WebSocketEventResponse>>(&string)

    // let resp = match serde_json::from_str::<CombinedStreamEvent<WebSocketEventResponse>>(&string) {
    //     Ok(val) => val,
    //     Err(err) => panic(err),
    // };

    // if let Ok(resp) = serde_json::from_str::<CombinedStreamEvent<WebSocketEventResponse>>(&string) {
    //     return resp;
    // } else {
    //     panic!()
    // }
    // if let Ok(response) = serde_json::from_str::<WebSocketEventResponse>(&string) {
    //     match response {
    //         WebSocketEventResponse::AggregateTrade(val) => {
    //             println!("got aggragate trade")
    //         }
    //         WebSocketEventResponse::Trade(_) => {
    //             println!("got trade")
    //         }
    //         WebSocketEventResponse::Kline(_) => {
    //             println!("got Kline")
    //         }
    //     }
    // }
}
