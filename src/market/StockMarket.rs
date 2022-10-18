use std::fmt::Debug;
use std::process::id;

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

pub trait StockMarketMethod {
    fn new() -> Self;

    fn push_order(&mut self, order: Order);

    fn get_spread(&self);

    fn process(&mut self) -> Transaction;

    fn get_amount(&self, id: usize) -> &f64;

    fn get_price(&self, id: usize) -> &f64;

    // fn find_by_id(&self, id: usize) -> Order;
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

    fn process(&mut self) -> Transaction {
        let mut deal = &self.order;
        let mut good_deals: Vec<&Order> = Vec::new();
        let mut id_good_deal = 0;
        let mut deal_id = 0;

        for offer in 10..deal.len() {
            if deal[offer].type_operation == TypeOfOperation::Buy {
                for i in deal {
                    if i.type_operation == TypeOfOperation::Sell
                        && i.currency == deal[offer].currency
                        && deal[offer].by_course >= i.by_course
                    {
                        if deal[offer].id != i.id {
                            good_deals.push(i);
                        }
                    } else if i.type_operation == TypeOfOperation::Buy
                        && i.currency != deal[offer].currency
                        && deal[offer].by_course >= (i.amount as f32 / i.price as f32 * 1000.0).round() / 1000.0
                    {
                        if deal[offer].id != i.id {
                            good_deals.push(i);
                        }
                    }
                }

                for ord in &good_deals {
                    if id_good_deal == 0 {
                        id_good_deal = ord.id;
                        continue;
                    }
                    if ord.type_operation == TypeOfOperation::Sell {
                        if deal[id_good_deal as usize - 1].by_course >= ord.by_course {
                            id_good_deal = ord.id;
                        }
                    } else if ord.type_operation == TypeOfOperation::Buy {
                        if deal[id_good_deal as usize - 1].by_course >= (ord.amount as f32 / ord.price as f32 * 1000.0) / 1000.0 {
                            id_good_deal = ord.id;
                        }
                    }
                }

                println!("{:#?}", good_deals);
                println!("        {}", id_good_deal);

                // let best_offer = self.find_by_id(id_good_deal as usize);

                 // if deal[offer].amount < deal[id_good_deal as usize].amount {
                 // // good_deals[id_good_deal as usize].amount -= deal[offer].amount;
                 //    println!("Best Best Best Best Best587: {}", deal[id_good_deal as usize].amount);
                 // }
                // println!("{:?}", filter_good_deals);
            } else if deal[offer].type_operation == TypeOfOperation::Sell {
                    for i in deal {
                        if i.type_operation == TypeOfOperation::Buy
                            && i.currency == deal[offer].currency
                            && deal[offer].by_course <= i.by_course
                        {
                            if deal[offer].id != i.id {
                                good_deals.push(i);
                            }
                        } else if i.type_operation == TypeOfOperation::Sell
                            && i.currency != deal[offer].currency
                            && deal[offer].by_course <= (i.amount as f32 / i.price as f32 * 1000.0).round() / 1000.0
                        {
                            if deal[offer].id != i.id {
                                good_deals.push(i);
                            }
                        }
                    }

                    for ord in &good_deals {
                        if id_good_deal == 0 {
                            id_good_deal = ord.id;
                            continue;
                        }
                        if ord.type_operation == TypeOfOperation::Buy {
                            if deal[id_good_deal as usize - 1].by_course <= ord.by_course {
                                id_good_deal = ord.id;
                            }
                        } else if ord.type_operation == TypeOfOperation::Sell {
                            if deal[id_good_deal as usize - 1].by_course <= (ord.amount as f32 / ord.price as f32 * 1000.0) / 1000.0 {
                                id_good_deal = ord.id;
                        }
                    }
                }

                // deal[id_good_deal].

                // let mut best_offer = self.find_by_id(id_good_deal as usize);


                // if deal[offer].amount < deal[id_good_deal as usize].amount {
                //     *deal[id_good_deal as usize].amount -= *deal[offer].amount;
                // }

                println!("{:#?}", good_deals);
                println!("        {}", id_good_deal);
            }
        }

        good_deals = vec![];

        Transaction {
            seller: "".to_string(),
            buyer: "".to_string(),
            amount: 0.0,
            price: 0.0
        }
    }

    fn get_amount(&self, id: usize) -> &f64 {
        &self.order[id].amount
    }

    fn get_price(&self, id: usize) -> &f64 {
        &self.order[id].price
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

        // println!("order: {:?}", curr);
    }

    return max_purchase_price
}
