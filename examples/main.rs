use minicross::*;

fn main() {
    let mut exchange = Exchange::new();

    exchange.add_order(Order {
        symbol: "BTC/USD".to_string(),
        side: Side::Bid,
        price: 100.0,
        amount: 1.0,
    });

    exchange.add_order(Order {
        symbol: "BTC/USD".to_string(),
        side: Side::Ask,
        price: 100.0,
        amount: 1.0,
    });

    exchange.match_all_orders();
}
