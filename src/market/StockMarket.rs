use std::fmt::Debug;
use crate::util::rounding::rounding_multiplication_f64;
use super::super::util::rounding::{rounding_dividing_f64};

pub const EURO: f64 = 1.02; // 1 евро стоит 1.3 центов
pub const USD: f64 = 0.97; // 1 доллар стоит 0.97 евро

#[derive(Debug)]
pub struct StockMarket {
    pub order: Vec<Order>,
    pub transaction: Vec<Transaction>
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Currency {
    USD,
    EURO,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum TypeOfOperation {
    Sell,
    Buy
}

pub enum Arithmetic {
    More,
    Less,
    Evenly
}

#[derive(Debug, Clone, PartialEq)]
pub struct Order {
    pub id: u64,
    pub type_operation: TypeOfOperation,
    pub amount: f64,
    pub price: f64,
    pub seller: String,
    pub currency: Currency,
    pub by_course: f32
}

#[derive(Debug)]
pub struct Transaction {
    pub seller: String,
    pub buyer: String,
    pub amount: f64,
    pub price: f64
}

#[derive(Debug)]
pub enum LogicalOperations {
    Greater,
    Less,
    Equal,
    OfPriceLess
}

macro_rules! either {
    ($test:expr => $true_expr:expr; $false_expr:expr) => {
        if $test {
            $true_expr
        }
        else {
            $false_expr
        }
    }
}

pub trait StockMarketMethod {
    fn new() -> Self;

    fn push_order(&mut self, order: Order);

    fn get_spread(&self);

    fn process(&mut self);

    fn get_amount(&self, id: usize) -> &f64;

    fn get_price(&self, id: usize) -> &f64;

    fn push_transaction(&mut self, transaction: Transaction);
}

impl Transaction {
    pub fn new(seller: String, buyer: String, amount: f64, price: f64) -> Transaction {
        Transaction {
            seller,
            buyer,
            amount,
            price
        }
    }
        // offer_id я deal_id продовец
    pub fn create_transaction(operation: LogicalOperations, offer_id: &Order, deal_id: &Order) -> Transaction {
        println!("Answer: {}", (deal_id.amount / deal_id.price) * deal_id.price);
        let transaction = match operation {
                LogicalOperations::Less => {
                    println!("Create transaction 1");
                    Transaction {
                        seller: either!(deal_id.type_operation == TypeOfOperation::Sell => offer_id.seller.clone(); deal_id.seller.clone()),
                        buyer: either!(deal_id.type_operation == TypeOfOperation::Buy => offer_id.seller.clone(); deal_id.seller.clone()),
                        amount: deal_id.price,
                        price: either!(deal_id.type_operation == TypeOfOperation::Sell => deal_id.by_course as f64 * offer_id.amount; (deal_id.amount / deal_id.price) * deal_id.price)
                    }
                },
                LogicalOperations::Equal => {
                    println!("Create transaction 2");

                    Transaction {
                        seller: either!(deal_id.type_operation == TypeOfOperation::Sell => offer_id.seller.clone(); deal_id.seller.clone()),
                        buyer: either!(deal_id.type_operation == TypeOfOperation::Buy => offer_id.seller.clone(); deal_id.seller.clone()),
                        amount: either!(deal_id.type_operation == TypeOfOperation::Sell => deal_id.by_course as f64 * offer_id.amount; (deal_id.amount / deal_id.price) * deal_id.price),
                        price: either!(deal_id.type_operation == TypeOfOperation::Sell => deal_id.by_course as f64 * offer_id.amount; (deal_id.amount / deal_id.price) * deal_id.price)
                    }
                },
                LogicalOperations::Greater => {
                    println!("Create transaction 3");

                    Transaction {
                        seller: either!(deal_id.type_operation == TypeOfOperation::Sell => offer_id.seller.clone(); deal_id.seller.clone()),
                        buyer: either!(deal_id.type_operation == TypeOfOperation::Buy => offer_id.seller.clone(); deal_id.seller.clone()),
                        amount: offer_id.price,
                        price: either!(deal_id.type_operation == TypeOfOperation::Sell => rounding_multiplication_f64(deal_id.by_course as f64, offer_id.amount); ((deal_id.amount as f64 / deal_id.price) * offer_id.amount * 1000.0).round() / 1000.0)
                    }
                },
                LogicalOperations::OfPriceLess => {
                    println!("create transaction 4");

                    Transaction {
                        seller: either!(deal_id.type_operation == TypeOfOperation::Sell => offer_id.seller.clone(); deal_id.seller.clone()),
                        buyer: either!(deal_id.type_operation == TypeOfOperation::Buy => offer_id.seller.clone(); deal_id.seller.clone()),
                        amount: offer_id.amount,
                        price: either!(deal_id.type_operation == TypeOfOperation::Sell => rounding_multiplication_f64(deal_id.by_course as f64, deal_id.amount); ((deal_id.amount as f64 / deal_id.price) * offer_id.amount * 1000.0).round() / 1000.0)
                    }
                }
            };

            transaction
    }
}

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

    fn get_spread(&self) -> () {
        let stock_market = self;

        let mut min_sale_price_usd = 0.0;
        let mut min_sale_price_euro = 0.0;

        let mut max_purchase_price_usd = 0.0;
        let mut max_purchase_price_euro = 0.0;

        for curr in stock_market.order.iter() {
            min_sale_price_usd = min_sale_price(curr, Currency::USD, min_sale_price_usd);
            min_sale_price_euro = min_sale_price(curr, Currency::EURO, min_sale_price_euro);

            max_purchase_price_usd = max_purchase_price(curr, Currency::USD, max_purchase_price_usd);
            max_purchase_price_euro = max_purchase_price(curr, Currency::EURO, max_purchase_price_euro);
        }

        println!("usd: {}", min_sale_price_usd);
        println!("euro: {}", min_sale_price_euro);

        println!("usd: {}", max_purchase_price_usd);
        println!("euro: {}", max_purchase_price_euro);

    }

    fn process(&mut self) {
        let mut id_good_deal = 0;
        let mut good_deals: Vec<Order> = Vec::new();
        let mut deal = &mut self.order;

        for offer in 11..deal.len() {
            if deal[offer].type_operation == TypeOfOperation::Buy {
                for i in 0..deal.len() {
                    if deal[i].type_operation == TypeOfOperation::Sell
                        && deal[i].currency == deal[offer].currency
                        && deal[offer].by_course >= deal[i].by_course
                    {
                        if deal[offer].id != deal[i].id {
                            good_deals.push(deal[i].clone());
                        }
                    } else if deal[i].type_operation == TypeOfOperation::Buy
                        && deal[i].currency != deal[offer].currency
                        && deal[offer].by_course >= rounding_dividing_f64(deal[i].amount, deal[i].price) as f32
                    {
                        if deal[offer].id != deal[i].id {
                            good_deals.push(deal[i].clone());
                        }
                    }
                }

                id_good_deal = good_deal(&good_deals, id_good_deal, &deal);
                println!("{}", id_good_deal);

                if id_good_deal != 0 {
                    if id_good_deal - 1 != offer as u64 {
                        // if deal[id_good_deal as usize - 1].type_operation == TypeOfOperation::Buy {
                            if deal[id_good_deal as usize - 1].price < deal[offer].amount {
                                println!("прайс меньше суммы");
                                let new_transaction = Transaction::create_transaction(LogicalOperations::Less, &deal[offer as usize], &deal[id_good_deal as usize - 1]);

                                deal[offer].amount = deal[offer].amount - deal[id_good_deal as usize - 1].price;
                                deal[offer].price =  rounding_multiplication_f64(deal[offer].amount, deal[offer].by_course as f64);

                                deal[id_good_deal as usize - 1].price = 0.0;
                                deal[id_good_deal as usize - 1].amount = 0.0;

                                self.transaction.push(new_transaction);
                            } else if deal[offer].price > deal[id_good_deal as usize - 1].amount {
                                println!("прайс больше суммы");
                                println!("2d");
                                let new_transaction = Transaction::create_transaction(LogicalOperations::Greater, &deal[offer], &deal[id_good_deal as usize - 1]);

                                deal[id_good_deal as usize - 1].amount = deal[offer].price - deal[id_good_deal as usize - 1].amount;
                                deal[id_good_deal as usize - 1].amount = 0.0;

                                deal[offer].amount = deal[offer].amount - deal[id_good_deal as usize - 1].price;

                                self.transaction.push(new_transaction);
                            } else if deal[offer].price == deal[id_good_deal as usize - 1].amount {
                                println!("прайс равен сумме");
                                println!("3d");
                                let new_transaction = Transaction::create_transaction(LogicalOperations::Equal,&deal[offer as usize], &deal[id_good_deal as usize - 1]);

                                deal[id_good_deal as usize - 1].amount = 0.0;
                                deal[offer].amount = 0.0;

                                self.transaction.push(new_transaction);
                            } else if deal[id_good_deal as usize - 1].price > deal[offer].amount{
                                let new_transaction = Transaction::create_transaction(LogicalOperations::OfPriceLess, &deal[offer as usize], &deal[id_good_deal as usize - 1]);

                                deal[id_good_deal as usize - 1].amount = ((deal[id_good_deal as usize - 1].amount - deal[offer].price) * 1000.0).round() / 1000.0;
                                deal[id_good_deal as usize - 1].price =  rounding_multiplication_f64(deal[id_good_deal as usize - 1].amount, deal[id_good_deal as usize - 1].by_course as f64);

                                deal[offer].price = 0.0;
                                deal[offer].amount = 0.0;
                                self.transaction.push(new_transaction);
                            }

                            if deal[id_good_deal as usize - 1].amount == 0.0 {
                                deal.remove((id_good_deal - 1) as usize);
                            } else if deal[offer].amount == 0.0 {
                                deal.remove(offer);
                            }

                            id_good_deal = 0;
                            continue;
                        // }
                    }
                } else {
                    println!("Выгодной сделки не были найдены но ваш ордер останется активным и будет закрыт когда найдётся выгодная сделка");
                }


            } else if deal[offer].type_operation == TypeOfOperation::Sell {
                for i in 0..deal.len() {
                    if deal[i].type_operation == TypeOfOperation::Buy
                        && deal[i].currency == deal[offer].currency
                        && deal[offer].by_course <= deal[i].by_course
                    {
                        if deal[offer].id != deal[i].id {
                            good_deals.push(deal[i].clone());
                        }
                    } else if deal[i].type_operation == TypeOfOperation::Sell
                        && deal[i].currency != deal[offer].currency
                        && deal[offer].by_course <= rounding_dividing_f64(deal[i].amount, deal[i].price) as f32
                    {
                        if deal[offer].id != deal[i].id {
                            good_deals.push(deal[i].clone());
                        }
                    }
                }

                id_good_deal = good_deal(&good_deals, id_good_deal, &deal);
            }

            if id_good_deal != 0 {
                if id_good_deal != offer as u64 {
                    if deal[id_good_deal as usize - 1].amount < deal[offer as usize].amount {
                        deal[offer as usize].amount -= deal[id_good_deal as usize - 1].amount;
                        deal[id_good_deal as usize - 1].amount = 0.0;
                    } else if deal[id_good_deal as usize - 1].amount > deal[offer as usize].amount {
                        deal[id_good_deal as usize - 1].amount -= deal[offer as usize].amount;
                        deal[offer as usize].amount = 0.0;
                    } else if deal[id_good_deal as usize - 1].amount == deal[offer as usize].amount{
                        deal[offer as usize].amount = 0.0;
                        deal[id_good_deal as usize - 1].amount -= 0.0;
                    }
                }
            } else {
                println!("Выгодной сделки не были найдены но ваш ордер останется активным и будет закрыт когда найдётся выгодная сделка");
            }

            println!("Transaction my wife{:?}", self.transaction);

            good_deals = vec![];
        }
    }

    fn push_transaction(&mut self, transaction: Transaction) {
        self.transaction.push(transaction);
    }

    fn get_amount(&self, id: usize) -> &f64 {
        &self.order[id].amount
    }

    fn get_price(&self, id: usize) -> &f64 {
        &self.order[id].price
    }
}

fn min_sale_price(curr: &Order, currency: Currency, mut min_sale_price: f64) -> f64{
    if currency == Currency::EURO {
        println!("curr: {:?}", curr);
    }
    if curr.currency == currency && curr.type_operation == TypeOfOperation::Sell {
        // let amount_per_price = curr.amount / EURO;
        let current_offer_rate = curr.amount / curr.price;

        if current_offer_rate < min_sale_price && min_sale_price != 0.0 {
            min_sale_price = current_offer_rate;
        } else if min_sale_price == 0.0 {
            min_sale_price = current_offer_rate;
        }
    }

    return min_sale_price
}


impl Order {
    pub fn new(id: u64, type_operation: TypeOfOperation, amount: f64, price: f64, seller: String, currency: Currency, by_course: f32) -> Order {
        Order {
            id,
            type_operation,
            amount,
            price,
            seller,
            currency,
            by_course
        }
    }
}

fn max_purchase_price(curr: &Order, currency: Currency, mut max_purchase_price: f64) -> f64{
    if curr.currency == currency && curr.type_operation == TypeOfOperation::Buy {
        let current_offer_rate = curr.amount / curr.price;

        if current_offer_rate > max_purchase_price {
            max_purchase_price = current_offer_rate;
        }
    }

    return max_purchase_price
}

fn good_deal(good_deals: &Vec<Order>, mut id_good_deal: u64, deal: &Vec<Order>) -> u64 {
    for ord in good_deals {
        if id_good_deal == 0 {
            id_good_deal = ord.id;
            continue;
        }
        if ord.type_operation == TypeOfOperation::Sell {
            if id_good_deal == 0 {
                id_good_deal = ord.id;
                continue;
            }
            if deal[id_good_deal as usize - 1].by_course >= ord.by_course {
                id_good_deal = ord.id;
            }
        } else if ord.type_operation == TypeOfOperation::Buy {
            if deal[id_good_deal as usize - 1].by_course >= rounding_dividing_f64(ord.amount, ord.price) as f32 {
                id_good_deal = ord.id;
            }
        }
    }

    id_good_deal
}







    // fn find_by_id(&self, id: usize) -> Order {
    //     let order = &self.order;
    //     let mut find_order = &Order::new(
    //         0, TypeOfOperation::Buy, 0.0, 0.0, "".to_string(), Currency::USD, 0.0
    //     );
    //
    //     for ord in order {
    //         if ord.id == id as u64 {
    //             find_order = ord
    //         }
    //     }
    //
    //     return find_order.clone()
    // }