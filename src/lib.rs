use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct OrderBook {
    pub bids: Vec<(f64, f64)>,
    pub asks: Vec<(f64, f64)>,
}

impl OrderBook {
    pub fn new() -> Self {
        Self {
            bids: Vec::new(),
            asks: Vec::new(),
        }
    }

    pub fn add_bid(&mut self, price: f64, amount: f64) {
        self.bids.push((price, amount));
    }

    pub fn add_ask(&mut self, price: f64, amount: f64) {
        self.asks.push((price, amount));
    }

    pub fn match_orders(&mut self) {
        let mut i = 0;
        let mut j = 0;
        while i < self.bids.len() && j < self.asks.len() {
            let bid = self.bids[i];
            let ask = self.asks[j];
            if bid.0 >= ask.0 {
                let price = ask.0;
                let amount = bid.1.min(ask.1);
                self.bids[i].1 -= amount;
                self.asks[j].1 -= amount;
                println!("Matched: {} @ {}", amount, price);
                if self.bids[i].1 == 0.0 {
                    i += 1;
                }
                if self.asks[j].1 == 0.0 {
                    j += 1;
                }
            } else {
                break;
            }
        }

        self.bids.retain(|bid| bid.1 > 0.0);
        self.asks.retain(|ask| ask.1 > 0.0);
    }
}

pub struct Exchange {
    pub orderbooks: HashMap<String, OrderBook>,
}

impl Exchange {
    pub fn new() -> Self {
        Self {
            orderbooks: HashMap::new(),
        }
    }

    pub fn add_order(&mut self, order: Order) {
        let orderbook = self
            .orderbooks
            .entry(order.symbol)
            .or_insert(OrderBook::new());
        match order.side {
            Side::Bid => orderbook.add_bid(order.price, order.amount),
            Side::Ask => orderbook.add_ask(order.price, order.amount),
        }
    }

    pub fn match_orders(&mut self, symbol: &str) {
        if let Some(orderbook) = self.orderbooks.get_mut(symbol) {
            orderbook.match_orders();
        }
    }

    pub fn match_all_orders(&mut self) {
        for (_, orderbook) in self.orderbooks.iter_mut() {
            orderbook.match_orders();
        }
    }
}

#[derive(Debug, Clone)]
pub enum Side {
    Bid,
    Ask,
}

#[derive(Debug, Clone)]
pub struct Order {
    pub symbol: String,
    pub side: Side,
    pub price: f64,
    pub amount: f64,
}
