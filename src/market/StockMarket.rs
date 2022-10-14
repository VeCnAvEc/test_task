use std::fmt::Debug;
use std::convert::From;

#[derive(Debug)]
pub struct StockMarket {
    pub order: Vec<Order>,
    pub transaction: Vec<Transaction>
}

#[derive(Debug, PartialEq)]
pub enum Currency {
    USD,
    EURO,
}

#[derive(Debug, PartialEq)]
pub enum TypeOfOperation {
    Sell,
    Buy
}

enum StringOrNumber {
    String(String),
    Number(u64)
}

#[derive(Debug)]
pub struct Order {
    pub type_operation: TypeOfOperation,
    pub amount: u64,
    pub price: u64,
    pub seller: String,
    pub currency: Currency
}

#[derive(Debug)]
pub struct Transaction {
    pub seller: String,
    pub buyer: String,
    pub amount: u64,
    pub price: u64
}

pub trait StockMarketMethod {
    fn new() -> Self;

    fn push_order(&mut self, order: Order);

    fn get_spread();

    fn process(transaction: Transaction) -> Transaction;
}
// <T: Debug + From<u64> + Default>
impl StockMarketMethod for StockMarket {
       fn new() -> Self {
            StockMarket {
                order: vec![],
                transaction: vec![]
            }
       }

    fn push_order(&mut self, order: Order) {
        &self.order.push(order);
    }

    fn get_spread() {
        todo!()
    }

    fn process(transaction: Transaction) -> Transaction {
        Transaction {
            seller: "".to_string(),
            buyer: "".to_string(),
            amount: 0,
            price: 0
        }
    }
}
