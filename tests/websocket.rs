// File: tests/integration_tests.rs

// Import necessary modules
// You can use the `#[cfg(test)]` attribute to compile this code only during tests.
#[cfg(test)]
mod tests {

    use rust_websockets::models::websocket::{CombinedStreamEvent, WebSocketEventResponse};

    #[test]
    fn test_deserialise() {
        // Arrange: Set up the test scenario.
        use ::rust_websockets::controllers::websocket::deser_websocket;
        println!("-----------starting test");
        //         let resp_text ="{\"stream\":\"btcusdt@aggTrade\",\"data\":{\"e\":\"aggTrade\",\"E\":1711350754941,\"s\":\"BTCUSDT\",\"a\":2937327945,\"p\":\"67179.
        // 55000000\",\"q\":\"0.00015000\",\"f\":3511658295,\"l\":3511658295,\"T\":1711350754940,\"m\":false,\"M\":true}}".to_string();
        let resp_text = r#"{"stream":"btcusdt@aggTrade","data":{"e":"aggTrade","E":1712058977883,"s":"BTCUSDT","a":2949452387,"p":"65420.01000000","q":"0.00010000","f":3527264994,"l":3527264994,"T":1712058977883,"m":false,"M":true}}"#;

        let deserialized_response = deser_websocket(&resp_text);
        println!("{:?}", deserialized_response);

        let text_object_response: CombinedStreamEvent<WebSocketEventResponse> =
            CombinedStreamEvent {
                stream: "btcusdt@aggTrade".to_string(),
                data: WebSocketEventResponse::AggregateTrade(
                    rust_websockets::models::websocket::AggregateTradePayload {
                        event_time: 1712058977883u64,
                        symbol: "BTCUSDT".to_string(),
                        aggregate_trade_id: 2949452387u64,
                        price: "65420.01000000".to_string(),
                        quantity: "0.00010000".to_string(),
                        first_trade_id: 3527264994u64,
                        last_trade_id: 3527264994u64,
                        trade_time: 1712058977883u64,
                        ignore: false,
                        is_buyer_market_maker: true,
                    },
                ),
            };

        // assert_eq!(text_object_response.stream, deserialized_response.clone().unwrap().stream);
        match text_object_response.data {
            WebSocketEventResponse::AggregateTrade(val) => {
                match deserialized_response.unwrap().data {
                    WebSocketEventResponse::AggregateTrade(deser_val) => {
                        assert_eq!(val.first_trade_id, deser_val.first_trade_id)
                    }
                    WebSocketEventResponse::Trade(_) => {}
                    WebSocketEventResponse::Kline(_) => {}
                }
            }
            WebSocketEventResponse::Trade(_) => {}
            WebSocketEventResponse::Kline(_) => {}
        }
    }
}
