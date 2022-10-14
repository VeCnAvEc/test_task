mod market;

use market::StockMarket::{Transaction, Order, TypeOfOperation, StockMarket};

use std::io;
use std::io::{stdin};
use std::num::ParseIntError;
use std::convert::From;
use std::fmt::Debug;
use crate::market::StockMarket::{Currency, StockMarketMethod};

struct User {
    amount: u64,
    price: u64,
    seller: String,
    currency: Currency
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

fn main() {
    let mut operation = String::from("");
    let mut stock_market = StockMarket::new();

    loop {
        println!("Какую операцию вы хотите провести?\n\
            Введите 1 что-бы добавить ордер\n\
            Введите 2 что-бы просмотреть предложение\n\
            Введите 3 что-бы получить разница между минимальной ценой продажи и максимальной ценой покупки
        ");
        operation.clear();
        stdin().read_line(&mut operation).unwrap();
        match operation.trim().parse::<u64>() {
            Ok(1) => {
                let mut type_of_operation = String::from("");
                sell_or_buy(&mut type_of_operation, &mut stock_market);
            },
            Ok(2) => {
                let mut type_of_operation = String::from("");
                get_offer(&stock_market.order);
            },
            // Ok(3) => {
            //     println!("200");
            // },
            Err(e) => {
                println!("Введены не коррктные символы")
            },
            _ => {
                println!("Введите число в диопазоне 1-3!")
            }
        }
    }


}

fn trading_currency(currency: &mut String, type_operation: &TypeOfOperation) -> Currency {
    println!("{:?}", type_operation);
    'valid_type: loop {
        if *type_operation == TypeOfOperation::Buy {
            println!("Что вы хотите купить?\nВведите 1 что-бы купить USD\nВведите 2 что-бы купить EURO");
        } else if *type_operation == TypeOfOperation::Sell {
            println!("Что вы хотите продать?\nВведите 1 что-бы продать USD\nВведите 2 что-бы продать EURO");
        }

         currency.clear();
         stdin().read_line(currency).unwrap();

         match currency.trim().parse::<u64>() {
            Ok(1) => {
                return Currency::USD;
            },
            Ok(2) => {
                return Currency::EURO;
            },
            Err(e) => {
                 println!("Вы ввели не коррктное символы");
                 continue
            },
            _ => {
                 println!("выберите в диапозоне 1-2");
                 continue
            }
        };
    }
}

fn sell_or_buy(type_of_operation: &mut String, stock_market: &mut StockMarket) {
    'validation_sell_buy: loop {
        println!("Введите 1 что-бы добавить ордер на покупку\nВведите 2 что-бы добавить ордер на продажу");
        type_of_operation.clear();
        stdin().read_line(type_of_operation).unwrap();

        match type_of_operation.trim().parse::<u64>() {
            Ok(1) => {
                let user = date_or_order(TypeOfOperation::Buy);

                StockMarket::push_order(stock_market, Order {
                    type_operation: TypeOfOperation::Buy,
                    amount: user.amount,
                    price: user.price,
                    seller: user.seller,
                    currency: user.currency,
                });

                println!("{:#?}", stock_market);
                break;
            },

            Ok(2) => {
                let user = date_or_order(TypeOfOperation::Sell);

                StockMarket::push_order(stock_market, Order {
                    type_operation: TypeOfOperation::Sell,
                    amount: user.amount,
                    price: user.price,
                    seller: user.seller,
                    currency: user.currency,
                });

                println!("{:#?}", stock_market);
                break
            },
            Err(e) => {
                 println!("Введены не коррктные символы");
                 continue
            },
            _ => {
                println!("Введите число в диопазоне 1-2!");
                continue
            }
       }
    }
}

fn date_or_order(type_operation: TypeOfOperation) -> User {
    let mut seller = String::from("");
    let mut currency_type = String::from("");
    let mut amount = String::from("");
    let mut price = String::from("");

    println!("ddddd: {:?}", type_operation);
    let currency_sell = trading_currency(&mut currency_type, &type_operation);

    if type_operation == TypeOfOperation::Buy {
        println!("Введите ваше имя");
        stdin().read_line(&mut seller).unwrap();

        println!("Сколько {:?} вы хотите купить?", currency_sell);
        stdin().read_line(&mut amount).unwrap();

        println!("Назавите свою цену, она будет в {:?}", either!(currency_sell == Currency::USD => Currency::EURO; Currency::USD));
        stdin().read_line(&mut price).unwrap();
    } else {
        println!("Введите ваше имя");
        stdin().read_line(&mut seller).unwrap();

        println!("Сколько {:?} вы хотите продать?", currency_sell);
        stdin().read_line(&mut amount).unwrap();

        println!("За сколько вы хотите продать {:?}?", either!(currency_sell == Currency::USD => Currency::EURO; Currency::USD));
        stdin().read_line(&mut price).unwrap();
    };

    let amount_to_u64 = amount.trim().parse::<u64>().unwrap();
    let price_to_u64 = price.trim().parse::<u64>().unwrap();

    User {
        amount: amount_to_u64,
        price: price_to_u64,
        seller,
        currency: currency_sell
    }
}

fn get_offer(stock_market: &Vec<Order>) {
    loop {
        let mut offer = String::from("");
        println!("1 просмотреть все предложение\n2 просмотреть что доступно в продаже\n3 просмотреть что доступно в покупке");
        stdin().read_line(&mut offer).unwrap();

        match offer.trim().parse::<u64>() {
            Ok(1) => {
                if stock_market.len() == 0 {
                    println!("Не чего не найдено")
                }
                println!("{:#?}", stock_market);
            },
            Ok(2) => {
                let filter_by_type_operation: Vec<&Order> = stock_market.iter().map(|of| of).filter(|el| el.type_operation == TypeOfOperation::Sell).collect();
                println!("{:?}", filter_by_type_operation);
            },
            Ok(3) => {
                let filter_by_type_operation: Vec<&Order> = stock_market.iter().map(|of| of).filter(|el| el.type_operation == TypeOfOperation::Buy).collect();
                println!("{:?}", filter_by_type_operation);
            },
            _ => {
                println!("Введите цифру в диапазоне 1-3");
                continue;
            },
            Err(e) => {
                println!("Введите корректные символы");
                continue;
            }
        }
    }
}

fn print_type_of<T>(_: &T) {
    println!("My type::::: {}", std::any::type_name::<T>())
}


























// fn convert_string_u64(amount: &Currency, mut quantity: String) -> u64 {
//     let res: u64 = 'validation_quantity: loop {
//         println!("Сколько {:?} вы хотите купить?", amount);
//         stdin().read_line(&mut quantity).unwrap();
//
//         match quantity.trim().parse::<u64>().unwrap() {
//             u64 => {
//                 break u64;
//             },
//             _ => {
//                 println!("Введите правильное число");
//                 continue;
//             },
//             Err(e) => {
//                 continue;
//             }
//         }
//     };
//
//     return res
// }