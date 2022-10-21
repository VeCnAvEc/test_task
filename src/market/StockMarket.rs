use std::fmt::Debug;
use crate::util::rounding::rounding_multiplication_f64;
use super::super::util::rounding::{rounding_dividing_f64};
#[macro_use] use super::super::either;

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
    OfPriceLess,
    DlPriceLess
}

pub trait StockMarketMethod {
    fn new() -> Self;

    fn push_order(&mut self, order: Order);

    fn get_spread(&self);

    fn process(&mut self);
}

impl Transaction {
    pub fn create_transaction(offer_id: &Order, deal_id: &Order) -> Transaction {
            let seller: String;
            let buyer: String;
            let amount: f64;
            let price: f64;

            if TypeOfOperation::Buy == offer_id.type_operation {
                buyer = offer_id.seller.clone();
                seller = deal_id.seller.clone();

                if offer_id.type_operation != deal_id.type_operation {
                    if offer_id.amount >= deal_id.amount {
                        amount = deal_id.amount;
                        price = deal_id.price
                    } else {
                        amount = offer_id.amount;
                        price = (offer_id.amount * deal_id.by_course as f64 * 1000.0).round() / 1000.0;
                    }
                } else {
                    if deal_id.price >= offer_id.amount {
                        amount = offer_id.amount;
                        price = offer_id.amount * (deal_id.amount / deal_id.price * 1000.0).round() / 1000.0;
                    } else {
                        amount = deal_id.price;
                        price = deal_id.amount;
                    }
                }
            } else {
                buyer = deal_id.seller.clone();
                seller = offer_id.seller.clone();

                if deal_id.type_operation != offer_id.type_operation {
                    if offer_id.amount >= deal_id.amount {
                        amount = deal_id.amount;
                        price = deal_id.price;
                    } else {
                        amount = offer_id.amount;
                        price = offer_id.amount * (deal_id.amount / deal_id.price * 1000.0).round() / 1000.0;
                    }
                } else {
                    if deal_id.price >= offer_id.amount {
                        amount = offer_id.amount;
                        price = (offer_id.amount / deal_id.by_course as f64 * 1000.0) / 1000.0;
                    } else {
                        amount = deal_id.amount;
                        price = (deal_id.amount / deal_id.by_course as f64 * 1000.0) / 1000.0;
                    }
                }
            }

        Transaction {
            seller,
            buyer,
            amount,
            price
        }
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

        println!("минимальная цена продажи usd: {}", (min_sale_price_usd * 1000.0).round() / 1000.0);
        println!("минимальная цена продажи euro: {}\n", (min_sale_price_euro * 1000.0).round() / 1000.0);

        println!("максимальна цена покупки usd: {}", (max_purchase_price_usd * 1000.0).round() / 1000.0);
        println!("максимальная цена покупки euro: {}\n", (max_purchase_price_euro * 1000.0).round() / 1000.0);

    }

    fn process(&mut self) {
        let mut id_good_deal = 0;
        let mut good_deals: Vec<Order> = Vec::new();
        let mut deal = &mut self.order;

        for offer in 11..deal.len() {
            if deal[offer].by_course == 0.0 {
                continue;
            }
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

                let mut index = 0;

                for zx in index..good_deals.len() {
                    id_good_deal = good_deal(&mut good_deals, id_good_deal, &deal, TypeOfOperation::Sell);
                    index +=1;

                    // for rm in 0..good_deals.len(){
                    //     if good_deals.len() == rm {
                    //         break;
                    //     }
                    //     if good_deals[rm].id == id_good_deal {
                    //         good_deals.remove(rm);
                    //     }
                    // }

                    remove_el_good_deals(&mut good_deals, id_good_deal);

                    if id_good_deal > 0 {
                        if id_good_deal - 1 != offer as u64 {
                            if deal[id_good_deal as usize -1].amount != 0.0 && deal[offer].amount != 0.0 {

                                if deal[id_good_deal as usize - 1].price < deal[offer].amount {
                                    let new_transaction = Transaction::create_transaction(&deal[offer as usize], &deal[id_good_deal as usize - 1]);

                                    deal[offer].amount = deal[offer].amount - deal[id_good_deal as usize - 1].price;
                                    deal[offer].price =  rounding_multiplication_f64(deal[offer].amount, deal[offer].by_course as f64);

                                    close_good_deal(&mut deal[id_good_deal as usize - 1]);
                                    self.transaction.push(new_transaction);
                                } else if deal[offer].price > deal[id_good_deal as usize - 1].amount {

                                    let new_transaction = Transaction::create_transaction(&deal[offer], &deal[id_good_deal as usize - 1]);

                                    deal[offer].amount = deal[offer].amount - deal[id_good_deal as usize - 1].price;
                                    deal[offer].price = (deal[offer].amount * deal[offer].price * 1000.0) / 1000.0;

                                    close_good_deal(&mut deal[id_good_deal as usize - 1]);
                                    self.transaction.push(new_transaction);
                                } else if deal[offer].price == deal[id_good_deal as usize - 1].amount {
                                    let new_transaction = Transaction::create_transaction(&deal[offer as usize], &deal[id_good_deal as usize - 1]);

                                    close_good_deal(&mut deal[id_good_deal as usize - 1]);
                                    close_offer(&mut deal[offer]);
                                    self.transaction.push(new_transaction);
                                } else if deal[id_good_deal as usize - 1].price > deal[offer].amount {
                                    let new_transaction = Transaction::create_transaction(&deal[offer as usize], &deal[id_good_deal as usize - 1]);

                                    deal[id_good_deal as usize - 1].amount = ((deal[id_good_deal as usize - 1].amount - deal[offer].price) * 1000.0).round() / 1000.0;
                                    deal[id_good_deal as usize - 1].price =  rounding_multiplication_f64(deal[id_good_deal as usize - 1].amount, deal[id_good_deal as usize - 1].by_course as f64);


                                    close_offer(&mut deal[offer]);
                                    self.transaction.push(new_transaction);
                                }
                            } else {
                                index -= 1;
                            }

                            id_good_deal = 0;
                        }
                    } else {
                        continue;
                    }
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

                let mut index = 0;

                for zx in index..good_deals.len() {
                    index += 1;
                    id_good_deal = good_deal(&mut good_deals, id_good_deal, &deal, TypeOfOperation::Buy);

                    remove_el_good_deals(&mut good_deals, id_good_deal);


                    // for rm in 0..good_deals.len(){
                    //     if good_deals.len() == rm {
                    //         break;
                    //     }
                    //     if good_deals[rm].id == id_good_deal {
                    //         good_deals.remove(rm);
                    //     }
                    // }

                    if deal[id_good_deal as usize -1].amount != 0.0 && deal[offer].amount != 0.0 {
                        if id_good_deal > 0 {
                            if id_good_deal - 1 != offer as u64 {
                                if deal[id_good_deal as usize - 1].price < deal[offer].amount {
                                        let new_transaction = Transaction::create_transaction(&deal[offer as usize], &deal[id_good_deal as usize - 1]);

                                        deal[offer].amount = deal[offer].amount - deal[id_good_deal as usize - 1].price;
                                        deal[offer].price =  rounding_multiplication_f64(deal[offer].amount, deal[offer].by_course as f64);

                                        deal[id_good_deal as usize - 1].amount = 0.0;

                                        close_good_deal(&mut deal[id_good_deal as usize - 1]);
                                        self.transaction.push(new_transaction);
                                } else if deal[offer].amount > deal[id_good_deal as usize - 1].price {
                                    let new_transaction = Transaction::create_transaction(&deal[offer as usize], &deal[id_good_deal as usize - 1]);

                                    deal[offer].amount = deal[offer].amount - deal[id_good_deal as usize - 1].price;
                                    deal[offer].price = deal[offer].amount * deal[offer].by_course as f64;

                                    deal[id_good_deal as usize - 1].price = 0.0;
                                    self.transaction.push(new_transaction);
                                } else if deal[offer].price < deal[id_good_deal as usize - 1].amount {
                                    let new_transaction = Transaction::create_transaction(&deal[offer as usize], &deal[id_good_deal as usize - 1]);

                                    deal[id_good_deal as usize - 1].amount = deal[id_good_deal as usize - 1].amount - deal[offer].price;
                                    deal[id_good_deal as usize - 1].price = deal[id_good_deal as usize - 1].amount * deal[id_good_deal as usize - 1].by_course as f64;

                                    close_offer(&mut deal[offer]);
                                    self.transaction.push(new_transaction);
                                }
                            }
                        } else {
                            println!("Выгодной сделки не были найдены но ваш ордер останется активным и будет закрыт когда найдётся выгодная сделка");
                            break;
                        }
                    } else {
                        index -= 1;
                    }
                }
            }

            good_deals = vec![];
        }
    }
}

fn min_sale_price(curr: &Order, currency: Currency, mut min_sale_price: f64) -> f64{


    // if currency == Currency::EURO {
    // }
    // if curr.currency == currency && curr.type_operation == TypeOfOperation::Sell {
    //     // let amount_per_price = curr.amount / EURO;
    //     let current_offer_rate = curr.amount / curr.price;
    //
    //     if current_offer_rate < min_sale_price && min_sale_price != 0.0 {
    //         min_sale_price = current_offer_rate;
    //     } else if min_sale_price == 0.0 {
    //         min_sale_price = current_offer_rate;
    //     }
    // }

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

fn good_deal(good_deals: &mut Vec<Order>, mut id_good_deal: u64, deal: &Vec<Order>, type_operation: TypeOfOperation) -> u64 {
    if type_operation == TypeOfOperation::Sell {
        let mut deals_len = good_deals.len();

        for ord in 0..deals_len {
            if id_good_deal == 0 {
                id_good_deal = good_deals[ord].id;
                continue;
            }
            if good_deals[ord].type_operation == TypeOfOperation::Sell {
                if id_good_deal == 0 {
                    id_good_deal = good_deals[ord].id;
                    continue;
                }
                if deal[id_good_deal as usize - 1].by_course >= good_deals[ord].by_course {
                    id_good_deal = good_deals[ord].id;

                    deals_len -= 1;
                }
            } else if good_deals[ord].type_operation == TypeOfOperation::Buy {
                if deal[id_good_deal as usize - 1].by_course >= rounding_dividing_f64(good_deals[ord].amount, good_deals[ord].price) as f32 {
                    id_good_deal = good_deals[ord].id;
                }
            }
        }
    } else if type_operation == TypeOfOperation::Buy {
         let mut deals_len = good_deals.len();
         for ord in 0..deals_len {
             if id_good_deal == 0 {
                 id_good_deal = good_deals[ord].id;
                 continue;
             }
             if good_deals[ord].type_operation == TypeOfOperation::Buy {
                 if deal[id_good_deal as usize - 1].by_course <= good_deals[ord].by_course {
                     id_good_deal = good_deals[ord].id;
                 }
             } else if good_deals[ord].type_operation == TypeOfOperation::Sell {
                 if deal[id_good_deal as usize - 1].by_course <= (good_deals[ord].amount as f32 / good_deals[ord].price as f32 * 1000.0) / 1000.0 {
                     id_good_deal = good_deals[ord].id;
                 }
             }
        }
    }

    id_good_deal
}



fn close_offer(deal: &mut Order) {
    deal.seller = "Closed".to_string();
    deal.price = 0.0;
    deal.amount = 0.0;
    deal.by_course = 0.0;
}

fn close_good_deal(deal: &mut Order) {
    deal.seller = "Closed".to_string();
    deal.price = 0.0;
    deal.amount = 0.0;
    deal.by_course = 0.0;
}

fn remove_el_good_deals (good_deals: &mut Vec<Order>, id_good_deal: u64) {
    for rm in 0..good_deals.len(){
        if good_deals.len() == rm {
            break;
        }
        if good_deals[rm].id == id_good_deal {
            good_deals.remove(rm);
        }
    }
}