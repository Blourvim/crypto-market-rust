use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AggregateTradePayload {
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "a")]
    pub aggregate_trade_id: u64,
    #[serde(rename = "p")]
    pub price: String,
    #[serde(rename = "q")]
    pub quantity: String,
    #[serde(rename = "f")]
    pub first_trade_id: u64,
    #[serde(rename = "l")]
    pub last_trade_id: u64,
    #[serde(rename = "T")]
    pub trade_time: u64,
    #[serde(rename = "m")]
    pub is_buyer_market_maker: bool,
    #[serde(rename = "M")]
    pub ignore: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TradePayload {
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "t")]
    pub trade_id: u64,
    #[serde(rename = "p")]
    pub price: String,
    #[serde(rename = "q")]
    pub quantity: String,
    #[serde(rename = "b")]
    pub buyer_order_id: u64,
    #[serde(rename = "a")]
    pub seller_order_id: u64,
    #[serde(rename = "T")]
    pub trade_time: u64,
    #[serde(rename = "m")]
    pub is_buyer_market_maker: bool,
    #[serde(rename = "M")]
    pub ignore: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KlinePayload {
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "k")]
    pub kline: KlineData,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KlineData {
    #[serde(rename = "t")]
    pub start_time: u64,
    #[serde(rename = "T")]
    pub close_time: u64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "i")]
    pub interval: String,
    #[serde(rename = "f")]
    pub first_trade_id: u64,
    #[serde(rename = "L")]
    pub last_trade_id: u64,
    #[serde(rename = "o")]
    pub open_price: String,
    #[serde(rename = "c")]
    pub close_price: String,
    #[serde(rename = "h")]
    pub high_price: String,
    #[serde(rename = "l")]
    pub low_price: String,
    #[serde(rename = "v")]
    pub base_asset_volume: String,
    #[serde(rename = "n")]
    pub number_of_trades: u64,
    #[serde(rename = "x")]
    pub is_kline_closed: bool,
    #[serde(rename = "q")]
    pub quote_asset_volume: String,
    #[serde(rename = "V")]
    pub taker_buy_base_asset_volume: String,
    #[serde(rename = "Q")]
    pub taker_buy_quote_asset_volume: String,
    #[serde(rename = "B")]
    pub ignore: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CombinedStreamEvent<T> {
    pub stream: String,
    pub data: T,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "e")]
pub enum WebSocketEventResponse {
    #[serde(rename = "aggTrade")]
    AggregateTrade(AggregateTradePayload),
    #[serde(rename = "trade")]
    Trade(TradePayload),
    #[serde(rename = "kline")]
    Kline(KlinePayload),
}
